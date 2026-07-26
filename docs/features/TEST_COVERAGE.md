# Integration Test Coverage

Generated for Lagertha `unreleased`.

Coverage means passing integration snapshot evidence for a feature; it does not prove every criterion.

## Summary

| Metric | Count |
|---|---:|
| Features | 2 |
| Snapshot tests | 3 |
| Success tests | 1 |
| Error tests | 2 |

## Feature Coverage

### `class-format.interface-flags`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Error | [`InterfaceFlagWithoutAbstractErrMain.rns`](../../vm/tests/testdata/rns/class_format/InterfaceFlagWithoutAbstractErrMain.rns) | Rejects an interface missing ACC_ABSTRACT. |

### `execution.integer.arithmetic`

Implementation: **Implemented**  
Snapshot tests: 2

| Category | Test | Description |
|---|---|---|
| Success | [`ArithmeticOkMain.java`](../../vm/tests/testdata/java/primitives/ints/arithmetic/ArithmeticOkMain.java) | Verifies comprehensive source-level integer operations and edge cases. |
| Error | [`DivisionByZeroErrMain.java`](../../vm/tests/testdata/java/primitives/ints/errors/DivisionByZeroErrMain.java) | Verifies that integer division by zero throws ArithmeticException. |

## Features Without Integration Tests

None.

## Implemented Features Without Integration Tests

None.

## Partial Features With Regression Tests

None.

