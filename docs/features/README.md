# Lagertha Features

Generated for Lagertha `unreleased`.

Feature status describes declared JVM behavior. Test counts mean passing integration snapshot evidence, not exhaustive criterion coverage.

## Summary

| Status | Features |
|---|---:|
| Implemented | 19 |
| Partial | 0 |
| Missing | 0 |
| Blocked | 0 |
| Deferred | 0 |

## Feature Index

### class-format

| Feature | Status | Tests | Description |
|---|---|---:|---|
| `class-format.interface-flags` | Implemented | 1 | Validates access flag combinations for interface class files. |

### execution

| Feature | Status | Tests | Description |
|---|---|---:|---|
| `execution.control-flow.conditional-branches` | Implemented | 2 | Executes value-dependent conditional control flow. |
| `execution.control-flow.unconditional-branches` | Implemented | 1 | Executes forward and backward unconditional control transfers. |
| `execution.fields.access` | Implemented | 1 | Resolves and accesses instance and static fields. |
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

## Feature Details

### `class-format.interface-flags`

Validates access flag combinations for interface class files.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.1>  
Snapshot tests: 1

#### Criteria

- Requires ACC_ABSTRACT when ACC_INTERFACE is set.
- Rejects ACC_FINAL, ACC_SUPER, ACC_ENUM, and ACC_MODULE on interfaces.

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
Snapshot tests: 1

#### Criteria

- Reads and writes instance fields.
- Reads default and initialized instance field values.
- Reads and writes static fields.
- Resolves inherited fields through a subclass instance.

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

