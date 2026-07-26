# Testing Lagertha

This guide owns Lagertha's test commands, fixture pipeline, integration-test
discovery, and snapshot workflow.

## Test Commands

Run all workspace tests:

```bash
cargo test --workspace --locked
```

Run one package or test-name substring:

```bash
cargo test -p lvm-common
cargo test -p <package> <test-name-substring>
```

Run one VM fixture or related fixture group:

```bash
cargo test -p vm --test integration_test <fixture-name-substring>
```

Example:

```bash
cargo test -p vm --test integration_test HelloWorld
```

Commands that build `vm` require the exact Java and Runestaff toolchain defined
in [`BUILDING.md`](BUILDING.md).

## Fixture Compilation

`vm/build.rs` defines fixture compilation:

- `.java` sources anywhere under `vm/tests/testdata` compile with `javac`.
- `.rns` sources anywhere under `vm/tests/testdata` compile with `rnsc`.
- Existing `java/` and `rns/` roots remain supported during migration.
- Generated classes go under ignored `vm/tests/testdata/compiled`.
- The build script deletes and recreates compiled fixture output.

Never edit generated `.class` files.

Use Java fixtures for normal source-level behavior produced by `javac`. Use RNS
fixtures for exact bytecode, malformed class files, verifier edge cases, or
inputs `javac` cannot produce.

## Integration-Test Discovery

`vm/tests/integration_test.rs` currently supports two discovery paths during
migration.

New metadata-driven fixtures are discovered from source:

- `*Test.java` and `*Test.rns` are integration entries.
- The metadata `category` determines whether both VMs should succeed or fail.
- Entry names do not encode expected outcome.

Unmigrated compiled fixtures retain legacy suffix discovery:

- `*OkMain.class` must exit successfully.
- `*ErrMain.class` must exit unsuccessfully.

Other source names are helpers and do not become integration entries. Remove
legacy suffix discovery after every entry has metadata and a `*Test` name.

The harness runs each fixture twice:

1. On Lagertha.
2. On the reference JDK with assertions enabled using `java -ea`.

It snapshots combined stdout, stderr, and exit codes under `vm/snapshots`.

Keep fixtures focused enough that their intended capability and expected result
are clear. Follow [`FEATURE_TRACKING.md`](FEATURE_TRACKING.md) for fixture
metadata and feature-coverage ownership as that system is introduced.

## Snapshot Review

Review changed snapshots with:

```bash
cargo insta review
```

Snapshot approval requires semantic review. Confirm:

- Lagertha exited with the expected status.
- The reference JVM exited with the expected status.
- Output and exception behavior match the test's intent.
- Any Lagertha/reference-JVM difference is understood and intentional.
- The fixture proves the feature it claims to test.

Do not blindly accept all snapshots. The combined snapshot records both runs but
does not itself assert that their outputs are equivalent.

## Adding A Fixture

1. Choose Java for source-level behavior or RNS for exact bytecode.
2. Give the entry source a `*Test.java` or `*Test.rns` name.
3. Add exactly three metadata comments at the start of the entry source.
4. Keep the case focused on one primary feature.
5. Run the feature-tracking validator and focused integration test.
6. Review and accept the new snapshot semantically.
7. Run broader tests required by the affected subsystem.

## Feature Tracking Migration

Feature tracking is being introduced incrementally. The target architecture,
strictness rules, unresolved fixture layout, and implementation sequence live
in [`TEST_MIGRATION.md`](TEST_MIGRATION.md).

During migration, the existing fixture compiler and integration harness remain
authoritative:

- Java and RNS sources may remain under their existing language roots.
- `OkMain` and `ErrMain` suffixes still control discovery and expected exit
  behavior.
- The current tool only includes fixtures whose first line starts with `@test`
  metadata in validation and coverage reports. This temporary behavior must be
  replaced by strict validation before bulk migration.

Validate the registry, migrated metadata, and snapshot mappings with:

```bash
cargo run -p feature-tracking -- features
```

Generate the migration reports with an honest `unreleased` label:

```bash
cargo run -p feature-tracking -- coverage unreleased \
  --output docs/features/TEST_COVERAGE.md
cargo run -p feature-tracking -- feature-report unreleased \
  --output docs/features/README.md
```

Report writes are atomic. Omitting `--output` prints either report to stdout.

Do not bulk-migrate fixtures until the entry-source layout is selected and the
shared metadata parser and strict validator are ready. Do not copy the current
fixture hierarchy into the feature registry. Metadata inserted before Java
source shifts stack-trace line numbers even when runtime behavior is unchanged.
See [`FEATURE_TRACKING.md`](FEATURE_TRACKING.md) for schemas and permanent
coverage rules.
