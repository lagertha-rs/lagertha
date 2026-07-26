# Feature and Integration Test Tracking

This document defines how Lagertha tracks implemented features and integration
test coverage. GitHub Issues track temporary work. Feature files and snapshot
fixtures track permanent product state.

## Goals

- Record implemented, partial, blocked, deferred, and missing features.
- Show which features have integration snapshot coverage.
- Keep feature definitions small and independently editable.
- Keep Java and RNS fixtures usable directly by their normal tools.
- Generate readable feature and test coverage reports.
- Avoid manually maintained status tables and test counts.

## Feature Registry

Each feature is one YAML file. Directories define categories and subcategories.

```text
features/
├── bootstrap/
│   ├── vm-startup.yaml
│   ├── main-method.yaml
│   └── shutdown.yaml
├── class-loading/
│   ├── jimage.yaml
│   ├── classpath.yaml
│   ├── jar.yaml
│   └── custom-loader.yaml
├── opcodes/
│   ├── constants/
│   │   ├── aconst-null.yaml
│   │   ├── iconst-0.yaml
│   │   └── ldc.yaml
│   ├── arithmetic/
│   │   ├── iadd.yaml
│   │   ├── isub.yaml
│   │   ├── imul.yaml
│   │   └── idiv.yaml
│   ├── control-flow/
│   │   ├── goto.yaml
│   │   ├── tableswitch.yaml
│   │   └── lookupswitch.yaml
│   └── references/
│       ├── new.yaml
│       ├── checkcast.yaml
│       └── instanceof.yaml
├── arrays/
│   ├── primitive.yaml
│   ├── reference.yaml
│   ├── multidimensional.yaml
│   └── array-store-check.yaml
├── exceptions/
│   ├── throwing.yaml
│   ├── handler-selection.yaml
│   ├── propagation.yaml
│   └── stack-traces.yaml
└── natives/
    └── system/
        ├── arraycopy.yaml
        └── identity-hash-code.yaml
```

The feature ID is derived from its path. It is not repeated inside the file.

```text
features/opcodes/arithmetic/iadd.yaml
-> opcodes.arithmetic.iadd
```

One file per opcode is intentional. Small files avoid giant inventories, reduce
merge conflicts, and let an agent update one feature without rewriting an entire
category.

## Feature Schema

An implemented feature:

```yaml
name: Integer addition
description: Adds two integer values using Java wrapping semantics.
status: implemented
spec: "https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.iadd"

criteria:
  - Decodes the iadd opcode.
  - Pops two integer values.
  - Pushes their sum.
  - Wraps on signed overflow.
```

A partial feature:

```yaml
name: Reference cast
description: Checks whether an object can be cast to a target type.
status: partial
spec: "https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.checkcast"

criteria:
  - Null always succeeds.
  - Compatible class casts succeed.
  - Compatible interface casts succeed.
  - Compatible array casts succeed.
  - Invalid casts throw ClassCastException.

limitations:
  - Current implementation accepts every non-null cast.
```

A blocked feature:

```yaml
name: Dynamic invocation
description: Resolves and executes dynamic call sites.
status: blocked
spec: "https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokedynamic"

criteria:
  - Resolves the bootstrap method.
  - Creates a call site.
  - Caches the resolved call site.
  - Invokes the target method handle.

blocked_by:
  - method-handles.resolution
  - method-handles.invocation
```

A deferred feature:

```yaml
name: Breakpoint opcode
description: Supports the reserved breakpoint opcode.
status: deferred
spec: "https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.2"

criteria:
  - Recognizes the opcode in debugger-managed bytecode.

reason: Debugger opcode support is outside the current execution milestone.
```

### Status Values

| Status | Meaning | Additional data |
|---|---|---|
| `missing` | No meaningful implementation exists | None |
| `partial` | Some declared behavior works | `limitations` |
| `implemented` | All declared behavior works | None |
| `blocked` | Progress depends on another feature | `blocked_by` |
| `deferred` | Intentionally outside the current horizon | `reason` |

Every item in `criteria` describes part of the feature scope. Criteria do not
have separate IDs or required flags.

### Specification References

The Java SE 25 specifications are the source of truth:

- [Java Virtual Machine Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/index.html)
- [Java Language Specification](https://docs.oracle.com/javase/specs/jls/se25/html/index.html)

When present, `spec` must be a direct section link into one of these SE 25
specifications, including its fragment. Plain labels such as `JVMS 6.5.iadd` are
not valid registry references.

Before adding or changing a specification link, verify its page with
`curl --fail --silent --show-error --location`. URL fragments are not sent to
the server, so also inspect the returned HTML and verify the fragment ID exists.
Do not add unverified links. Registry validation enforces URL shape without
performing network access.

Implementation state and test coverage are independent. An implemented feature
may have no tests, while a partial feature may have extensive regression tests.

## Integration Fixtures

Java and RNS fixtures remain normal source files. Source must not be embedded in
YAML or JSON because that would degrade syntax highlighting, language-server
support, formatting, direct execution, compiler diagnostics, and source diffs.

The fixture tree groups sources by capability rather than language:

```text
vm/tests/testdata/
├── primitives/
├── arrays/
├── exceptions/
├── class-format/
└── opcodes/
```

The build selects the compiler by extension:

- `.java` uses `javac`.
- `.rns` uses `rnsc`.
- `compiled/` remains generated output.

## Test Metadata

Every integration test main source starts with exactly three metadata comments.

Java example:

```java
// @test feature = "opcodes.arithmetic.iadd"
// @test description = "Verifies integer addition and signed overflow wrapping."
// @test category = "success"

package opcodes.arithmetic.iadd;
```

RNS example:

```rns
; @test feature = "class-format.interface-flags"
; @test description = "Rejects an interface missing ACC_ABSTRACT."
; @test category = "error"

.class interface InterfaceFlagWithoutAbstractTest
```

| Field | Meaning |
|---|---|
| `feature` | One feature intentionally tested |
| `description` | Exact behavior asserted by the test |
| `category` | Expected test outcome |

The source path provides test identity. The filename or class name provides its
display name. No separate test ID or test name is needed.

### Test Categories

Start with two categories:

- `success`: Lagertha and the reference JVM should exit successfully.
- `error`: Lagertha and the reference JVM should reject or fail.

Additional categories should only be introduced when they change harness
behavior. A future `known-divergence` category may represent intentionally
different Lagertha and reference JVM behavior.

## One Feature Per Test

Each integration fixture has exactly one primary feature. Tests should only
claim behavior they intentionally assert, not every incidental opcode emitted by
`javac`.

For example, the existing comprehensive integer arithmetic fixture belongs to a
broad feature:

```text
execution.integer.arithmetic
```

It should not automatically count as focused coverage for every arithmetic
opcode. Exact opcode coverage should use focused RNS fixtures, for example:

```text
vm/tests/testdata/opcodes/arithmetic/iadd/OverflowTest.rns
```

This distinction keeps coverage claims honest:

- Java fixtures test broad source-level behavior produced by `javac`.
- RNS fixtures test exact instructions and malformed bytecode behavior.

## Generated Reports

Two reports are generated from feature YAML files, fixture metadata, and
approved snapshots.

### Feature Report

`docs/features/README.md` answers: "What is implemented?"

It contains:

- Features grouped by category.
- Current implementation status.
- Feature descriptions and criteria.
- Known limitations, blockers, and deferred reasons.
- Specification references.
- Integration snapshot count for each feature.

Example summary:

| Feature | Status | Tests | Description |
|---|---|---:|---|
| `opcodes.arithmetic.iadd` | Implemented | 2 | Integer addition |
| `opcodes.arithmetic.isub` | Implemented | 0 | Integer subtraction |
| `opcodes.references.checkcast` | Partial | 1 | Reference casts |
| `opcodes.invocation.invokedynamic` | Blocked | 0 | Dynamic invocation |

### Test Coverage Report

`docs/features/TEST_COVERAGE.md` answers: "What integration evidence exists?"

It contains:

- Every feature and its linked integration tests.
- Test source links, descriptions, and categories.
- Features without integration tests.
- Implemented features without integration tests.
- Partial features with regression tests.
- Test totals grouped by category.
- Invalid feature references and orphan test artifacts.

Example detail:

```markdown
## opcodes.arithmetic.iadd

Implementation: Implemented
Snapshot tests: 2

| Category | Test | Description |
|---|---|---|
| Success | `opcodes/arithmetic/iadd/BasicTest.rns` | Adds two positive integers |
| Success | `opcodes/arithmetic/iadd/OverflowTest.rns` | Wraps signed overflow |
```

The report must not calculate criterion coverage percentages. A linked passing
snapshot proves integration evidence for its feature, not exhaustive proof of
every criterion.

## Validation

Validate the feature registry and fixture metadata locally:

```bash
cargo run -p feature-tracking -- features
cargo run -p feature-tracking -- inventory
```

Preview either deterministic report for a release version:

```bash
cargo run -p feature-tracking -- coverage <version>
cargo run -p feature-tracking -- feature-report <version>
```

Pass `--output <path>` to write a report atomically instead of printing it.

The workspace test suite also validates the repository registry and
fixture-to-snapshot mappings.

CI validates the registry and integration fixtures:

- Every YAML path produces a unique feature ID.
- Every feature defines `name`, `description`, `status`, and `criteria`.
- Every partial feature defines `limitations`.
- Every blocked feature defines `blocked_by`.
- Every deferred feature defines `reason`.
- Every integration main fixture starts with complete metadata.
- Only `*Test.java` and `*Test.rns` sources may contain entry metadata.
- Every test references an existing feature.
- Every test uses a supported category.
- Every test source maps to exactly one approved snapshot.
- Every snapshot maps to exactly one test source.
- Snapshot identities cannot collide.
- Test exit behavior matches its category.

Only passing integration snapshot tests count toward test coverage. Unit tests,
source-code handler detection, and incidental bytecode presence do not count.

## Release Workflow

For a release, after the full integration suite passes:

1. Validates feature files and fixture metadata.
2. Generates `docs/features/README.md`.
3. Generates `docs/features/TEST_COVERAGE.md`.
4. Add both generated reports to the release pull request.

Generated reports describe the latest released version. Their header should
include the release version and explain that coverage means passing integration
snapshot evidence.

## Update Rules

A behavioral change should update all affected sources of truth in the same pull
request:

1. Update the relevant feature YAML status, criteria, or limitations.
2. Add or update a Java or RNS integration fixture.
3. Add or update the approved snapshot.

GitHub Issues remain responsible for planned work, prioritization, and progress.
Feature files describe permanent capability state after that work lands.
