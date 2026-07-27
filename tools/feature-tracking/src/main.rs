use feature_tracking::{
    build_migration_inventory, render_feature_report, render_migration_inventory,
    render_test_coverage_report, validate_registry, validate_tracked_fixtures, write_report_atomic,
};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

enum ReportKind {
    Coverage,
    Features,
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    match args.next().as_deref() {
        Some(command) if command == "coverage" => run_report(ReportKind::Coverage, args),
        Some(command) if command == "feature-report" => run_report(ReportKind::Features, args),
        Some(command) if command == "inventory" => run_inventory(args),
        first => run_validation(first.map(OsStr::to_os_string), args),
    }
}

fn run_inventory(mut args: impl Iterator<Item = OsString>) {
    let output = match args.next() {
        Some(argument) if argument == "--output" => {
            Some(PathBuf::from(args.next().unwrap_or_else(|| usage())))
        }
        Some(_) => usage(),
        None => None,
    };
    if args.next().is_some() {
        usage();
    }

    let repository_root = std::env::current_dir().expect("cannot read current directory");
    let registry = validate_registry(Path::new("features")).unwrap_or_else(|errors| {
        eprint!("{errors}");
        std::process::exit(1);
    });
    let inventory = build_migration_inventory(
        Path::new("vm/tests/testdata"),
        Path::new("vm/snapshots"),
        &registry,
    )
    .unwrap_or_else(|errors| {
        eprint!("{errors}");
        std::process::exit(1);
    });
    let report = render_migration_inventory(&repository_root, &inventory);
    if let Some(output) = output {
        write_report_atomic(&output, &report).unwrap_or_else(|error| {
            eprintln!("failed to write {}: {error}", output.display());
            std::process::exit(1);
        });
        println!("generated {}", output.display());
    } else {
        print!("{report}");
    }
    if !inventory.is_clean() {
        std::process::exit(1);
    }
}

fn run_validation(first: Option<OsString>, mut args: impl Iterator<Item = OsString>) {
    let features_root = first
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("features"));
    let fixtures_root = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("vm/tests/testdata"));
    let snapshots_root = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("vm/snapshots"));
    if args.next().is_some() {
        usage();
    }

    let (registry, fixtures) = validated_inputs(&features_root, &fixtures_root, &snapshots_root);
    println!(
        "validated {} feature files and {} tracked fixtures",
        registry.len(),
        fixtures.len()
    );
}

fn run_report(kind: ReportKind, mut args: impl Iterator<Item = OsString>) {
    let version = args.next().map(required_utf8).unwrap_or_else(|| usage());
    let mut output = None;
    while let Some(argument) = args.next() {
        if argument != "--output" || output.is_some() {
            usage();
        }
        output = Some(PathBuf::from(args.next().unwrap_or_else(|| usage())));
    }

    let repository_root = std::env::current_dir().expect("cannot read current directory");
    let (registry, fixtures) = validated_inputs(
        Path::new("features"),
        Path::new("vm/tests/testdata"),
        Path::new("vm/snapshots"),
    );
    let report = match kind {
        ReportKind::Coverage => {
            render_test_coverage_report(&version, &repository_root, &registry, &fixtures)
        }
        ReportKind::Features => render_feature_report(&version, &registry),
    };

    if let Some(output) = output {
        write_report_atomic(&output, &report).unwrap_or_else(|error| {
            eprintln!("failed to write {}: {error}", output.display());
            std::process::exit(1);
        });
        println!("generated {}", output.display());
    } else {
        print!("{report}");
    }
}

fn validated_inputs(
    features_root: &Path,
    fixtures_root: &Path,
    snapshots_root: &Path,
) -> (
    feature_tracking::FeatureRegistry,
    Vec<feature_tracking::TrackedFixture>,
) {
    let registry = validate_registry(features_root).unwrap_or_else(|errors| {
        eprint!("{errors}");
        std::process::exit(1);
    });
    let fixtures = validate_tracked_fixtures(fixtures_root, snapshots_root, &registry)
        .unwrap_or_else(|errors| {
            eprint!("{errors}");
            std::process::exit(1);
        });
    (registry, fixtures)
}

fn required_utf8(value: OsString) -> String {
    value.into_string().unwrap_or_else(|_| {
        eprintln!("version must be valid UTF-8");
        std::process::exit(2);
    })
}

fn usage() -> ! {
    eprintln!(
        "usage:\n  feature-tracking [features] [fixtures] [snapshots]\n  feature-tracking inventory [--output <path>]\n  feature-tracking coverage <version> [--output <path>]\n  feature-tracking feature-report <version> [--output <path>]"
    );
    std::process::exit(2);
}
