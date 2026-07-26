pub use lvm_common::test_metadata::{TestCategory, TestMetadata};
use lvm_common::test_metadata::{
    parse_test_metadata as parse_shared_test_metadata, validate_feature_id,
};
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct ValidationErrors {
    messages: Vec<String>,
}

impl ValidationErrors {
    pub fn messages(&self) -> &[String] {
        &self.messages
    }
}

impl Display for ValidationErrors {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        for message in &self.messages {
            writeln!(formatter, "{message}")?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationErrors {}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub spec: Option<String>,
    pub criteria: Vec<String>,
    pub limitations: Option<Vec<String>>,
    pub blocked_by: Option<Vec<String>>,
    pub reason: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Missing,
    Partial,
    Implemented,
    Blocked,
    Deferred,
}

impl Display for Status {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Missing => "Missing",
            Self::Partial => "Partial",
            Self::Implemented => "Implemented",
            Self::Blocked => "Blocked",
            Self::Deferred => "Deferred",
        })
    }
}

#[derive(Debug)]
pub struct FeatureRegistry {
    features: BTreeMap<String, Feature>,
}

impl FeatureRegistry {
    pub fn len(&self) -> usize {
        self.features.len()
    }

    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }

    pub fn contains(&self, id: &str) -> bool {
        self.features.contains_key(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Feature)> {
        self.features
            .iter()
            .map(|(id, feature)| (id.as_str(), feature))
    }
}

#[derive(Debug)]
pub struct TrackedFixture {
    pub source_path: PathBuf,
    pub snapshot_path: PathBuf,
    pub metadata: TestMetadata,
}

#[derive(Debug)]
pub struct MigrationInventory {
    pub source_count: usize,
    pub entry_count: usize,
    pub tracked_entries: Vec<PathBuf>,
    pub untracked_entries: Vec<PathBuf>,
    pub invalid_entries: Vec<String>,
    pub missing_snapshots: Vec<PathBuf>,
    pub orphan_snapshots: Vec<PathBuf>,
    pub pending_snapshots: Vec<PathBuf>,
    pub identity_collisions: Vec<String>,
    pub entries_by_language: BTreeMap<String, usize>,
    pub tracked_by_category: BTreeMap<String, usize>,
}

impl MigrationInventory {
    pub fn is_clean(&self) -> bool {
        self.untracked_entries.is_empty()
            && self.invalid_entries.is_empty()
            && self.missing_snapshots.is_empty()
            && self.orphan_snapshots.is_empty()
            && self.pending_snapshots.is_empty()
            && self.identity_collisions.is_empty()
    }
}

pub fn build_migration_inventory(
    fixtures_root: &Path,
    snapshots_root: &Path,
    registry: &FeatureRegistry,
) -> Result<MigrationInventory, ValidationErrors> {
    if !fixtures_root.is_dir() {
        return Err(errors([format!(
            "fixture root does not exist: {}",
            fixtures_root.display()
        )]));
    }
    if !snapshots_root.is_dir() {
        return Err(errors([format!(
            "snapshot root does not exist: {}",
            snapshots_root.display()
        )]));
    }

    let mut sources = Vec::new();
    let mut walk_errors = Vec::new();
    for entry in WalkDir::new(fixtures_root) {
        match entry {
            Ok(entry)
                if entry.file_type().is_file()
                    && matches!(
                        entry.path().extension().and_then(|value| value.to_str()),
                        Some("java" | "rns")
                    ) =>
            {
                sources.push(entry.into_path());
            }
            Ok(_) => {}
            Err(error) => walk_errors.push(format!(
                "failed to walk {}: {error}",
                fixtures_root.display()
            )),
        }
    }
    if !walk_errors.is_empty() {
        return Err(ValidationErrors {
            messages: walk_errors,
        });
    }
    sources.sort();

    let mut inventory = MigrationInventory {
        source_count: sources.len(),
        entry_count: 0,
        tracked_entries: Vec::new(),
        untracked_entries: Vec::new(),
        invalid_entries: Vec::new(),
        missing_snapshots: Vec::new(),
        orphan_snapshots: Vec::new(),
        pending_snapshots: Vec::new(),
        identity_collisions: Vec::new(),
        entries_by_language: BTreeMap::new(),
        tracked_by_category: BTreeMap::new(),
    };
    let mut expected_snapshots = BTreeMap::<PathBuf, PathBuf>::new();

    for path in sources {
        let source = match fs::read_to_string(&path) {
            Ok(source) => source,
            Err(error) => {
                inventory
                    .invalid_entries
                    .push(format!("failed to read {}: {error}", path.display()));
                continue;
            }
        };
        let has_metadata = has_metadata_header(&path, &source);
        if !has_metadata && !is_named_entry_source(&path) {
            continue;
        }

        inventory.entry_count += 1;
        let language = path
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("unknown")
            .to_string();
        *inventory.entries_by_language.entry(language).or_default() += 1;

        if has_metadata {
            inventory.tracked_entries.push(path.clone());
            match parse_test_metadata(&path, &source) {
                Ok(metadata) => {
                    if !registry.contains(&metadata.feature) {
                        inventory.invalid_entries.push(format!(
                            "{}: unknown feature {}",
                            path.display(),
                            metadata.feature
                        ));
                    }
                    *inventory
                        .tracked_by_category
                        .entry(metadata.category.to_string())
                        .or_default() += 1;
                    validate_entry_name(&path, &metadata.category, &mut inventory.invalid_entries);
                }
                Err(errors) => inventory.invalid_entries.extend(errors.messages),
            }
        } else {
            inventory.untracked_entries.push(path.clone());
        }

        let identity = match fixture_identity(fixtures_root, &path, &source) {
            Ok(identity) => identity,
            Err(error) => {
                inventory.invalid_entries.push(error);
                continue;
            }
        };
        let snapshot = snapshots_root.join(format!("{}.snap", identity.replace('/', "-")));
        if !snapshot.is_file() {
            inventory.missing_snapshots.push(snapshot.clone());
        }
        if let Some(existing) = expected_snapshots.insert(snapshot.clone(), path.clone()) {
            inventory.identity_collisions.push(format!(
                "{} maps from {} and {}",
                snapshot.display(),
                existing.display(),
                path.display()
            ));
        }
    }

    for entry in WalkDir::new(snapshots_root).min_depth(1).max_depth(1) {
        match entry {
            Ok(entry) if entry.file_type().is_file() => {
                let path = entry.into_path();
                let filename = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("");
                if filename.ends_with(".snap.new") {
                    inventory.pending_snapshots.push(path);
                } else if path
                    .extension()
                    .is_some_and(|extension| extension == "snap")
                    && !expected_snapshots.contains_key(&path)
                {
                    inventory.orphan_snapshots.push(path);
                }
            }
            Ok(_) => {}
            Err(error) => inventory.invalid_entries.push(format!(
                "failed to walk {}: {error}",
                snapshots_root.display()
            )),
        }
    }

    inventory.tracked_entries.sort();
    inventory.untracked_entries.sort();
    inventory.invalid_entries.sort();
    inventory.missing_snapshots.sort();
    inventory.orphan_snapshots.sort();
    inventory.pending_snapshots.sort();
    inventory.identity_collisions.sort();
    Ok(inventory)
}

pub fn render_migration_inventory(
    repository_root: &Path,
    inventory: &MigrationInventory,
) -> String {
    let mut report = String::new();
    writeln!(report, "# Integration Test Migration Inventory\n").unwrap();
    writeln!(report, "| Metric | Count |").unwrap();
    writeln!(report, "|---|---:|").unwrap();
    writeln!(report, "| Source files | {} |", inventory.source_count).unwrap();
    writeln!(report, "| Entry candidates | {} |", inventory.entry_count).unwrap();
    writeln!(
        report,
        "| Tracked entries | {} |",
        inventory.tracked_entries.len()
    )
    .unwrap();
    writeln!(
        report,
        "| Untracked entries | {} |",
        inventory.untracked_entries.len()
    )
    .unwrap();
    writeln!(
        report,
        "| Invalid entries | {} |",
        inventory.invalid_entries.len()
    )
    .unwrap();
    writeln!(
        report,
        "| Missing snapshots | {} |",
        inventory.missing_snapshots.len()
    )
    .unwrap();
    writeln!(
        report,
        "| Orphan snapshots | {} |",
        inventory.orphan_snapshots.len()
    )
    .unwrap();
    writeln!(
        report,
        "| Pending snapshots | {} |\n",
        inventory.pending_snapshots.len()
    )
    .unwrap();

    render_count_section(
        &mut report,
        "Entries By Language",
        &inventory.entries_by_language,
    );
    render_count_section(
        &mut report,
        "Tracked Entries By Category",
        &inventory.tracked_by_category,
    );
    render_path_section(
        &mut report,
        "Untracked Entries",
        repository_root,
        &inventory.untracked_entries,
    );
    render_text_section(&mut report, "Invalid Entries", &inventory.invalid_entries);
    render_path_section(
        &mut report,
        "Missing Snapshots",
        repository_root,
        &inventory.missing_snapshots,
    );
    render_path_section(
        &mut report,
        "Orphan Snapshots",
        repository_root,
        &inventory.orphan_snapshots,
    );
    render_path_section(
        &mut report,
        "Pending Snapshots",
        repository_root,
        &inventory.pending_snapshots,
    );
    render_text_section(
        &mut report,
        "Identity Collisions",
        &inventory.identity_collisions,
    );
    report
}

pub fn render_test_coverage_report(
    version: &str,
    repository_root: &Path,
    registry: &FeatureRegistry,
    fixtures: &[TrackedFixture],
) -> String {
    let mut by_feature = BTreeMap::<&str, Vec<&TrackedFixture>>::new();
    let mut success_count = 0;
    let mut error_count = 0;
    for fixture in fixtures {
        by_feature
            .entry(&fixture.metadata.feature)
            .or_default()
            .push(fixture);
        match fixture.metadata.category {
            TestCategory::Success => success_count += 1,
            TestCategory::Error => error_count += 1,
        }
    }
    for fixtures in by_feature.values_mut() {
        fixtures.sort_by_key(|fixture| &fixture.source_path);
    }

    let mut report = String::new();
    writeln!(report, "# Integration Test Coverage\n").unwrap();
    writeln!(report, "Generated for Lagertha `{version}`.\n").unwrap();
    writeln!(
        report,
        "Coverage means passing integration snapshot evidence for a feature; it does not prove every criterion.\n"
    )
    .unwrap();
    writeln!(report, "## Summary\n").unwrap();
    writeln!(report, "| Metric | Count |").unwrap();
    writeln!(report, "|---|---:|").unwrap();
    writeln!(report, "| Features | {} |", registry.len()).unwrap();
    writeln!(report, "| Snapshot tests | {} |", fixtures.len()).unwrap();
    writeln!(report, "| Success tests | {success_count} |").unwrap();
    writeln!(report, "| Error tests | {error_count} |\n").unwrap();
    writeln!(report, "## Feature Coverage\n").unwrap();

    for (id, feature) in registry.iter() {
        let feature_fixtures = by_feature.get(id).map(Vec::as_slice).unwrap_or_default();
        writeln!(report, "### `{id}`\n").unwrap();
        writeln!(report, "Implementation: **{}**  ", feature.status).unwrap();
        writeln!(report, "Snapshot tests: {}\n", feature_fixtures.len()).unwrap();
        if feature_fixtures.is_empty() {
            writeln!(report, "No passing integration snapshot tests.\n").unwrap();
            continue;
        }

        writeln!(report, "| Category | Test | Description |").unwrap();
        writeln!(report, "|---|---|---|").unwrap();
        for fixture in feature_fixtures {
            let source = repository_relative_path(repository_root, &fixture.source_path);
            let display_name = fixture
                .source_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(&source);
            writeln!(
                report,
                "| {} | [`{}`](../../{}) | {} |",
                fixture.metadata.category,
                markdown_table_text(display_name),
                source,
                markdown_table_text(&fixture.metadata.description)
            )
            .unwrap();
        }
        report.push('\n');
    }

    render_gap_section(
        &mut report,
        "Features Without Integration Tests",
        registry
            .iter()
            .filter(|(id, _)| !by_feature.contains_key(id))
            .map(|(id, _)| id),
    );
    render_gap_section(
        &mut report,
        "Implemented Features Without Integration Tests",
        registry
            .iter()
            .filter(|(id, feature)| {
                feature.status == Status::Implemented && !by_feature.contains_key(id)
            })
            .map(|(id, _)| id),
    );
    render_gap_section(
        &mut report,
        "Partial Features With Regression Tests",
        registry
            .iter()
            .filter(|(id, feature)| {
                feature.status == Status::Partial && by_feature.contains_key(id)
            })
            .map(|(id, _)| id),
    );

    report
}

pub fn render_feature_report(
    version: &str,
    registry: &FeatureRegistry,
    fixtures: &[TrackedFixture],
) -> String {
    let mut snapshot_counts = BTreeMap::<&str, usize>::new();
    for fixture in fixtures {
        *snapshot_counts
            .entry(&fixture.metadata.feature)
            .or_default() += 1;
    }

    let mut categories = BTreeMap::<&str, Vec<(&str, &Feature)>>::new();
    let mut status_counts = BTreeMap::<String, usize>::new();
    for (id, feature) in registry.iter() {
        let category = id.split('.').next().unwrap_or(id);
        categories.entry(category).or_default().push((id, feature));
        *status_counts.entry(feature.status.to_string()).or_default() += 1;
    }

    let mut report = String::new();
    writeln!(report, "# Lagertha Features\n").unwrap();
    writeln!(report, "Generated for Lagertha `{version}`.\n").unwrap();
    writeln!(
        report,
        "Feature status describes declared JVM behavior. Test counts mean passing integration snapshot evidence, not exhaustive criterion coverage.\n"
    )
    .unwrap();
    writeln!(report, "## Summary\n").unwrap();
    writeln!(report, "| Status | Features |").unwrap();
    writeln!(report, "|---|---:|").unwrap();
    for status in [
        Status::Implemented,
        Status::Partial,
        Status::Missing,
        Status::Blocked,
        Status::Deferred,
    ] {
        writeln!(
            report,
            "| {status} | {} |",
            status_counts.get(&status.to_string()).copied().unwrap_or(0)
        )
        .unwrap();
    }
    report.push('\n');

    writeln!(report, "## Feature Index\n").unwrap();
    for (category, features) in categories {
        writeln!(report, "### {category}\n").unwrap();
        writeln!(report, "| Feature | Status | Tests | Description |").unwrap();
        writeln!(report, "|---|---|---:|---|").unwrap();
        for (id, feature) in features {
            writeln!(
                report,
                "| `{id}` | {} | {} | {} |",
                feature.status,
                snapshot_counts.get(id).copied().unwrap_or(0),
                markdown_table_text(&feature.description)
            )
            .unwrap();
        }
        report.push('\n');
    }

    writeln!(report, "## Feature Details\n").unwrap();
    for (id, feature) in registry.iter() {
        writeln!(report, "### `{id}`\n").unwrap();
        writeln!(report, "{}\n", feature.description).unwrap();
        writeln!(report, "Status: **{}**  ", feature.status).unwrap();
        writeln!(
            report,
            "Specification: {}  ",
            feature
                .spec
                .as_deref()
                .map(|spec| format!("<{spec}>"))
                .unwrap_or_else(|| "Not specified".to_string())
        )
        .unwrap();
        writeln!(
            report,
            "Snapshot tests: {}\n",
            snapshot_counts.get(id).copied().unwrap_or(0)
        )
        .unwrap();
        writeln!(report, "#### Criteria\n").unwrap();
        render_markdown_list(&mut report, &feature.criteria);
        if let Some(limitations) = &feature.limitations {
            writeln!(report, "#### Limitations\n").unwrap();
            render_markdown_list(&mut report, limitations);
        }
        if let Some(blocked_by) = &feature.blocked_by {
            writeln!(report, "#### Blocked By\n").unwrap();
            render_markdown_list(&mut report, blocked_by);
        }
        if let Some(reason) = &feature.reason {
            writeln!(report, "#### Deferred Reason\n").unwrap();
            writeln!(report, "{reason}\n").unwrap();
        }
    }

    report
}

pub fn write_report_atomic(path: &Path, report: &str) -> std::io::Result<()> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty());
    if let Some(parent) = parent {
        fs::create_dir_all(parent)?;
    }
    let parent = parent.unwrap_or_else(|| Path::new("."));
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("report.md");
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let temporary = parent.join(format!(".{filename}.{}.{}.tmp", std::process::id(), nonce));

    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(report.as_bytes())?;
        file.sync_all()?;
        fs::rename(&temporary, path)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

fn render_markdown_list(report: &mut String, values: &[String]) {
    for value in values {
        writeln!(report, "- {}", value.replace(['\r', '\n'], " ")).unwrap();
    }
    report.push('\n');
}

fn render_gap_section<'a>(report: &mut String, heading: &str, ids: impl Iterator<Item = &'a str>) {
    writeln!(report, "## {heading}\n").unwrap();
    let ids = ids.collect::<Vec<_>>();
    if ids.is_empty() {
        report.push_str("None.\n\n");
    } else {
        for id in ids {
            writeln!(report, "- `{id}`").unwrap();
        }
        report.push('\n');
    }
}

fn render_count_section(report: &mut String, heading: &str, counts: &BTreeMap<String, usize>) {
    writeln!(report, "## {heading}\n").unwrap();
    if counts.is_empty() {
        report.push_str("None.\n\n");
        return;
    }
    writeln!(report, "| Value | Count |").unwrap();
    writeln!(report, "|---|---:|").unwrap();
    for (value, count) in counts {
        writeln!(report, "| {} | {count} |", markdown_table_text(value)).unwrap();
    }
    report.push('\n');
}

fn render_path_section(report: &mut String, heading: &str, root: &Path, paths: &[PathBuf]) {
    writeln!(report, "## {heading}\n").unwrap();
    if paths.is_empty() {
        report.push_str("None.\n\n");
        return;
    }
    for path in paths {
        writeln!(report, "- `{}`", repository_relative_path(root, path)).unwrap();
    }
    report.push('\n');
}

fn render_text_section(report: &mut String, heading: &str, values: &[String]) {
    writeln!(report, "## {heading}\n").unwrap();
    if values.is_empty() {
        report.push_str("None.\n\n");
        return;
    }
    for value in values {
        writeln!(report, "- {}", value.replace(['\r', '\n'], " ")).unwrap();
    }
    report.push('\n');
}

fn repository_relative_path(repository_root: &Path, path: &Path) -> String {
    path.strip_prefix(repository_root)
        .unwrap_or(path)
        .iter()
        .map(|component| component.to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn markdown_table_text(value: &str) -> String {
    value.replace('|', "\\|").replace(['\r', '\n'], " ")
}

pub fn validate_registry(root: &Path) -> Result<FeatureRegistry, ValidationErrors> {
    if !root.is_dir() {
        return Err(errors([format!(
            "feature registry does not exist: {}",
            root.display()
        )]));
    }

    let mut files = Vec::new();
    let mut messages = Vec::new();
    for entry in WalkDir::new(root) {
        match entry {
            Ok(entry)
                if entry.file_type().is_file()
                    && entry
                        .path()
                        .extension()
                        .is_some_and(|extension| extension == "yaml") =>
            {
                files.push(entry.into_path());
            }
            Ok(_) => {}
            Err(error) => messages.push(format!("failed to walk {}: {error}", root.display())),
        }
    }
    files.sort();

    if files.is_empty() {
        messages.push(format!("feature registry is empty: {}", root.display()));
    }

    let mut ids = HashMap::<String, PathBuf>::new();
    let mut features = BTreeMap::new();
    for path in &files {
        let id = match feature_id(root, path) {
            Ok(id) => id,
            Err(message) => {
                messages.push(message);
                continue;
            }
        };

        if let Some(existing) = ids.insert(id.clone(), path.clone()) {
            messages.push(format!(
                "duplicate feature ID {id}: {} and {}",
                existing.display(),
                path.display()
            ));
        }

        match fs::read_to_string(path) {
            Ok(source) => match parse_feature_document(&id, path, &source) {
                Ok(feature) => {
                    features.insert(id, feature);
                }
                Err(errors) => messages.extend(errors),
            },
            Err(error) => messages.push(format!("failed to read {}: {error}", path.display())),
        }
    }

    if messages.is_empty() {
        Ok(FeatureRegistry { features })
    } else {
        Err(ValidationErrors { messages })
    }
}

pub fn parse_test_metadata(path: &Path, source: &str) -> Result<TestMetadata, ValidationErrors> {
    parse_shared_test_metadata(path, source).map_err(|error| errors([error.to_string()]))
}

pub fn validate_test_fixture(
    path: &Path,
    registry: &FeatureRegistry,
) -> Result<TestMetadata, ValidationErrors> {
    let source = fs::read_to_string(path).map_err(|error| {
        errors([format!(
            "failed to read fixture {}: {error}",
            path.display()
        )])
    })?;
    let metadata = parse_test_metadata(path, &source)?;
    if !registry.contains(&metadata.feature) {
        return Err(errors([format!(
            "{}: unknown feature {}",
            path.display(),
            metadata.feature
        )]));
    }
    Ok(metadata)
}

pub fn validate_tracked_fixtures(
    fixtures_root: &Path,
    snapshots_root: &Path,
    registry: &FeatureRegistry,
) -> Result<Vec<TrackedFixture>, ValidationErrors> {
    if !fixtures_root.is_dir() {
        return Err(errors([format!(
            "fixture root does not exist: {}",
            fixtures_root.display()
        )]));
    }
    if !snapshots_root.is_dir() {
        return Err(errors([format!(
            "snapshot root does not exist: {}",
            snapshots_root.display()
        )]));
    }

    let mut files = Vec::new();
    let mut messages = Vec::new();
    for entry in WalkDir::new(fixtures_root) {
        match entry {
            Ok(entry)
                if entry.file_type().is_file()
                    && matches!(
                        entry.path().extension().and_then(|value| value.to_str()),
                        Some("java" | "rns")
                    ) =>
            {
                files.push(entry.into_path());
            }
            Ok(_) => {}
            Err(error) => messages.push(format!(
                "failed to walk {}: {error}",
                fixtures_root.display()
            )),
        }
    }
    files.sort();

    let mut fixtures = Vec::new();
    let mut snapshots = HashMap::<PathBuf, PathBuf>::new();
    for path in files {
        let source = match fs::read_to_string(&path) {
            Ok(source) => source,
            Err(error) => {
                messages.push(format!(
                    "failed to read fixture {}: {error}",
                    path.display()
                ));
                continue;
            }
        };
        if !has_metadata_header(&path, &source) {
            continue;
        }

        let metadata = match parse_test_metadata(&path, &source) {
            Ok(metadata) => metadata,
            Err(errors) => {
                messages.extend(errors.messages);
                continue;
            }
        };
        if !registry.contains(&metadata.feature) {
            messages.push(format!(
                "{}: unknown feature {}",
                path.display(),
                metadata.feature
            ));
        }

        let identity = match fixture_identity(fixtures_root, &path, &source) {
            Ok(identity) => identity,
            Err(error) => {
                messages.push(error);
                continue;
            }
        };
        validate_entry_name(&path, &metadata.category, &mut messages);

        let snapshot_path = snapshots_root.join(format!("{}.snap", identity.replace('/', "-")));
        if !snapshot_path.is_file() {
            messages.push(format!(
                "{}: approved snapshot does not exist: {}",
                path.display(),
                snapshot_path.display()
            ));
        }
        if let Some(existing) = snapshots.insert(snapshot_path.clone(), path.clone()) {
            messages.push(format!(
                "snapshot identity collision: {} maps from {} and {}",
                snapshot_path.display(),
                existing.display(),
                path.display()
            ));
        }

        fixtures.push(TrackedFixture {
            source_path: path,
            snapshot_path,
            metadata,
        });
    }

    if messages.is_empty() {
        Ok(fixtures)
    } else {
        Err(ValidationErrors { messages })
    }
}

fn metadata_comment(path: &Path) -> Result<&'static str, ValidationErrors> {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("java") => Ok("//"),
        Some("rns") => Ok(";"),
        _ => Err(errors([format!(
            "unsupported fixture extension: {}",
            path.display()
        )])),
    }
}

fn has_metadata_header(path: &Path, source: &str) -> bool {
    let Ok(comment) = metadata_comment(path) else {
        return false;
    };
    source
        .lines()
        .next()
        .is_some_and(|line| line.starts_with(&format!("{comment} @test ")))
}

fn is_named_entry_source(path: &Path) -> bool {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .is_some_and(|stem| {
            stem.ends_with("Test") || stem.ends_with("OkMain") || stem.ends_with("ErrMain")
        })
}

pub fn fixture_identity(fixtures_root: &Path, path: &Path, source: &str) -> Result<String, String> {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("java") => java_fixture_identity(path, source),
        Some("rns") => rns_fixture_identity(fixtures_root, path, source),
        _ => Err(format!("unsupported fixture extension: {}", path.display())),
    }
}

fn java_fixture_identity(path: &Path, source: &str) -> Result<String, String> {
    let class_name = path
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            format!(
                "fixture filename does not define a UTF-8 class name: {}",
                path.display()
            )
        })?;
    let package = source.lines().find_map(|line| {
        line.trim()
            .strip_prefix("package ")
            .and_then(|value| value.strip_suffix(';'))
            .map(str::trim)
    });

    match package {
        Some("") => Err(format!(
            "{}: Java package must not be empty",
            path.display()
        )),
        Some(package) => Ok(format!("{}/{class_name}", package.replace('.', "/"))),
        None => Ok(class_name.to_string()),
    }
}

fn rns_fixture_identity(fixtures_root: &Path, path: &Path, source: &str) -> Result<String, String> {
    let relative = path.strip_prefix(fixtures_root).map_err(|_| {
        format!(
            "fixture {} is outside root {}",
            path.display(),
            fixtures_root.display()
        )
    })?;
    let mut components = relative.components();
    let first = components.next();
    let relative = if first.is_some_and(|component| component.as_os_str() == "rns") {
        components.as_path()
    } else {
        relative
    };
    let output_path = relative.with_extension("");
    let output_identity = output_path
        .iter()
        .map(|component| component.to_str())
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| format!("fixture path is not UTF-8: {}", path.display()))?
        .join("/");
    let declared_identity = source
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix(".class ")
                .and_then(|declaration| declaration.split_whitespace().last())
        })
        .ok_or_else(|| format!("{}: RNS fixture has no .class declaration", path.display()))?;

    if declared_identity != output_identity {
        return Err(format!(
            "{}: RNS class {declared_identity} does not match compiled path {output_identity}",
            path.display()
        ));
    }
    Ok(output_identity)
}

fn validate_entry_name(path: &Path, category: &TestCategory, messages: &mut Vec<String>) {
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_default();
    let valid = stem.ends_with("Test")
        || matches!(category, TestCategory::Success) && stem.ends_with("OkMain")
        || matches!(category, TestCategory::Error) && stem.ends_with("ErrMain");
    if !valid {
        messages.push(format!(
            "{}: {:?} entry must end with Test or its legacy outcome suffix",
            path.display(),
            category
        ));
    }
}

fn feature_id(root: &Path, path: &Path) -> Result<String, String> {
    let relative = path.strip_prefix(root).map_err(|_| {
        format!(
            "feature file {} is outside registry {}",
            path.display(),
            root.display()
        )
    })?;
    let without_extension = relative.with_extension("");
    let segments = without_extension
        .iter()
        .map(|segment| segment.to_str())
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| format!("feature path is not UTF-8: {}", path.display()))?;
    let id = segments.join(".");
    validate_feature_id(&id).map_err(|error| {
        format!(
            "feature path {} produces invalid ID {id:?}: {error}",
            path.display()
        )
    })?;
    Ok(id)
}

fn parse_feature_document(id: &str, path: &Path, source: &str) -> Result<Feature, Vec<String>> {
    let feature = match serde_yaml::from_str::<Feature>(source) {
        Ok(feature) => feature,
        Err(error) => return Err(vec![format!("{} ({id}): {error}", path.display())]),
    };

    let mut messages = Vec::new();
    require_text(&mut messages, id, "name", &feature.name);
    require_text(&mut messages, id, "description", &feature.description);
    require_list(&mut messages, id, "criteria", Some(&feature.criteria));
    if let Some(spec) = &feature.spec {
        require_text(&mut messages, id, "spec", spec);
        validate_spec_url(&mut messages, id, spec);
    }

    match feature.status {
        Status::Missing | Status::Implemented => {
            forbid(&mut messages, id, "limitations", &feature.limitations);
            forbid(&mut messages, id, "blocked_by", &feature.blocked_by);
            forbid(&mut messages, id, "reason", &feature.reason);
        }
        Status::Partial => {
            require_list(
                &mut messages,
                id,
                "limitations",
                feature.limitations.as_deref(),
            );
            forbid(&mut messages, id, "blocked_by", &feature.blocked_by);
            forbid(&mut messages, id, "reason", &feature.reason);
        }
        Status::Blocked => {
            require_list(
                &mut messages,
                id,
                "blocked_by",
                feature.blocked_by.as_deref(),
            );
            forbid(&mut messages, id, "limitations", &feature.limitations);
            forbid(&mut messages, id, "reason", &feature.reason);
        }
        Status::Deferred => {
            match feature.reason.as_deref() {
                Some(reason) => require_text(&mut messages, id, "reason", reason),
                None => messages.push(format!("{id}: reason is required for deferred features")),
            }
            forbid(&mut messages, id, "limitations", &feature.limitations);
            forbid(&mut messages, id, "blocked_by", &feature.blocked_by);
        }
    }

    if messages.is_empty() {
        Ok(feature)
    } else {
        Err(messages)
    }
}

fn require_text(messages: &mut Vec<String>, id: &str, field: &str, value: &str) {
    if value.trim().is_empty() {
        messages.push(format!("{id}: {field} must not be empty"));
    }
}

fn validate_spec_url(messages: &mut Vec<String>, id: &str, spec: &str) {
    const JVMS_ROOT: &str = "https://docs.oracle.com/javase/specs/jvms/se25/html/";
    const JLS_ROOT: &str = "https://docs.oracle.com/javase/specs/jls/se25/html/";

    let valid =
        [(JVMS_ROOT, "jvms-"), (JLS_ROOT, "jls-")]
            .into_iter()
            .any(|(root, fragment_prefix)| {
                spec.strip_prefix(root).is_some_and(|reference| {
                    reference.split_once('#').is_some_and(|(page, fragment)| {
                        page.ends_with(".html") && fragment.starts_with(fragment_prefix)
                    })
                })
            });
    if !valid {
        messages.push(format!(
            "{id}: spec must be a direct Java SE 25 JVMS or JLS section URL"
        ));
    }
}

fn require_list(messages: &mut Vec<String>, id: &str, field: &str, values: Option<&[String]>) {
    match values {
        Some([]) | None => messages.push(format!("{id}: {field} must not be empty")),
        Some(values) => {
            for (index, value) in values.iter().enumerate() {
                if value.trim().is_empty() {
                    messages.push(format!("{id}: {field}[{index}] must not be empty"));
                }
            }
        }
    }
}

fn forbid<T>(messages: &mut Vec<String>, id: &str, field: &str, value: &Option<T>) {
    if value.is_some() {
        messages.push(format!("{id}: {field} is not valid for this status"));
    }
}

fn errors(messages: impl IntoIterator<Item = String>) -> ValidationErrors {
    ValidationErrors {
        messages: messages.into_iter().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = r#"
name: Integer addition
description: Adds two integer values using Java wrapping semantics.
status: implemented
spec: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.iadd
criteria:
  - Decodes the iadd opcode.
"#;

    #[test]
    fn accepts_valid_implemented_feature() {
        assert!(parse_feature_document("opcodes.iadd", Path::new("iadd.yaml"), BASE).is_ok());
    }

    #[test]
    fn requires_status_specific_fields() {
        let source = BASE.replace("status: implemented", "status: partial");
        let messages =
            parse_feature_document("opcodes.iadd", Path::new("iadd.yaml"), &source).unwrap_err();

        assert_eq!(messages, ["opcodes.iadd: limitations must not be empty"]);
    }

    #[test]
    fn rejects_fields_for_another_status() {
        let source = format!("{BASE}limitations:\n  - Overflow is not handled.\n");
        let messages =
            parse_feature_document("opcodes.iadd", Path::new("iadd.yaml"), &source).unwrap_err();

        assert_eq!(
            messages,
            ["opcodes.iadd: limitations is not valid for this status"]
        );
    }

    #[test]
    fn rejects_unknown_fields() {
        let source = format!("{BASE}owner: runtime\n");
        let messages =
            parse_feature_document("opcodes.iadd", Path::new("iadd.yaml"), &source).unwrap_err();

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("unknown field `owner`"));
    }

    #[test]
    fn rejects_non_url_specification_reference() {
        let source = BASE.replace(
            "https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.iadd",
            "JVMS 6.5.iadd",
        );

        let messages =
            parse_feature_document("opcodes.iadd", Path::new("iadd.yaml"), &source).unwrap_err();

        assert_eq!(
            messages,
            ["opcodes.iadd: spec must be a direct Java SE 25 JVMS or JLS section URL"]
        );
    }

    #[test]
    fn parses_java_metadata() {
        let source = r#"// @test feature = "opcodes.arithmetic.iadd"
// @test description = "Adds two values."
// @test category = "success"

class AddOkMain {}
"#;

        let metadata = parse_test_metadata(Path::new("AddOkMain.java"), source).unwrap();

        assert_eq!(metadata.feature, "opcodes.arithmetic.iadd");
        assert_eq!(metadata.description, "Adds two values.");
        assert_eq!(metadata.category, TestCategory::Success);
    }

    #[test]
    fn parses_rns_metadata_with_escapes() {
        let source = r#"; @test feature = "class-format.interface-flags"
; @test description = "Rejects a \"bad\" interface."
; @test category = "error"

.class_end
"#;

        let metadata = parse_test_metadata(Path::new("BadErrMain.rns"), source).unwrap();

        assert_eq!(metadata.description, "Rejects a \"bad\" interface.");
        assert_eq!(metadata.category, TestCategory::Error);
    }

    #[test]
    fn rejects_incomplete_metadata() {
        let source = r#"// @test description = "Wrong first field."
// @test feature = "opcodes.arithmetic.iadd"
// @test category = "success"
"#;

        let errors = parse_test_metadata(Path::new("AddOkMain.java"), source).unwrap_err();

        assert!(errors.messages()[0].contains("expected `// @test feature"));
    }

    #[test]
    fn rejects_extra_metadata_comment() {
        let source = r#"; @test feature = "class-format.interface-flags"
; @test description = "Rejects invalid flags."
; @test category = "error"
; @test owner = "lvm-class"
"#;

        let errors = parse_test_metadata(Path::new("BadErrMain.rns"), source).unwrap_err();

        assert!(errors.messages()[0].contains("exactly three metadata comments"));
    }

    #[test]
    fn derives_java_snapshot_identity_from_package() {
        let source = "package opcodes.arithmetic.iadd;\nclass AddOkMain {}\n";

        let identity = java_fixture_identity(Path::new("AddOkMain.java"), source).unwrap();

        assert_eq!(identity, "opcodes/arithmetic/iadd/AddOkMain");
    }

    #[test]
    fn derives_rns_snapshot_identity_from_compiled_path() {
        let root = Path::new("tests/testdata");
        let path = root.join("rns/class_format/BadErrMain.rns");
        let source = ".class interface class_format/BadErrMain\n.class_end\n";

        let identity = rns_fixture_identity(root, &path, source).unwrap();

        assert_eq!(identity, "class_format/BadErrMain");
    }

    #[test]
    fn rejects_rns_class_that_differs_from_compiled_path() {
        let root = Path::new("tests/testdata");
        let path = root.join("rns/class_format/BadErrMain.rns");
        let source = ".class interface other/BadErrMain\n.class_end\n";

        let error = rns_fixture_identity(root, &path, source).unwrap_err();

        assert!(error.contains("does not match compiled path class_format/BadErrMain"));
    }

    #[test]
    fn category_must_match_legacy_outcome_suffix() {
        let mut messages = Vec::new();

        validate_entry_name(
            Path::new("FailureOkMain.java"),
            &TestCategory::Error,
            &mut messages,
        );

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("legacy outcome suffix"));
    }

    #[test]
    fn marked_entry_name_does_not_encode_category() {
        for category in [TestCategory::Success, TestCategory::Error] {
            let mut messages = Vec::new();

            validate_entry_name(Path::new("ArithmeticTest.java"), &category, &mut messages);

            assert!(messages.is_empty());
        }
    }

    #[test]
    fn discovers_marked_and_legacy_entry_names() {
        assert!(is_named_entry_source(Path::new("ArithmeticTest.java")));
        assert!(is_named_entry_source(Path::new("ArithmeticOkMain.java")));
        assert!(is_named_entry_source(Path::new("ArithmeticErrMain.rns")));
        assert!(!is_named_entry_source(Path::new("ArithmeticHelper.java")));
    }

    #[test]
    fn renders_deterministic_coverage_report() {
        let registry = FeatureRegistry {
            features: BTreeMap::from([
                (
                    "a.covered".to_string(),
                    test_feature("Covered", Status::Implemented),
                ),
                (
                    "b.missing".to_string(),
                    test_feature("Missing", Status::Implemented),
                ),
                (
                    "c.partial".to_string(),
                    test_feature("Partial", Status::Partial),
                ),
            ]),
        };
        let fixtures = vec![
            TrackedFixture {
                source_path: PathBuf::from("/repo/vm/tests/testdata/AOkMain.java"),
                snapshot_path: PathBuf::from("/repo/vm/snapshots/AOkMain.snap"),
                metadata: TestMetadata {
                    feature: "a.covered".to_string(),
                    description: "Covers A | B.".to_string(),
                    category: TestCategory::Success,
                },
            },
            TrackedFixture {
                source_path: PathBuf::from("/repo/vm/tests/testdata/CErrMain.rns"),
                snapshot_path: PathBuf::from("/repo/vm/snapshots/CErrMain.snap"),
                metadata: TestMetadata {
                    feature: "c.partial".to_string(),
                    description: "Preserves known behavior.".to_string(),
                    category: TestCategory::Error,
                },
            },
        ];

        let report = render_test_coverage_report("0.5.0", Path::new("/repo"), &registry, &fixtures);

        assert_eq!(
            report,
            r#"# Integration Test Coverage

Generated for Lagertha `0.5.0`.

Coverage means passing integration snapshot evidence for a feature; it does not prove every criterion.

## Summary

| Metric | Count |
|---|---:|
| Features | 3 |
| Snapshot tests | 2 |
| Success tests | 1 |
| Error tests | 1 |

## Feature Coverage

### `a.covered`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`AOkMain.java`](../../vm/tests/testdata/AOkMain.java) | Covers A \| B. |

### `b.missing`

Implementation: **Implemented**  
Snapshot tests: 0

No passing integration snapshot tests.

### `c.partial`

Implementation: **Partial**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Error | [`CErrMain.rns`](../../vm/tests/testdata/CErrMain.rns) | Preserves known behavior. |

## Features Without Integration Tests

- `b.missing`

## Implemented Features Without Integration Tests

- `b.missing`

## Partial Features With Regression Tests

- `c.partial`

"#
        );
    }

    fn test_feature(name: &str, status: Status) -> Feature {
        Feature {
            name: name.to_string(),
            description: format!("{name} description"),
            status,
            spec: None,
            criteria: vec![format!("{name} criterion")],
            limitations: None,
            blocked_by: None,
            reason: None,
        }
    }
}
