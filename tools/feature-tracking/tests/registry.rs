use feature_tracking::{
    TestCategory, build_migration_inventory, render_feature_report, render_migration_inventory,
    render_test_coverage_report, validate_registry, validate_tracked_fixtures, write_report_atomic,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn repository_feature_registry_is_valid() {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let result = validate_registry(&repository_root.join("features"));

    result.expect("feature registry should be valid");
}

#[test]
fn tracked_fixture_metadata_and_snapshots_are_valid() {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let registry = validate_registry(&repository_root.join("features"))
        .expect("feature registry should be valid");
    let fixtures = validate_tracked_fixtures(
        &repository_root.join("vm/tests/testdata"),
        &repository_root.join("vm/snapshots"),
        &registry,
    )
    .expect("tracked fixtures should be valid");
    let fixture = fixtures
        .iter()
        .find(|fixture| {
            fixture
                .source_path
                .ends_with("class_format/InterfaceFlagWithoutAbstractTest.rns")
        })
        .expect("RNS fixture should be tracked");

    assert_eq!(fixture.metadata.feature, "class-format.interface-flags");
    assert_eq!(fixture.metadata.category, TestCategory::Error);
}

#[test]
fn generated_coverage_file_matches_renderer_output() {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let registry = validate_registry(&repository_root.join("features"))
        .expect("feature registry should be valid");
    let fixtures = validate_tracked_fixtures(
        &repository_root.join("vm/tests/testdata"),
        &repository_root.join("vm/snapshots"),
        &registry,
    )
    .expect("tracked fixtures should be valid");
    let report = render_test_coverage_report("test", &repository_root, &registry, &fixtures);
    let temporary_root = temporary_test_directory();
    let output = temporary_root.join("nested/TEST_COVERAGE.md");

    write_report_atomic(&output, &report).expect("coverage report should be written");

    assert_eq!(fs::read_to_string(&output).unwrap(), report);
    fs::remove_dir_all(temporary_root).unwrap();
}

#[test]
fn repository_feature_report_contains_details_and_counts() {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let registry = validate_registry(&repository_root.join("features"))
        .expect("feature registry should be valid");
    let fixtures = validate_tracked_fixtures(
        &repository_root.join("vm/tests/testdata"),
        &repository_root.join("vm/snapshots"),
        &registry,
    )
    .expect("tracked fixtures should be valid");

    let report = render_feature_report("test", &registry, &fixtures);

    assert!(report.contains("### execution"));
    assert!(report.contains("| `execution.integer.arithmetic` | Implemented | 5 |"));
    assert!(report.contains("- Throws ArithmeticException when an integer divisor is zero."));
    assert_eq!(report, render_feature_report("test", &registry, &fixtures));
}

#[test]
fn checked_in_generated_reports_are_current() {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let registry = validate_registry(&repository_root.join("features"))
        .expect("feature registry should be valid");
    let fixtures = validate_tracked_fixtures(
        &repository_root.join("vm/tests/testdata"),
        &repository_root.join("vm/snapshots"),
        &registry,
    )
    .expect("tracked fixtures should be valid");
    let coverage =
        render_test_coverage_report("unreleased", &repository_root, &registry, &fixtures);
    let features = render_feature_report("unreleased", &registry, &fixtures);

    assert_eq!(
        fs::read_to_string(repository_root.join("docs/features/TEST_COVERAGE.md")).unwrap(),
        coverage
    );
    assert_eq!(
        fs::read_to_string(repository_root.join("docs/features/README.md")).unwrap(),
        features
    );
}

#[test]
fn repository_fixture_inventory_is_clean() {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let registry = validate_registry(&repository_root.join("features"))
        .expect("feature registry should be valid");

    let inventory = build_migration_inventory(
        &repository_root.join("vm/tests/testdata"),
        &repository_root.join("vm/snapshots"),
        &registry,
    )
    .expect("migration inventory should be built");
    let report = render_migration_inventory(&repository_root, &inventory);

    assert!(inventory.is_clean(), "{report}");
    assert_eq!(inventory.entry_count, inventory.tracked_entries.len());
    assert!(inventory.untracked_entries.is_empty());
    assert!(inventory.invalid_entries.is_empty());
    assert!(inventory.missing_snapshots.is_empty());
    assert!(inventory.orphan_snapshots.is_empty());
    assert!(inventory.pending_snapshots.is_empty());
    assert!(inventory.identity_collisions.is_empty());
}

fn temporary_test_directory() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "lagertha-feature-tracking-{}-{nonce}",
        std::process::id()
    ))
}
