# Integration Test Coverage

Generated for Lagertha `unreleased`.

Coverage means passing integration snapshot evidence for a feature; it does not prove every criterion.

## Summary

| Metric | Count |
|---|---:|
| Features | 19 |
| Snapshot tests | 32 |
| Success tests | 27 |
| Error tests | 5 |

## Feature Coverage

### `class-format.interface-flags`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Error | [`InterfaceFlagWithoutAbstractTest.rns`](../../vm/tests/testdata/rns/class_format/InterfaceFlagWithoutAbstractTest.rns) | Rejects an interface missing ACC_ABSTRACT. |

### `execution.control-flow.conditional-branches`

Implementation: **Implemented**  
Snapshot tests: 2

| Category | Test | Description |
|---|---|---|
| Success | [`LoopBranchTest.java`](../../vm/tests/testdata/execution/control_flow/branches/LoopBranchTest.java) | Verifies conditional branches across while, do-while, for, and nested loops. |
| Success | [`BooleanBranchTest.java`](../../vm/tests/testdata/execution/control_flow/conditional/BooleanBranchTest.java) | Verifies compiled boolean branches, short-circuiting, and conditional selection. |

### `execution.control-flow.unconditional-branches`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`BreakContinueTest.java`](../../vm/tests/testdata/execution/control_flow/branches/BreakContinueTest.java) | Verifies break and continue transfers across simple, nested, and labeled regions. |

### `execution.fields.access`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`FieldAccessTest.java`](../../vm/tests/testdata/execution/objects/fields/FieldAccessTest.java) | Verifies instance, static, initialized, default, and inherited field access. |

### `execution.frames.local-variables`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`LocalVariableTest.java`](../../vm/tests/testdata/execution/frames/LocalVariableTest.java) | Verifies local slots for each JVM computational value kind and references. |

### `execution.frames.method-arguments`

Implementation: **Implemented**  
Snapshot tests: 2

| Category | Test | Description |
|---|---|---|
| Success | [`MethodArgumentTest.java`](../../vm/tests/testdata/execution/frames/MethodArgumentTest.java) | Verifies argument slot order, receivers, wide values, null, arrays, and value-copy semantics. |
| Success | [`VarargsArrayTest.java`](../../vm/tests/testdata/execution/frames/VarargsArrayTest.java) | Verifies empty, populated, explicit, and fixed-prefix array arguments. |

### `execution.frames.recursion`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`RecursiveFrameTest.java`](../../vm/tests/testdata/execution/frames/RecursiveFrameTest.java) | Verifies isolated parameters and pending results across branching recursive frames. |

### `execution.integer.arithmetic`

Implementation: **Implemented**  
Snapshot tests: 5

| Category | Test | Description |
|---|---|---|
| Error | [`DivisionByZeroTest.java`](../../vm/tests/testdata/execution/integer/arithmetic/DivisionByZeroTest.java) | Verifies that integer division by zero throws ArithmeticException. |
| Success | [`DivisionRemainderTest.java`](../../vm/tests/testdata/execution/integer/arithmetic/DivisionRemainderTest.java) | Verifies signed integer division and remainder semantics and edge cases. |
| Success | [`IncrementDecrementTest.java`](../../vm/tests/testdata/execution/integer/arithmetic/IncrementDecrementTest.java) | Verifies compound arithmetic and integer increment and decrement expressions. |
| Success | [`OverflowTest.java`](../../vm/tests/testdata/execution/integer/arithmetic/OverflowTest.java) | Verifies integer overflow wrapping for binary and unary arithmetic. |
| Error | [`RemainderByZeroTest.java`](../../vm/tests/testdata/execution/integer/arithmetic/RemainderByZeroTest.java) | Verifies that integer remainder by zero throws ArithmeticException. |

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
Snapshot tests: 2

| Category | Test | Description |
|---|---|---|
| Success | [`NarrowCompoundTest.java`](../../vm/tests/testdata/execution/integer/conversions/NarrowCompoundTest.java) | Verifies narrowing after compound assignment and increment or decrement. |
| Success | [`NarrowingConversionTest.java`](../../vm/tests/testdata/execution/integer/conversions/NarrowingConversionTest.java) | Verifies narrowing integers to byte, short, and char values. |

### `execution.invocation.interface`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`InterfaceInvocationTest.java`](../../vm/tests/testdata/execution/invocation/InterfaceInvocationTest.java) | Verifies implementing-class and default-method dispatch through an interface. |

### `execution.invocation.special`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`SpecialInvocationTest.java`](../../vm/tests/testdata/execution/invocation/SpecialInvocationTest.java) | Verifies that a super call selects the superclass method implementation. |

### `execution.invocation.static`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`StaticInvocationTest.java`](../../vm/tests/testdata/execution/invocation/StaticInvocationTest.java) | Verifies static method invocation on classes and interfaces. |

### `execution.invocation.virtual`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`VirtualDispatchTest.java`](../../vm/tests/testdata/execution/invocation/VirtualDispatchTest.java) | Verifies runtime override selection and abstract superclass dispatch. |

### `execution.long.arithmetic`

Implementation: **Implemented**  
Snapshot tests: 5

| Category | Test | Description |
|---|---|---|
| Error | [`DivisionByZeroTest.java`](../../vm/tests/testdata/execution/long/arithmetic/DivisionByZeroTest.java) | Verifies that long division by zero throws ArithmeticException. |
| Success | [`DivisionRemainderTest.java`](../../vm/tests/testdata/execution/long/arithmetic/DivisionRemainderTest.java) | Verifies signed long division and remainder semantics and edge cases. |
| Success | [`IncrementDecrementTest.java`](../../vm/tests/testdata/execution/long/arithmetic/IncrementDecrementTest.java) | Verifies compound arithmetic and long increment and decrement expressions. |
| Success | [`OverflowTest.java`](../../vm/tests/testdata/execution/long/arithmetic/OverflowTest.java) | Verifies long overflow wrapping for binary and unary arithmetic. |
| Error | [`RemainderByZeroTest.java`](../../vm/tests/testdata/execution/long/arithmetic/RemainderByZeroTest.java) | Verifies that long remainder by zero throws ArithmeticException. |

### `execution.long.bitwise`

Implementation: **Implemented**  
Snapshot tests: 2

| Category | Test | Description |
|---|---|---|
| Success | [`BitwiseTest.java`](../../vm/tests/testdata/execution/long/bitwise/BitwiseTest.java) | Verifies long complement, conjunction, disjunction, and exclusive-or. |
| Success | [`ShiftTest.java`](../../vm/tests/testdata/execution/long/bitwise/ShiftTest.java) | Verifies long shifts, signedness, and shift-distance masking. |

### `execution.long.comparisons`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`ComparisonTest.java`](../../vm/tests/testdata/execution/long/comparisons/ComparisonTest.java) | Verifies signed long ordering, equality, and inequality. |

### `execution.long.conversions`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`LongConversionTest.java`](../../vm/tests/testdata/execution/long/conversions/LongConversionTest.java) | Verifies widening integers to long and narrowing long values to integers. |

## Features Without Integration Tests

None.

## Implemented Features Without Integration Tests

None.

## Partial Features With Regression Tests

None.

