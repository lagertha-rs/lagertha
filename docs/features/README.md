# Lagertha Feature Support

Generated for Lagertha `0.7.2`.

Lagertha is an early-stage educational JVM targeting Java 25. This report covers capabilities explicitly recorded in the feature registry; it is not a complete inventory of Java 25 or JVM functionality.

For execution evidence and internal test mapping, see [Integration Test Coverage](TEST_COVERAGE.md).

## Understanding Statuses

| Status | Meaning |
|---|---|
| Implemented | All declared scope criteria are implemented. This does not imply complete support for the broader JVM area. |
| Partial | Some declared behavior works; listed limitations remain. |
| Missing | No meaningful implementation exists for the declared scope. |
| Blocked | Work depends on another capability. |
| Deferred | Work is intentionally outside the current horizon. |

Declared scope defines the boundary evaluated by a status. In particular, **Implemented** does not mean every behavior in the linked specification section is supported.

## Capability Index

### Bootstrap

| Capability | Stable ID | Status | Summary |
|---|---|---|---|
| [Main method startup](#feature-bootstrap-main-method) | `bootstrap.main-method` | Partial | Selects an initial class and invokes its main method to drive VM execution. |

### Class format

| Capability | Stable ID | Status | Summary |
|---|---|---|---|
| [Interface access flags](#feature-class-format-interface-flags) | `class-format.interface-flags` | Implemented | Validates access flag combinations for interface class files. |

### Class loading

| Capability | Stable ID | Status | Summary |
|---|---|---|---|
| [Class and interface initialization](#feature-class-loading-initialization) | `class-loading.initialization` | Partial | Executes class and interface initialization methods after preparation. |
| [Class and interface preparation](#feature-class-loading-preparation) | `class-loading.preparation` | Implemented | Creates static field storage and assigns JVM default values before initialization. |

### Exceptions

| Capability | Stable ID | Status | Summary |
|---|---|---|---|
| [Exception handler selection](#feature-exceptions-handler-selection) | `exceptions.handler-selection` | Implemented | Searches exception tables in order for a handler compatible with the thrown object. |
| [Exception propagation](#feature-exceptions-propagation) | `exceptions.propagation` | Partial | Abruptly completes frames until a caller provides a matching handler. |
| [Exception stack traces](#feature-exceptions-stack-traces) | `exceptions.stack-traces` | Partial | Captures and renders Java call frames associated with a thrown exception. |
| [Explicit exception throwing](#feature-exceptions-throwing) | `exceptions.throwing` | Partial | Throws exception references with the athrow instruction. |
| [Uncaught exception reporting](#feature-exceptions-uncaught-reporting) | `exceptions.uncaught-reporting` | Partial | Reports an uncaught main-thread exception and terminates unsuccessfully. |

### Execution

| Capability | Stable ID | Status | Summary |
|---|---|---|---|
| [Array access exceptions](#feature-execution-arrays-access-exceptions) | `execution.arrays.access-exceptions` | Implemented | Rejects null array references and indices outside array bounds. |
| [Array allocation exceptions](#feature-execution-arrays-allocation-exceptions) | `execution.arrays.allocation-exceptions` | Implemented | Rejects negative one-dimensional primitive and reference array lengths. |
| [Array default values](#feature-execution-arrays-default-values) | `execution.arrays.default-values` | Implemented | Initializes newly allocated array components with JVM default values. |
| [Array length](#feature-execution-arrays-length) | `execution.arrays.length` | Implemented | Returns the fixed component count of an array. |
| [Multidimensional arrays](#feature-execution-arrays-multidimensional) | `execution.arrays.multidimensional` | Implemented | Allocates and accesses rectangular and nested array structures. |
| [Primitive array elements](#feature-execution-arrays-primitive-elements) | `execution.arrays.primitive-elements` | Implemented | Allocates primitive arrays and loads and stores their component values. |
| [Reference array elements](#feature-execution-arrays-reference-elements) | `execution.arrays.reference-elements` | Implemented | Allocates reference arrays and loads and stores compatible references. |
| [Conditional branches](#feature-execution-control-flow-conditional-branches) | `execution.control-flow.conditional-branches` | Implemented | Executes value-dependent conditional control flow. |
| [Unconditional branches](#feature-execution-control-flow-unconditional-branches) | `execution.control-flow.unconditional-branches` | Implemented | Executes forward and backward unconditional control transfers. |
| [Field access](#feature-execution-fields-access) | `execution.fields.access` | Implemented | Resolves and accesses instance and static fields. |
| [Local variables](#feature-execution-frames-local-variables) | `execution.frames.local-variables` | Implemented | Stores and loads JVM computational values in local variable slots. |
| [Method arguments](#feature-execution-frames-method-arguments) | `execution.frames.method-arguments` | Implemented | Transfers receiver and argument values into callee local variable slots. |
| [Recursive frames](#feature-execution-frames-recursion) | `execution.frames.recursion` | Implemented | Creates isolated frames for recursive method invocation. |
| [Integer arithmetic](#feature-execution-integer-arithmetic) | `execution.integer.arithmetic` | Implemented | Executes integer arithmetic with JVM-defined overflow and division semantics. |
| [Integer bitwise operations](#feature-execution-integer-bitwise) | `execution.integer.bitwise` | Implemented | Executes integer shifts and bitwise operations with JVM-defined semantics. |
| [Integer comparisons](#feature-execution-integer-comparisons) | `execution.integer.comparisons` | Implemented | Evaluates signed integer comparison expressions. |
| [Integer narrowing conversions](#feature-execution-integer-conversions) | `execution.integer.conversions` | Implemented | Narrows integer values to byte, short, and char values. |
| [Interface method invocation](#feature-execution-invocation-interface) | `execution.invocation.interface` | Implemented | Resolves and invokes methods through interface references. |
| [Special method invocation](#feature-execution-invocation-special) | `execution.invocation.special` | Implemented | Invokes a selected superclass implementation without virtual dispatch. |
| [Static method invocation](#feature-execution-invocation-static) | `execution.invocation.static` | Implemented | Resolves and invokes class and interface static methods. |
| [Virtual method invocation](#feature-execution-invocation-virtual) | `execution.invocation.virtual` | Implemented | Selects and invokes instance methods through class hierarchies. |
| [Long arithmetic](#feature-execution-long-arithmetic) | `execution.long.arithmetic` | Implemented | Executes long arithmetic with JVM-defined overflow and division semantics. |
| [Long bitwise operations](#feature-execution-long-bitwise) | `execution.long.bitwise` | Implemented | Executes long shifts and bitwise operations with JVM-defined semantics. |
| [Long comparisons](#feature-execution-long-comparisons) | `execution.long.comparisons` | Implemented | Evaluates signed long comparison expressions. |
| [Long conversions](#feature-execution-long-conversions) | `execution.long.conversions` | Implemented | Converts between integer and long computational values. |
| [Instance field default values](#feature-execution-objects-instance-default-values) | `execution.objects.instance-default-values` | Implemented | Initializes fields in newly allocated objects with JVM default values. |
| [Reference casting](#feature-execution-references-casting) | `execution.references.casting` | Partial | Checks whether a reference can be cast to a target reference type. |

### Natives

| Capability | Stable ID | Status | Summary |
|---|---|---|---|
| [Native method binding](#feature-natives-binding) | `natives.binding` | Partial | Binds native methods to registered VM implementations. |
| [Class assertion status](#feature-natives-class-assertion-status) | `natives.class.assertion-status` | Partial | Supplies assertion enablement used by compiled assert statements. |
| [Object runtime class](#feature-natives-object-get-class) | `natives.object.get-class` | Partial | Returns the canonical Class mirror representing an object's runtime class. |
| [System array copying](#feature-natives-system-arraycopy) | `natives.system.arraycopy` | Partial | Copies array subsequences with Java type, bounds, overlap, and exception semantics. |

## Known Gaps in Tracked Capabilities

This section summarizes incomplete capabilities already present in the registry. It is not a complete project roadmap or inventory of missing Java 25 functionality.

| Capability | Status | Primary gap |
|---|---|---|
| [Main method startup](#feature-bootstrap-main-method) | Partial | Startup only looks up a main method with descriptor ([Ljava/lang/String;)V. |
| [Class and interface initialization](#feature-class-loading-initialization) | Partial | Superinterface selection and ordering do not yet implement the default-method rules. |
| [Exception propagation](#feature-exceptions-propagation) | Partial | Synchronized-method monitor acquisition and release are not implemented. |
| [Exception stack traces](#feature-exceptions-stack-traces) | Partial | Causes, suppressed exceptions, common-frame elision, modules, loaders, and unknown-source frames lack integration evidence. |
| [Explicit exception throwing](#feature-exceptions-throwing) | Partial | Handler entry pushes the exception onto the existing operand stack instead of clearing it first. |
| [Uncaught exception reporting](#feature-exceptions-uncaught-reporting) | Partial | Only main-thread reporting has integration evidence. |
| [Reference casting](#feature-execution-references-casting) | Partial | Current non-null casts are accepted without assignment-compatibility checks. |
| [Native method binding](#feature-natives-binding) | Partial | Binding is limited to the internal registry and does not load general native libraries. |
| [Class assertion status](#feature-natives-class-assertion-status) | Partial | Assertion status is hardcoded enabled and ignores enablement and disablement configuration. |
| [Object runtime class](#feature-natives-object-get-class) | Partial | Primitive and multidimensional arrays produce incorrect mirrors because array descriptors are always reconstructed as one-dimensional reference arrays. |
| [System array copying](#feature-natives-system-arraycopy) | Partial | Zero-length copies bypass component-type and bounds validation. |

## Capability Details

### Bootstrap

<a id="feature-bootstrap-main-method"></a>

#### Main method startup

Selects an initial class and invokes its main method to drive VM execution.

**Stable ID:** `bootstrap.main-method`  
**Status:** Partial  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.2)

##### Declared Scope

- Loads, links, and initializes the selected initial class or interface.
- Selects a Java SE 25 candidate main method.
- Supplies the required string argument array when applicable.
- Invokes the selected main method and lets it drive further execution.

##### Current Limitations

- Startup only looks up a main method with descriptor ([Ljava/lang/String;)V.
- No-argument, instance, inherited, and non-public Java SE 25 candidate main methods are not selected.
- Program arguments are not exposed by the launcher or supplied as a String array.
- Missing or unsupported main methods do not consistently produce specified launcher failure behavior.

### Class format

<a id="feature-class-format-interface-flags"></a>

#### Interface access flags

Validates access flag combinations for interface class files.

**Stable ID:** `class-format.interface-flags`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.1)

##### Declared Scope

- Requires ACC_ABSTRACT when ACC_INTERFACE is set.
- Rejects ACC_FINAL, ACC_SUPER, ACC_ENUM, and ACC_MODULE on interfaces.

### Class loading

<a id="feature-class-loading-initialization"></a>

#### Class and interface initialization

Executes class and interface initialization methods after preparation.

**Stable ID:** `class-loading.initialization`  
**Status:** Partial  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.5)

##### Declared Scope

- Triggers initialization on active static field access and object creation.
- Initializes superclasses before subclasses.
- Executes initialization expressions and blocks in class-file order exactly once.
- Initializes an interface when its non-constant static field is actively used.
- Initializes required superinterfaces that declare default methods in specification order.
- Coordinates concurrent initialization and records initialization failures.

##### Current Limitations

- Superinterface selection and ordering do not yet implement the default-method rules.
- Concurrent initialization does not track the initializing thread or wait for completion.
- Failed initialization does not mark the class erroneous or produce the required later errors.
- ConstantValue field attributes are not applied during initialization.

<a id="feature-class-loading-preparation"></a>

#### Class and interface preparation

Creates static field storage and assigns JVM default values before initialization.

**Stable ID:** `class-loading.preparation`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.4.2)

##### Declared Scope

- Creates static storage for classes and interfaces.
- Assigns false and numeric zero to primitive static fields before explicit initializers run.
- Assigns null to reference static fields before explicit initializers run.
- Makes prepared values observable to earlier expressions in the initialization method.

### Exceptions

<a id="feature-exceptions-handler-selection"></a>

#### Exception handler selection

Searches exception tables in order for a handler compatible with the thrown object.

**Stable ID:** `exceptions.handler-selection`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.10)

##### Declared Scope

- Selects handlers whose protected half-open range contains the throwing instruction.
- Chooses the first table entry whose catch type matches the thrown class or a superclass.
- Supports catch-all handlers and nested protected regions.
- Skips incompatible handlers and bypasses all handlers on normal completion.

<a id="feature-exceptions-propagation"></a>

#### Exception propagation

Abruptly completes frames until a caller provides a matching handler.

**Stable ID:** `exceptions.propagation`  
**Status:** Partial  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6.5)

##### Declared Scope

- Discards intervening Java frames while preserving the thrown exception object.
- Propagates exceptions from constructors and exception handlers.
- Skips instructions following an abruptly completed invocation.
- Executes compiler-generated catch-all and rethrow paths for finally blocks.
- Releases monitors held by synchronized methods during abrupt completion.

##### Current Limitations

- Synchronized-method monitor acquisition and release are not implemented.
- Internal failures during handler lookup can replace Java propagation with a VM error.

<a id="feature-exceptions-stack-traces"></a>

#### Exception stack traces

Captures and renders Java call frames associated with a thrown exception.

**Stable ID:** `exceptions.stack-traces`  
**Status:** Partial  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.10)

##### Declared Scope

- Captures the throwing method followed by caller frames.
- Renders method names, source files, and source line numbers.
- Represents native methods as native frames.
- Supports explicit Throwable.printStackTrace output.

##### Current Limitations

- Causes, suppressed exceptions, common-frame elision, modules, loaders, and unknown-source frames lack integration evidence.
- Stack-trace native implementations use an incomplete custom model.

<a id="feature-exceptions-throwing"></a>

#### Explicit exception throwing

Throws exception references with the athrow instruction.

**Stable ID:** `exceptions.throwing`  
**Status:** Partial  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.9)

##### Declared Scope

- Throws a non-null exception object and preserves its identity.
- Throws NullPointerException when the athrow operand is null.
- Supplies the thrown reference to a matching exception handler.
- Clears the operand stack before entering a matching handler.

##### Current Limitations

- Handler entry pushes the exception onto the existing operand stack instead of clearing it first.
- Abrupt synchronized-method completion does not release the method monitor.

<a id="feature-exceptions-uncaught-reporting"></a>

#### Uncaught exception reporting

Reports an uncaught main-thread exception and terminates unsuccessfully.

**Stable ID:** `exceptions.uncaught-reporting`  
**Status:** Partial  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.7)

##### Declared Scope

- Terminates the VM unsuccessfully when an exception escapes the main method.
- Reports the thread name, exception type, message, and Java frames.
- Dispatches the exception through the main thread group.

##### Current Limitations

- Only main-thread reporting has integration evidence.
- Custom handlers, secondary threads, and failures during reporting are not supported or tested.

### Execution

<a id="feature-execution-arrays-access-exceptions"></a>

#### Array access exceptions

Rejects null array references and indices outside array bounds.

**Stable ID:** `execution.arrays.access-exceptions`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.5)

##### Declared Scope

- Throws ArrayIndexOutOfBoundsException for negative and length-equal load indices.
- Throws ArrayIndexOutOfBoundsException for negative and length-equal store indices.
- Throws NullPointerException for loads and stores through null array references.
- Throws NullPointerException for access through a null nested row.
- Leaves valid components unchanged after a failed store.

<a id="feature-execution-arrays-allocation-exceptions"></a>

#### Array allocation exceptions

Rejects negative one-dimensional primitive and reference array lengths.

**Stable ID:** `execution.arrays.allocation-exceptions`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.5)

##### Declared Scope

- Throws NegativeArraySizeException for a negative primitive array length.
- Throws NegativeArraySizeException for a negative reference array length.
- Performs no array allocation when the requested length is negative.

<a id="feature-execution-arrays-default-values"></a>

#### Array default values

Initializes newly allocated array components with JVM default values.

**Stable ID:** `execution.arrays.default-values`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.5)

##### Declared Scope

- Initializes boolean components to false and numeric components to zero.
- Initializes reference components to null.
- Initializes each newly allocated nested row independently.
- Leaves unallocated nested rows null.

<a id="feature-execution-arrays-length"></a>

#### Array length

Returns the fixed component count of an array.

**Stable ID:** `execution.arrays.length`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.arraylength)

##### Declared Scope

- Returns zero for an empty array.
- Returns the allocated length for primitive and reference arrays.
- Returns each independently allocated nested row length.
- Throws NullPointerException for a null array reference.

<a id="feature-execution-arrays-multidimensional"></a>

#### Multidimensional arrays

Allocates and accesses rectangular and nested array structures.

**Stable ID:** `execution.arrays.multidimensional`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.multianewarray)

##### Declared Scope

- Allocates all requested dimensions of rectangular primitive and reference arrays.
- Loads and stores values through nested array references.
- Supports partially allocated dimensions whose components start null.
- Supports independently sized, empty, reassigned, and aliased rows.

<a id="feature-execution-arrays-primitive-elements"></a>

#### Primitive array elements

Allocates primitive arrays and loads and stores their component values.

**Stable ID:** `execution.arrays.primitive-elements`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.5)

##### Declared Scope

- Allocates arrays for every primitive component type.
- Loads and stores boolean, byte, char, short, int, long, float, and double components.
- Preserves signed byte and short values and unsigned char values.
- Normalizes boolean array values to true or false.

<a id="feature-execution-arrays-reference-elements"></a>

#### Reference array elements

Allocates reference arrays and loads and stores compatible references.

**Stable ID:** `execution.arrays.reference-elements`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.5)

##### Declared Scope

- Allocates arrays with object and narrower reference component types.
- Loads and stores compatible object references and null.
- Preserves object identity and aliases through array components.
- Preserves array identity when array references are copied and reassigned.

<a id="feature-execution-control-flow-conditional-branches"></a>

#### Conditional branches

Executes value-dependent conditional control flow.

**Stable ID:** `execution.control-flow.conditional-branches`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.7)

##### Declared Scope

- Selects control flow from integer zero and nonzero conditions.
- Preserves short-circuit evaluation of compiled conditional expressions.
- Selects the correct compiled conditional-expression branch.
- Repeats and exits compiled loop control flow.

<a id="feature-execution-control-flow-unconditional-branches"></a>

#### Unconditional branches

Executes forward and backward unconditional control transfers.

**Stable ID:** `execution.control-flow.unconditional-branches`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.goto)

##### Declared Scope

- Transfers execution forward without fallthrough.
- Transfers execution backward to continue repeated control flow.
- Exits nested compiled control-flow regions at the selected target.

<a id="feature-execution-fields-access"></a>

#### Field access

Resolves and accesses instance and static fields.

**Stable ID:** `execution.fields.access`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.4.3.2)

##### Declared Scope

- Reads and writes primitive and reference instance fields.
- Reads and writes primitive and reference static fields.
- Keeps instance storage independent and static storage shared across objects.
- Resolves inherited fields and keeps hidden same-name fields distinct by declaring class.
- Throws NullPointerException for instance field reads and writes through a null receiver.

<a id="feature-execution-frames-local-variables"></a>

#### Local variables

Stores and loads JVM computational values in local variable slots.

**Stable ID:** `execution.frames.local-variables`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6.1)

##### Declared Scope

- Stores and loads int, float, reference, long, and double computational values.
- Preserves category-2 long and double values across two local slots.
- Stores null, object, and array references in reference local slots.

<a id="feature-execution-frames-method-arguments"></a>

#### Method arguments

Transfers receiver and argument values into callee local variable slots.

**Stable ID:** `execution.frames.method-arguments`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6.1)

##### Declared Scope

- Preserves static argument order across category-1 and category-2 values.
- Places an instance receiver before explicit method arguments.
- Places a constructor receiver before explicit constructor arguments.
- Passes primitive and reference values by value.
- Passes null and array references.

<a id="feature-execution-frames-recursion"></a>

#### Recursive frames

Creates isolated frames for recursive method invocation.

**Stable ID:** `execution.frames.recursion`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6)

##### Declared Scope

- Creates a fresh parameter local for each recursive invocation.
- Preserves pending return values across branching recursive calls.
- Returns composed results through nested frames.

<a id="feature-execution-integer-arithmetic"></a>

#### Integer arithmetic

Executes integer arithmetic with JVM-defined overflow and division semantics.

**Stable ID:** `execution.integer.arithmetic`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.3)

##### Declared Scope

- Wraps addition, subtraction, multiplication, and negation on overflow.
- Implements signed division and remainder, including the minimum-value edge case.
- Throws ArithmeticException when an integer divisor is zero.
- Implements compound arithmetic assignments and increment and decrement expressions.

<a id="feature-execution-integer-bitwise"></a>

#### Integer bitwise operations

Executes integer shifts and bitwise operations with JVM-defined semantics.

**Stable ID:** `execution.integer.bitwise`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.3)

##### Declared Scope

- Masks integer shift distances to five bits.
- Distinguishes arithmetic and logical right shifts.
- Implements integer complement, conjunction, disjunction, and exclusive-or operations.

<a id="feature-execution-integer-comparisons"></a>

#### Integer comparisons

Evaluates signed integer comparison expressions.

**Stable ID:** `execution.integer.comparisons`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.7)

##### Declared Scope

- Compares signed integer values by order.
- Evaluates integer equality and inequality.

<a id="feature-execution-integer-conversions"></a>

#### Integer narrowing conversions

Narrows integer values to byte, short, and char values.

**Stable ID:** `execution.integer.conversions`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.5)

##### Declared Scope

- Narrows integers to signed byte values.
- Narrows integers to signed short values.
- Narrows integers to unsigned char values.
- Promotes narrowed char values back to integers without sign extension.
- Applies narrowing conversion after compound assignments and increment or decrement.

<a id="feature-execution-invocation-interface"></a>

#### Interface method invocation

Resolves and invokes methods through interface references.

**Stable ID:** `execution.invocation.interface`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokeinterface)

##### Declared Scope

- Selects an implementing class method through an interface reference.
- Selects and invokes an inherited interface default method.

<a id="feature-execution-invocation-special"></a>

#### Special method invocation

Invokes a selected superclass implementation without virtual dispatch.

**Stable ID:** `execution.invocation.special`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokespecial)

##### Declared Scope

- Invokes a superclass method selected by a super call.
- Bypasses an overriding method on the current receiver class.

<a id="feature-execution-invocation-static"></a>

#### Static method invocation

Resolves and invokes class and interface static methods.

**Stable ID:** `execution.invocation.static`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokestatic)

##### Declared Scope

- Invokes a static class method without a receiver.
- Invokes a static interface method without a receiver.

<a id="feature-execution-invocation-virtual"></a>

#### Virtual method invocation

Selects and invokes instance methods through class hierarchies.

**Stable ID:** `execution.invocation.virtual`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokevirtual)

##### Declared Scope

- Selects an override from the runtime receiver class.
- Invokes concrete implementations of abstract superclass methods.
- Invokes inherited concrete methods.

<a id="feature-execution-long-arithmetic"></a>

#### Long arithmetic

Executes long arithmetic with JVM-defined overflow and division semantics.

**Stable ID:** `execution.long.arithmetic`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.3)

##### Declared Scope

- Wraps addition, subtraction, multiplication, and negation on overflow.
- Implements signed division and remainder, including the minimum-value edge case.
- Throws ArithmeticException when a long divisor is zero.
- Implements compound arithmetic assignments and increment and decrement expressions.

<a id="feature-execution-long-bitwise"></a>

#### Long bitwise operations

Executes long shifts and bitwise operations with JVM-defined semantics.

**Stable ID:** `execution.long.bitwise`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.3)

##### Declared Scope

- Masks long shift distances to six bits.
- Distinguishes arithmetic and logical right shifts.
- Implements long complement, conjunction, disjunction, and exclusive-or operations.

<a id="feature-execution-long-comparisons"></a>

#### Long comparisons

Evaluates signed long comparison expressions.

**Stable ID:** `execution.long.comparisons`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.7)

##### Declared Scope

- Compares signed long values by order.
- Evaluates long equality and inequality.

<a id="feature-execution-long-conversions"></a>

#### Long conversions

Converts between integer and long computational values.

**Stable ID:** `execution.long.conversions`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.5)

##### Declared Scope

- Widens signed integer values to long without loss.
- Narrows long values to the low 32 integer bits.

<a id="feature-execution-objects-instance-default-values"></a>

#### Instance field default values

Initializes fields in newly allocated objects with JVM default values.

**Stable ID:** `execution.objects.instance-default-values`  
**Status:** Implemented  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.new)

##### Declared Scope

- Initializes boolean fields to false and numeric primitive fields to zero.
- Initializes reference fields to null.
- Initializes inherited fields in newly allocated subclass instances.
- Gives each object independent default-initialized field storage.

<a id="feature-execution-references-casting"></a>

#### Reference casting

Checks whether a reference can be cast to a target reference type.

**Stable ID:** `execution.references.casting`  
**Status:** Partial  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.checkcast)

##### Declared Scope

- Accepts null for class, interface, and array target types.
- Accepts references assignment-compatible with the target type.
- Throws ClassCastException for incompatible non-null references.

##### Current Limitations

- Current non-null casts are accepted without assignment-compatibility checks.

### Natives

<a id="feature-natives-binding"></a>

#### Native method binding

Binds native methods to registered VM implementations.

**Stable ID:** `natives.binding`  
**Status:** Partial  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.6)

##### Declared Scope

- Resolves registered native methods by declaring class, name, and descriptor.
- Throws UnsatisfiedLinkError when no implementation is bound.
- Preserves the missing native call as a native stack frame.

##### Current Limitations

- Binding is limited to the internal registry and does not load general native libraries.
- UnsatisfiedLinkError message punctuation differs from the reference JVM.

<a id="feature-natives-class-assertion-status"></a>

#### Class assertion status

Supplies assertion enablement used by compiled assert statements.

**Stable ID:** `natives.class.assertion-status`  
**Status:** Partial  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jls/se25/html/jls-14.html#jls-14.10)

##### Declared Scope

- Enables assertion condition evaluation before class initialization completes.
- Allows a true assertion to complete normally.
- Constructs AssertionError with the detail expression when a condition is false.
- Honors launcher, class, package, and class-loader assertion configuration.

##### Current Limitations

- Assertion status is hardcoded enabled and ignores enablement and disablement configuration.

<a id="feature-natives-object-get-class"></a>

#### Object runtime class

Returns the canonical Class mirror representing an object's runtime class.

**Stable ID:** `natives.object.get-class`  
**Status:** Partial  
**Java SE 25 reference:** [Specification](https://docs.oracle.com/javase/specs/jls/se25/html/jls-4.html#jls-4.3.2)

##### Declared Scope

- Returns the runtime implementation mirror independently of the reference's static class or interface type.
- Returns an array-class mirror when the receiver is an array.
- Reuses one Class object across getClass calls, distinct instances, and class literals.
- Distinguishes an implementation-class mirror from mirrors for its implemented interfaces.

##### Current Limitations

- Primitive and multidimensional arrays produce incorrect mirrors because array descriptors are always reconstructed as one-dimensional reference arrays.
- Reference-array object headers commonly store component-class identity, so passing behavior depends on descriptor reconstruction.
- Runtime class identity does not include the defining class loader for same-named types.

<a id="feature-natives-system-arraycopy"></a>

#### System array copying

Copies array subsequences with Java type, bounds, overlap, and exception semantics.

**Stable ID:** `natives.system.arraycopy`  
**Status:** Partial  
**Java SE 25 reference:** No JVMS/JLS reference recorded.

##### Declared Scope

- Copies primitive and reference subsequences while preserving untouched components.
- Handles overlapping copies as if the source subsequence were copied through a temporary array.
- Validates null references, array kinds, component compatibility, and bounds even for zero-length copies.
- Throws NullPointerException for null source or destination arguments before array-kind validation.
- Throws ArrayStoreException for non-array arguments and incompatible primitive or reference components.
- Throws IndexOutOfBoundsException for invalid positions or lengths without modifying the destination.
- Copies only the compatible reference prefix before throwing ArrayStoreException for an incompatible element.

##### Current Limitations

- Zero-length copies bypass component-type and bounds validation.
- Primitive component mismatches and primitive/reference mismatches are copied without ArrayStoreException.
- Reference components are copied without assignment-compatibility checks or required partial-copy behavior.
- Destination null validation occurs after source array-kind validation.
- Bounds checks use overflow-prone signed addition.
- Reference-array object headers do not consistently contain array-class identity.

