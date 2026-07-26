# Lagertha Features

Generated for Lagertha `unreleased`.

Feature status describes declared JVM behavior. Test counts mean passing integration snapshot evidence, not exhaustive criterion coverage.

## Summary

| Status | Features |
|---|---:|
| Implemented | 5 |
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
| `execution.integer.arithmetic` | Implemented | 4 | Executes integer arithmetic with JVM-defined overflow and division semantics. |
| `execution.integer.bitwise` | Implemented | 2 | Executes integer shifts and bitwise operations with JVM-defined semantics. |
| `execution.integer.comparisons` | Implemented | 1 | Evaluates signed integer comparison expressions. |
| `execution.integer.conversions` | Implemented | 1 | Narrows integer values to byte, short, and char values. |

## Feature Details

### `class-format.interface-flags`

Validates access flag combinations for interface class files.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.1>  
Snapshot tests: 1

#### Criteria

- Requires ACC_ABSTRACT when ACC_INTERFACE is set.
- Rejects ACC_FINAL, ACC_SUPER, ACC_ENUM, and ACC_MODULE on interfaces.

### `execution.integer.arithmetic`

Executes integer arithmetic with JVM-defined overflow and division semantics.

Status: **Implemented**  
Specification: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.11.3>  
Snapshot tests: 4

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
Snapshot tests: 1

#### Criteria

- Narrows integers to signed byte values.
- Narrows integers to signed short values.
- Narrows integers to unsigned char values.
- Promotes narrowed char values back to integers without sign extension.

