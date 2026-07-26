# Integration Test Coverage

Generated for Lagertha `unreleased`.

Coverage means passing integration snapshot evidence for a feature; it does not prove every criterion.

## Summary

| Metric | Count |
|---|---:|
| Features | 5 |
| Snapshot tests | 9 |
| Success tests | 7 |
| Error tests | 2 |

## Feature Coverage

### `class-format.interface-flags`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Error | [`InterfaceFlagWithoutAbstractTest.rns`](../../vm/tests/testdata/rns/class_format/InterfaceFlagWithoutAbstractTest.rns) | Rejects an interface missing ACC_ABSTRACT. |

### `execution.integer.arithmetic`

Implementation: **Implemented**  
Snapshot tests: 4

| Category | Test | Description |
|---|---|---|
| Error | [`DivisionByZeroTest.java`](../../vm/tests/testdata/execution/integer/arithmetic/DivisionByZeroTest.java) | Verifies that integer division by zero throws ArithmeticException. |
| Success | [`DivisionRemainderTest.java`](../../vm/tests/testdata/execution/integer/arithmetic/DivisionRemainderTest.java) | Verifies signed integer division and remainder semantics and edge cases. |
| Success | [`IncrementDecrementTest.java`](../../vm/tests/testdata/execution/integer/arithmetic/IncrementDecrementTest.java) | Verifies compound arithmetic and integer increment and decrement expressions. |
| Success | [`OverflowTest.java`](../../vm/tests/testdata/execution/integer/arithmetic/OverflowTest.java) | Verifies integer overflow wrapping for binary and unary arithmetic. |

### `execution.integer.bitwise`

Implementation: **Implemented**  
Snapshot tests: 2

| Category | Test | Description |
|---|---|---|
| Success | [`BitwiseTest.java`](../../vm/tests/testdata/execution/integer/bitwise/BitwiseTest.java) | Verifies integer complement, conjunction, disjunction, and exclusive-or. |
| Success | [`ShiftTest.java`](../../vm/tests/testdata/execution/integer/bitwise/ShiftTest.java) | Verifies integer shifts, signedness, and shift-distance masking. |

### `execution.integer.comparisons`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`ComparisonTest.java`](../../vm/tests/testdata/execution/integer/comparisons/ComparisonTest.java) | Verifies signed integer ordering, equality, and inequality. |

### `execution.integer.conversions`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`NarrowingConversionTest.java`](../../vm/tests/testdata/execution/integer/conversions/NarrowingConversionTest.java) | Verifies narrowing integers to byte, short, and char values. |

## Features Without Integration Tests

None.

## Implemented Features Without Integration Tests

None.

## Partial Features With Regression Tests

None.

