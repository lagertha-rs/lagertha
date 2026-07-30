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
- Generated classes go under ignored `vm/tests/testdata/compiled`.
- The build script deletes and recreates compiled fixture output.

Never edit generated `.class` files.

Use Java fixtures for normal source-level behavior produced by `javac`. Use RNS
fixtures for exact bytecode, malformed class files, verifier edge cases, or
inputs `javac` cannot produce.

## Integration-Test Discovery

`vm/tests/integration_test.rs` discovers integration entries from source:

- `*Test.java` and `*Test.rns` are integration entries.
- Every entry starts with exactly three metadata comments.
- The metadata `category` determines whether both VMs should succeed or fail.
- Entry names do not encode expected outcome.

Other source names are helpers and do not become integration entries. Helpers
must not end in `Test` and must not contain entry metadata.

The harness runs each fixture twice:

1. On Lagertha.
2. On the reference JDK with assertions enabled using `java -ea`.

It snapshots combined stdout, stderr, and exit codes under `vm/snapshots`.

Keep fixtures focused enough that their intended capability and expected result
are clear. Follow [`FEATURE_TRACKING.md`](FEATURE_TRACKING.md) for fixture
metadata and feature-coverage ownership.

## Issue-Driven TDD

For behavioral Issues, establish the integration failure before changing
production code:

1. Read the Issue, relevant feature definition, and direct Java SE 25 sections.
2. Choose Java for source-level behavior or RNS for exact class-file behavior.
3. Add the smallest focused fixture with its intended final `success` or `error`
   category. The category describes both VMs after implementation; it is not an
   expected-failure marker for current Lagertha behavior.
4. Encode expected semantics in assertions or exit behavior so the red result
   proves a behavioral gap rather than only a missing snapshot.
5. Run the focused integration test and confirm Lagertha fails for the intended
   reason, not because of compilation, metadata, discovery, or unrelated setup.
6. If the harness stops after Lagertha's category check, run the compiled class
   manually on the reference JDK with `java -ea` to verify the oracle behavior.
7. Use `javap -c -p` when emitted instruction choice or symbolic ownership is
   part of the claim.

Stop there for the red phase. Do not accept a snapshot, update generated
reports, or expect feature-tracking validation to pass before an approved
snapshot exists.

After implementation reaches the intended behavior:

1. Rerun the focused integration test on Lagertha and the reference JDK.
2. Review and accept the combined snapshot semantically.
3. Rerun the focused test against the approved snapshot.
4. Run feature-tracking validation.
5. Run broader checks required by the affected subsystem.

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
5. Run the focused integration test.
6. Review and accept the new snapshot semantically when intended behavior works
   on both VMs.
7. Rerun the focused test, then run feature-tracking validation.
8. Run broader tests required by the affected subsystem.

## Feature Tracking

Validate the registry, fixture metadata, and direct snapshot mappings with:

```bash
cargo run -p feature-tracking -- features
```

Validate reverse mappings, orphan and pending snapshots, and entry identity
collisions with:

```bash
cargo run -p feature-tracking -- inventory
```

Generate development reports with the `unreleased` label:

```bash
cargo run -p feature-tracking -- coverage unreleased \
  --output docs/features/TEST_COVERAGE.md
cargo run -p feature-tracking -- feature-report unreleased \
  --output docs/features/README.md
```

Report writes are atomic. Omitting `--output` prints either report to stdout.
See [`FEATURE_TRACKING.md`](FEATURE_TRACKING.md) for schemas and permanent
coverage rules.
