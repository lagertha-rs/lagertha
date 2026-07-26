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

#[derive(Debug, PartialEq, Eq)]
pub struct TestMetadata {
    pub feature: String,
    pub description: String,
    pub category: TestCategory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestCategory {
    Success,
    Error,
}

impl Display for TestCategory {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Success => "Success",
            Self::Error => "Error",
        })
    }
}

#[derive(Debug)]
pub struct TrackedFixture {
    pub source_path: PathBuf,
    pub snapshot_path: PathBuf,
    pub metadata: TestMetadata,
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
            feature.spec.as_deref().unwrap_or("Not specified")
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
    let comment = metadata_comment(path)?;
    let lines = source.lines().collect::<Vec<_>>();
    if lines.len() < 3 {
        return Err(errors([format!(
            "{}: fixture must start with three metadata comments",
            path.display()
        )]));
    }

    let feature = metadata_value(path, lines[0], comment, "feature")?;
    let description = metadata_value(path, lines[1], comment, "description")?;
    let category = metadata_value(path, lines[2], comment, "category")?;
    if lines
        .get(3)
        .is_some_and(|line| line.starts_with(&format!("{comment} @test ")))
    {
        return Err(errors([format!(
            "{}: fixture must have exactly three metadata comments",
            path.display()
        )]));
    }

    let mut messages = Vec::new();
    require_text(
        &mut messages,
        &path.display().to_string(),
        "feature",
        &feature,
    );
    require_text(
        &mut messages,
        &path.display().to_string(),
        "description",
        &description,
    );
    let category = match category.as_str() {
        "success" => Some(TestCategory::Success),
        "error" => Some(TestCategory::Error),
        value => {
            messages.push(format!(
                "{}: unsupported test category {value:?}",
                path.display()
            ));
            None
        }
    };

    match (messages.is_empty(), category) {
        (true, Some(category)) => Ok(TestMetadata {
            feature,
            description,
            category,
        }),
        _ => Err(ValidationErrors { messages }),
    }
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
        validate_category_suffix(&path, &identity, &metadata.category, &mut messages);

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

fn fixture_identity(fixtures_root: &Path, path: &Path, source: &str) -> Result<String, String> {
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

fn validate_category_suffix(
    path: &Path,
    identity: &str,
    category: &TestCategory,
    messages: &mut Vec<String>,
) {
    let expected = match category {
        TestCategory::Success => "OkMain",
        TestCategory::Error => "ErrMain",
    };
    if !identity.ends_with(expected) {
        messages.push(format!(
            "{}: {:?} test identity must end with {expected}",
            path.display(),
            category
        ));
    }
}

fn metadata_value(
    path: &Path,
    line: &str,
    comment: &str,
    field: &str,
) -> Result<String, ValidationErrors> {
    let prefix = format!("{comment} @test {field} = \"");
    let encoded = line
        .strip_prefix(&prefix)
        .and_then(|value| value.strip_suffix('"'))
        .ok_or_else(|| {
            errors([format!(
                "{}: expected `{comment} @test {field} = \"...\"`",
                path.display()
            )])
        })?;

    let mut value = String::new();
    let mut chars = encoded.chars();
    while let Some(character) = chars.next() {
        match character {
            '\\' => match chars.next() {
                Some('"') => value.push('"'),
                Some('\\') => value.push('\\'),
                _ => {
                    return Err(errors([format!(
                        "{}: {field} contains an invalid escape",
                        path.display()
                    )]));
                }
            },
            '"' => {
                return Err(errors([format!(
                    "{}: {field} contains an unescaped quote",
                    path.display()
                )]));
            }
            character => value.push(character),
        }
    }
    Ok(value)
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
    Ok(segments.join("."))
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
spec: JVMS 6.5.iadd
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
    fn category_must_match_harness_suffix() {
        let mut messages = Vec::new();

        validate_category_suffix(
            Path::new("FailureOkMain.java"),
            "FailureOkMain",
            &TestCategory::Error,
            &mut messages,
        );

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("must end with ErrMain"));
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
