# Integration Test Coverage

Generated for Lagertha `unreleased`.

Coverage means passing integration snapshot evidence for a feature; it does not prove every criterion.

## Summary

| Metric | Count |
|---|---:|
| Features | 40 |
| Snapshot tests | 59 |
| Success tests | 51 |
| Error tests | 8 |

## Feature Coverage

### `bootstrap.main-method`

Implementation: **Partial**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`HelloWorldTest.java`](../../vm/tests/testdata/bootstrap/main/HelloWorldTest.java) | Verifies startup invokes a conventional public static main method and completes successfully. |

### `class-format.interface-flags`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Error | [`InterfaceFlagWithoutAbstractTest.rns`](../../vm/tests/testdata/class_format/InterfaceFlagWithoutAbstractTest.rns) | Rejects an interface missing ACC_ABSTRACT. |

### `class-loading.initialization`

Implementation: **Partial**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`ClassInitializationTest.java`](../../vm/tests/testdata/class_loading/initialization/ClassInitializationTest.java) | Verifies active initialization triggers, superclass and textual order, interfaces, and one-time execution. |

### `class-loading.preparation`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`StaticFieldPreparationTest.java`](../../vm/tests/testdata/class_loading/preparation/StaticFieldPreparationTest.java) | Verifies class and interface static fields expose prepared defaults before explicit initializers. |

### `exceptions.handler-selection`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`HandlerSelectionTest.java`](../../vm/tests/testdata/exceptions/handling/HandlerSelectionTest.java) | Verifies exact, superclass, ordered, catch-all, nested, mismatched, and normal exception-table paths. |

### `exceptions.propagation`

Implementation: **Partial**  
Snapshot tests: 2

| Category | Test | Description |
|---|---|---|
| Success | [`FinallyUnwindingTest.java`](../../vm/tests/testdata/exceptions/propagation/FinallyUnwindingTest.java) | Verifies finally paths execute after normal and abrupt completion and rethrow the original exception. |
| Success | [`FrameUnwindingTest.java`](../../vm/tests/testdata/exceptions/propagation/FrameUnwindingTest.java) | Verifies multi-frame, constructor, and replacement-exception propagation skips abruptly completed code. |

### `exceptions.stack-traces`

Implementation: **Partial**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`StackTraceTest.java`](../../vm/tests/testdata/exceptions/stack_traces/StackTraceTest.java) | Verifies explicit stack-trace output contains the throw site and ordered Java callers. |

### `exceptions.throwing`

Implementation: **Partial**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`ThrowingTest.java`](../../vm/tests/testdata/exceptions/throwing/ThrowingTest.java) | Verifies explicit throws preserve identity and message, while throwing null produces NullPointerException. |

### `exceptions.uncaught-reporting`

Implementation: **Partial**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Error | [`UncaughtExceptionTest.java`](../../vm/tests/testdata/exceptions/reporting/UncaughtExceptionTest.java) | Verifies an exception escaping main reports its type, message, and caller frames before failure. |

### `execution.arrays.access-exceptions`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`ArrayAccessExceptionTest.java`](../../vm/tests/testdata/execution/arrays/errors/ArrayAccessExceptionTest.java) | Verifies null and out-of-bounds array loads and stores leave valid components unchanged. |

### `execution.arrays.allocation-exceptions`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`NegativeArraySizeTest.java`](../../vm/tests/testdata/execution/arrays/errors/NegativeArraySizeTest.java) | Verifies negative primitive and reference array lengths throw NegativeArraySizeException. |

### `execution.arrays.default-values`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`ArrayDefaultValueTest.java`](../../vm/tests/testdata/execution/arrays/defaults/ArrayDefaultValueTest.java) | Verifies default values for primitive, reference, and nested array components. |

### `execution.arrays.length`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`ArrayLengthTest.java`](../../vm/tests/testdata/execution/arrays/length/ArrayLengthTest.java) | Verifies primitive, reference, nested, empty, and null array length behavior. |

### `execution.arrays.multidimensional`

Implementation: **Implemented**  
Snapshot tests: 2

| Category | Test | Description |
|---|---|---|
| Success | [`JaggedArrayTest.java`](../../vm/tests/testdata/execution/arrays/multidimensional/JaggedArrayTest.java) | Verifies independently allocated, empty, reassigned, aliased, and partial nested rows. |
| Success | [`RectangularArrayTest.java`](../../vm/tests/testdata/execution/arrays/multidimensional/RectangularArrayTest.java) | Verifies rectangular primitive and reference arrays across multiple ranks. |

### `execution.arrays.primitive-elements`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`PrimitiveArrayTest.java`](../../vm/tests/testdata/execution/arrays/elements/PrimitiveArrayTest.java) | Verifies loads and stores for every primitive array component kind. |

### `execution.arrays.reference-elements`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`ReferenceArrayTest.java`](../../vm/tests/testdata/execution/arrays/elements/ReferenceArrayTest.java) | Verifies compatible references, null, identity, aliases, and array-reference reassignment. |

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
Snapshot tests: 2

| Category | Test | Description |
|---|---|---|
| Success | [`FieldAccessTest.java`](../../vm/tests/testdata/execution/objects/fields/FieldAccessTest.java) | Verifies primitive and reference fields, receiver storage, shared statics, inheritance, hiding, and null writes. |
| Error | [`NullFieldAccessTest.java`](../../vm/tests/testdata/execution/objects/fields/NullFieldAccessTest.java) | Verifies an instance field read through a null receiver throws NullPointerException. |

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

### `execution.objects.instance-default-values`

Implementation: **Implemented**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`InstanceFieldDefaultValueTest.java`](../../vm/tests/testdata/execution/objects/fields/InstanceFieldDefaultValueTest.java) | Verifies primitive, reference, inherited, and per-object instance field defaults. |

### `execution.references.casting`

Implementation: **Partial**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`NullCastTest.java`](../../vm/tests/testdata/execution/references/casting/NullCastTest.java) | Verifies checkcast accepts null for class, interface, and array targets. |

### `natives.binding`

Implementation: **Partial**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Error | [`MissingNativeTest.java`](../../vm/tests/testdata/natives/binding/MissingNativeTest.java) | Verifies an unbound native method throws UnsatisfiedLinkError with a native stack frame. |

### `natives.class.assertion-status`

Implementation: **Partial**  
Snapshot tests: 1

| Category | Test | Description |
|---|---|---|
| Success | [`AssertionStatusTest.java`](../../vm/tests/testdata/natives/class/AssertionStatusTest.java) | Verifies assertions are enabled, true conditions complete, and false details construct AssertionError. |

### `natives.object.get-class`

Implementation: **Partial**  
Snapshot tests: 2

| Category | Test | Description |
|---|---|---|
| Success | [`ObjectGetClassTest.java`](../../vm/tests/testdata/natives/object/getclass/ObjectGetClassTest.java) | Verifies getClass returns and reuses the implementation mirror through concrete, Object, and interface references. |
| Success | [`ReferenceArrayGetClassTest.java`](../../vm/tests/testdata/natives/object/getclass/ReferenceArrayGetClassTest.java) | Verifies one-dimensional reference arrays reuse one mirror across getClass calls, literals, and instances. |

### `natives.system.arraycopy`

Implementation: **Partial**  
Snapshot tests: 3

| Category | Test | Description |
|---|---|---|
| Success | [`CopySemanticsTest.java`](../../vm/tests/testdata/natives/system/arraycopy/CopySemanticsTest.java) | Verifies full, partial, zero-length, overlapping, primitive, and reference array copies. |
| Success | [`NonArrayArgumentTest.java`](../../vm/tests/testdata/natives/system/arraycopy/NonArrayArgumentTest.java) | Verifies non-array source and destination arguments throw ArrayStoreException at zero length. |
| Success | [`NullArgumentTest.java`](../../vm/tests/testdata/natives/system/arraycopy/NullArgumentTest.java) | Verifies null source and destination arguments throw NullPointerException without copying. |

## Features Without Integration Tests

None.

## Implemented Features Without Integration Tests

None.

## Partial Features With Regression Tests

- `bootstrap.main-method`
- `class-loading.initialization`
- `exceptions.propagation`
- `exceptions.stack-traces`
- `exceptions.throwing`
- `exceptions.uncaught-reporting`
- `execution.references.casting`
- `natives.binding`
- `natives.class.assertion-status`
- `natives.object.get-class`
- `natives.system.arraycopy`

