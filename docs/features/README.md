# Lagertha Features

Generated for Lagertha `unreleased`.

Feature status describes declared JVM behavior. Test counts mean passing integration snapshot evidence, not exhaustive criterion coverage.

## Summary

| Status | Features |
|---|---:|
| Implemented | 29 |
| Partial | 9 |
| Missing | 0 |
| Blocked | 0 |
| Deferred | 0 |

## Feature Index

### class-format

| Feature | Status | Tests | Description |
|---|---|---:|---|
| `class-format.interface-flags` | Implemented | 1 | Validates access flag combinations for interface class files. |

### class-loading

| Feature | Status | Tests | Description |
|---|---|---:|---|
| `class-loading.initialization` | Partial | 1 | Executes class and interface initialization methods after preparation. |
| `class-loading.preparation` | Implemented | 1 | Creates static field storage and assigns JVM default values before initialization. |

### exceptions

| Feature | Status | Tests | Description |
|---|---|---:|---|
| `exceptions.handler-selection` | Implemented | 1 | Searches exception tables in order for a handler compatible with the thrown object. |
| `exceptions.propagation` | Partial | 2 | Abruptly completes frames until a caller provides a matching handler. |
| `exceptions.stack-traces` | Partial | 1 | Captures and renders Java call frames associated with a thrown exception. |
| `exceptions.throwing` | Partial | 1 | Throws exception references with the athrow instruction. |
| `exceptions.uncaught-reporting` | Partial | 1 | Reports an uncaught main-thread exception and terminates unsuccessfully. |

### execution

| Feature | Status | Tests | Description |
|---|---|---:|---|
| `execution.arrays.access-exceptions` | Implemented | 1 | Rejects null array references and indices outside array bounds. |
| `execution.arrays.allocation-exceptions` | Implemented | 1 | Rejects negative one-dimensional primitive and reference array lengths. |
| `execution.arrays.default-values` | Implemented | 1 | Initializes newly allocated array components with JVM default values. |
| `execution.arrays.length` | Implemented | 1 | Returns the fixed component count of an array. |
| `execution.arrays.multidimensional` | Implemented | 2 | Allocates and accesses rectangular and nested array structures. |
| `execution.arrays.primitive-elements` | Implemented | 1 | Allocates primitive arrays and loads and stores their component values. |
| `execution.arrays.reference-elements` | Implemented | 1 | Allocates reference arrays and loads and stores compatible references. |
| `execution.control-flow.conditional-branches` | Implemented | 2 | Executes value-dependent conditional control flow. |
| `execution.control-flow.unconditional-branches` | Implemented | 1 | Executes forward and backward unconditional control transfers. |
| `execution.fields.access` | Implemented | 2 | Resolves and accesses instance and static fields. |
| `execution.frames.local-variables` | Implemented | 1 | Stores and loads JVM computational values in local variable slots. |
| `execution.frames.method-arguments` | Implemented | 2 | Transfers receiver and argument values into callee local variable slots. |
| `execution.frames.recursion` | Implemented | 1 | Creates isolated frames for recursive method invocation. |
| `execution.integer.arithmetic` | Implemented | 5 | Executes integer arithmetic with JVM-defined overflow and division semantics. |
| `execution.integer.bitwise` | Implemented | 2 | Executes integer shifts and bitwise operations with JVM-defined semantics. |
| `execution.integer.comparisons` | Implemented | 1 | Evaluates signed integer comparison expressions. |
| `execution.integer.conversions` | Implemented | 2 | Narrows integer values to byte, short, and char values. |
| `execution.invocation.interface` | Implemented | 1 | Resolves and invokes methods through interface references. |
| `execution.invocation.special` | Implemented | 1 | Invokes a selected superclass implementation without virtual dispatch. |
| `execution.invocation.static` | Implemented | 1 | Resolves and invokes class and interface static methods. |
| `execution.invocation.virtual` | Implemented | 1 | Selects and invokes instance methods through class hierarchies. |
| `execution.long.arithmetic` | Implemented | 5 | Executes long arithmetic with JVM-defined overflow and division semantics. |
| `execution.long.bitwise` | Implemented | 2 | Executes long shifts and bitwise operations with JVM-defined semantics. |
| `execution.long.comparisons` | Implemented | 1 | Evaluates signed long comparison expressions. |
| `execution.long.conversions` | Implemented | 1 | Converts between integer and long computational values. |
| `execution.objects.instance-default-values` | Implemented | 1 | Initializes fields in newly allocated objects with JVM default values. |
| `execution.references.casting` | Partial | 1 | Checks whether a reference can be cast to a target reference type. |

### natives

| Feature | Status | Tests | Description |
|---|---|---:|---|
| `natives.binding` | Partial | 1 | Binds native methods to registered VM implementations. |
| `natives.class.assertion-status` | Partial | 1 | Supplies assertion enablement used by compiled assert statements. |
| `natives.system.arraycopy` | Partial | 3 | Copies array subsequences with Java type, bounds, overlap, and exception semantics. |

## Feature Details

### `class-format.interface-flags`

Validates access flag combinations for interface class files.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.1>  
Snapshot tests: 1

#### Criteria

- Requires ACC_ABSTRACT when ACC_INTERFACE is set.
- Rejects ACC_FINAL, ACC_SUPER, ACC_ENUM, and ACC_MODULE on interfaces.

### `class-loading.initialization`

Executes class and interface initialization methods after preparation.

Status: **Partial**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.5>  
Snapshot tests: 1

#### Criteria

- Triggers initialization on active static field access and object creation.
- Initializes superclasses before subclasses.
- Executes initialization expressions and blocks in class-file order exactly once.
- Initializes an interface when its non-constant static field is actively used.
- Initializes required superinterfaces that declare default methods in specification order.
- Coordinates concurrent initialization and records initialization failures.

#### Limitations

- Superinterface selection and ordering do not yet implement the default-method rules.
- Concurrent initialization does not track the initializing thread or wait for completion.
- Failed initialization does not mark the class erroneous or produce the required later errors.
- ConstantValue field attributes are not applied during initialization.

### `class-loading.preparation`

Creates static field storage and assigns JVM default values before initialization.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.4.2>  
Snapshot tests: 1

#### Criteria

- Creates static storage for classes and interfaces.
- Assigns false and numeric zero to primitive static fields before explicit initializers run.
- Assigns null to reference static fields before explicit initializers run.
- Makes prepared values observable to earlier expressions in the initialization method.

### `exceptions.handler-selection`

Searches exception tables in order for a handler compatible with the thrown object.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.10>  
Snapshot tests: 1

#### Criteria

- Selects handlers whose protected half-open range contains the throwing instruction.
- Chooses the first table entry whose catch type matches the thrown class or a superclass.
- Supports catch-all handlers and nested protected regions.
- Skips incompatible handlers and bypasses all handlers on normal completion.

### `exceptions.propagation`

Abruptly completes frames until a caller provides a matching handler.

Status: **Partial**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6.5>  
Snapshot tests: 2

#### Criteria

- Discards intervening Java frames while preserving the thrown exception object.
- Propagates exceptions from constructors and exception handlers.
- Skips instructions following an abruptly completed invocation.
- Executes compiler-generated catch-all and rethrow paths for finally blocks.
- Releases monitors held by synchronized methods during abrupt completion.

#### Limitations

- Synchronized-method monitor acquisition and release are not implemented.
- Internal failures during handler lookup can replace Java propagation with a VM error.

### `exceptions.stack-traces`

Captures and renders Java call frames associated with a thrown exception.

Status: **Partial**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.10>  
Snapshot tests: 1

#### Criteria

- Captures the throwing method followed by caller frames.
- Renders method names, source files, and source line numbers.
- Represents native methods as native frames.
- Supports explicit Throwable.printStackTrace output.

#### Limitations

- Causes, suppressed exceptions, common-frame elision, modules, loaders, and unknown-source frames lack integration evidence.
- Stack-trace native implementations use an incomplete custom model.

### `exceptions.throwing`

Throws exception references with the athrow instruction.

Status: **Partial**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.9>  
Snapshot tests: 1

#### Criteria

- Throws a non-null exception object and preserves its identity.
- Throws NullPointerException when the athrow operand is null.
- Supplies the thrown reference to a matching exception handler.
- Clears the operand stack before entering a matching handler.

#### Limitations

- Handler entry pushes the exception onto the existing operand stack instead of clearing it first.
- Abrupt synchronized-method completion does not release the method monitor.

### `exceptions.uncaught-reporting`

Reports an uncaught main-thread exception and terminates unsuccessfully.

Status: **Partial**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.7>  
Snapshot tests: 1

#### Criteria

- Terminates the VM unsuccessfully when an exception escapes the main method.
- Reports the thread name, exception type, message, and Java frames.
- Dispatches the exception through the main thread group.

#### Limitations

- Only main-thread reporting has integration evidence.
- Custom handlers, secondary threads, and failures during reporting are not supported or tested.

### `execution.arrays.access-exceptions`

Rejects null array references and indices outside array bounds.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.5>  
Snapshot tests: 1

#### Criteria

- Throws ArrayIndexOutOfBoundsException for negative and length-equal load indices.
- Throws ArrayIndexOutOfBoundsException for negative and length-equal store indices.
- Throws NullPointerException for loads and stores through null array references.
- Throws NullPointerException for access through a null nested row.
- Leaves valid components unchanged after a failed store.

### `execution.arrays.allocation-exceptions`

Rejects negative one-dimensional primitive and reference array lengths.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.5>  
Snapshot tests: 1

#### Criteria

- Throws NegativeArraySizeException for a negative primitive array length.
- Throws NegativeArraySizeException for a negative reference array length.
- Performs no array allocation when the requested length is negative.

### `execution.arrays.default-values`

Initializes newly allocated array components with JVM default values.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.5>  
Snapshot tests: 1

#### Criteria

- Initializes boolean components to false and numeric components to zero.
- Initializes reference components to null.
- Initializes each newly allocated nested row independently.
- Leaves unallocated nested rows null.

### `execution.arrays.length`

Returns the fixed component count of an array.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.arraylength>  
Snapshot tests: 1

#### Criteria

- Returns zero for an empty array.
- Returns the allocated length for primitive and reference arrays.
- Returns each independently allocated nested row length.
- Throws NullPointerException for a null array reference.

### `execution.arrays.multidimensional`

Allocates and accesses rectangular and nested array structures.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.multianewarray>  
Snapshot tests: 2

#### Criteria

- Allocates all requested dimensions of rectangular primitive and reference arrays.
- Loads and stores values through nested array references.
- Supports partially allocated dimensions whose components start null.
- Supports independently sized, empty, reassigned, and aliased rows.

### `execution.arrays.primitive-elements`

Allocates primitive arrays and loads and stores their component values.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.5>  
Snapshot tests: 1

#### Criteria

- Allocates arrays for every primitive component type.
- Loads and stores boolean, byte, char, short, int, long, float, and double components.
- Preserves signed byte and short values and unsigned char values.
- Normalizes boolean array values to true or false.

### `execution.arrays.reference-elements`

Allocates reference arrays and loads and stores compatible references.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.5>  
Snapshot tests: 1

#### Criteria

- Allocates arrays with object and narrower reference component types.
- Loads and stores compatible object references and null.
- Preserves object identity and aliases through array components.
- Preserves array identity when array references are copied and reassigned.

### `execution.control-flow.conditional-branches`

Executes value-dependent conditional control flow.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.7>  
Snapshot tests: 2

#### Criteria

- Selects control flow from integer zero and nonzero conditions.
- Preserves short-circuit evaluation of compiled conditional expressions.
- Selects the correct compiled conditional-expression branch.
- Repeats and exits compiled loop control flow.

### `execution.control-flow.unconditional-branches`

Executes forward and backward unconditional control transfers.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.goto>  
Snapshot tests: 1

#### Criteria

- Transfers execution forward without fallthrough.
- Transfers execution backward to continue repeated control flow.
- Exits nested compiled control-flow regions at the selected target.

### `execution.fields.access`

Resolves and accesses instance and static fields.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.4.3.2>  
Snapshot tests: 2

#### Criteria

- Reads and writes primitive and reference instance fields.
- Reads and writes primitive and reference static fields.
- Keeps instance storage independent and static storage shared across objects.
- Resolves inherited fields and keeps hidden same-name fields distinct by declaring class.
- Throws NullPointerException for instance field reads and writes through a null receiver.

### `execution.frames.local-variables`

Stores and loads JVM computational values in local variable slots.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6.1>  
Snapshot tests: 1

#### Criteria

- Stores and loads int, float, reference, long, and double computational values.
- Preserves category-2 long and double values across two local slots.
- Stores null, object, and array references in reference local slots.

### `execution.frames.method-arguments`

Transfers receiver and argument values into callee local variable slots.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6.1>  
Snapshot tests: 2

#### Criteria

- Preserves static argument order across category-1 and category-2 values.
- Places an instance receiver before explicit method arguments.
- Places a constructor receiver before explicit constructor arguments.
- Passes primitive and reference values by value.
- Passes null and array references.

### `execution.frames.recursion`

Creates isolated frames for recursive method invocation.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6>  
Snapshot tests: 1

#### Criteria

- Creates a fresh parameter local for each recursive invocation.
- Preserves pending return values across branching recursive calls.
- Returns composed results through nested frames.

### `execution.integer.arithmetic`

Executes integer arithmetic with JVM-defined overflow and division semantics.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.3>  
Snapshot tests: 5

#### Criteria

- Wraps addition, subtraction, multiplication, and negation on overflow.
- Implements signed division and remainder, including the minimum-value edge case.
- Throws ArithmeticException when an integer divisor is zero.
- Implements compound arithmetic assignments and increment and decrement expressions.

### `execution.integer.bitwise`

Executes integer shifts and bitwise operations with JVM-defined semantics.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.3>  
Snapshot tests: 2

#### Criteria

- Masks integer shift distances to five bits.
- Distinguishes arithmetic and logical right shifts.
- Implements integer complement, conjunction, disjunction, and exclusive-or operations.

### `execution.integer.comparisons`

Evaluates signed integer comparison expressions.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.7>  
Snapshot tests: 1

#### Criteria

- Compares signed integer values by order.
- Evaluates integer equality and inequality.

### `execution.integer.conversions`

Narrows integer values to byte, short, and char values.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.5>  
Snapshot tests: 2

#### Criteria

- Narrows integers to signed byte values.
- Narrows integers to signed short values.
- Narrows integers to unsigned char values.
- Promotes narrowed char values back to integers without sign extension.
- Applies narrowing conversion after compound assignments and increment or decrement.

### `execution.invocation.interface`

Resolves and invokes methods through interface references.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokeinterface>  
Snapshot tests: 1

#### Criteria

- Selects an implementing class method through an interface reference.
- Selects and invokes an inherited interface default method.

### `execution.invocation.special`

Invokes a selected superclass implementation without virtual dispatch.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokespecial>  
Snapshot tests: 1

#### Criteria

- Invokes a superclass method selected by a super call.
- Bypasses an overriding method on the current receiver class.

### `execution.invocation.static`

Resolves and invokes class and interface static methods.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokestatic>  
Snapshot tests: 1

#### Criteria

- Invokes a static class method without a receiver.
- Invokes a static interface method without a receiver.

### `execution.invocation.virtual`

Selects and invokes instance methods through class hierarchies.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokevirtual>  
Snapshot tests: 1

#### Criteria

- Selects an override from the runtime receiver class.
- Invokes concrete implementations of abstract superclass methods.
- Invokes inherited concrete methods.

### `execution.long.arithmetic`

Executes long arithmetic with JVM-defined overflow and division semantics.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.3>  
Snapshot tests: 5

#### Criteria

- Wraps addition, subtraction, multiplication, and negation on overflow.
- Implements signed division and remainder, including the minimum-value edge case.
- Throws ArithmeticException when a long divisor is zero.
- Implements compound arithmetic assignments and increment and decrement expressions.

### `execution.long.bitwise`

Executes long shifts and bitwise operations with JVM-defined semantics.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.3>  
Snapshot tests: 2

#### Criteria

- Masks long shift distances to six bits.
- Distinguishes arithmetic and logical right shifts.
- Implements long complement, conjunction, disjunction, and exclusive-or operations.

### `execution.long.comparisons`

Evaluates signed long comparison expressions.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.7>  
Snapshot tests: 1

#### Criteria

- Compares signed long values by order.
- Evaluates long equality and inequality.

### `execution.long.conversions`

Converts between integer and long computational values.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.5>  
Snapshot tests: 1

#### Criteria

- Widens signed integer values to long without loss.
- Narrows long values to the low 32 integer bits.

### `execution.objects.instance-default-values`

Initializes fields in newly allocated objects with JVM default values.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.new>  
Snapshot tests: 1

#### Criteria

- Initializes boolean fields to false and numeric primitive fields to zero.
- Initializes reference fields to null.
- Initializes inherited fields in newly allocated subclass instances.
- Gives each object independent default-initialized field storage.

### `execution.references.casting`

Checks whether a reference can be cast to a target reference type.

Status: **Partial**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.checkcast>  
Snapshot tests: 1

#### Criteria

- Accepts null for class, interface, and array target types.
- Accepts references assignment-compatible with the target type.
- Throws ClassCastException for incompatible non-null references.

#### Limitations

- Current non-null casts are accepted without assignment-compatibility checks.

### `natives.binding`

Binds native methods to registered VM implementations.

Status: **Partial**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.6>  
Snapshot tests: 1

#### Criteria

- Resolves registered native methods by declaring class, name, and descriptor.
- Throws UnsatisfiedLinkError when no implementation is bound.
- Preserves the missing native call as a native stack frame.

#### Limitations

- Binding is limited to the internal registry and does not load general native libraries.
- UnsatisfiedLinkError message punctuation differs from the reference JVM.

### `natives.class.assertion-status`

Supplies assertion enablement used by compiled assert statements.

Status: **Partial**  
Specification: <https://docs.oracle.com/javase/specs/jls/se25/html/jls-14.html#jls-14.10>  
Snapshot tests: 1

#### Criteria

- Enables assertion condition evaluation before class initialization completes.
- Allows a true assertion to complete normally.
- Constructs AssertionError with the detail expression when a condition is false.
- Honors launcher, class, package, and class-loader assertion configuration.

#### Limitations

- Assertion status is hardcoded enabled and ignores enablement and disablement configuration.

### `natives.system.arraycopy`

Copies array subsequences with Java type, bounds, overlap, and exception semantics.

Status: **Partial**  
Specification: Not specified  
Snapshot tests: 3

#### Criteria

- Copies primitive and reference subsequences while preserving untouched components.
- Handles overlapping copies as if the source subsequence were copied through a temporary array.
- Validates null references, array kinds, component compatibility, and bounds even for zero-length copies.
- Throws NullPointerException for null source or destination arguments before array-kind validation.
- Throws ArrayStoreException for non-array arguments and incompatible primitive or reference components.
- Throws IndexOutOfBoundsException for invalid positions or lengths without modifying the destination.
- Copies only the compatible reference prefix before throwing ArrayStoreException for an incompatible element.

#### Limitations

- Zero-length copies bypass component-type and bounds validation.
- Primitive component mismatches and primitive/reference mismatches are copied without ArrayStoreException.
- Reference components are copied without assignment-compatibility checks or required partial-copy behavior.
- Destination null validation occurs after source array-kind validation.
- Bounds checks use overflow-prone signed addition.
- Reference-array object headers do not consistently contain array-class identity.

