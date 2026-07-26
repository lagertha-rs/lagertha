# Lagertha Features

Generated for Lagertha `unreleased`.

Feature status describes declared JVM behavior. Test counts mean passing integration snapshot evidence, not exhaustive criterion coverage.

## Summary

| Status | Features |
|---|---:|
| Implemented | 2 |
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
| `execution.integer.arithmetic` | Implemented | 2 | Executes integer arithmetic and related operations with JVM-defined semantics. |

## Feature Details

### `class-format.interface-flags`

Validates access flag combinations for interface class files.

Status: **Implemented**  
Specification: JVMS 4.1  
Snapshot tests: 1

#### Criteria

- Requires ACC_ABSTRACT when ACC_INTERFACE is set.
- Rejects ACC_FINAL, ACC_SUPER, ACC_ENUM, and ACC_MODULE on interfaces.

### `execution.integer.arithmetic`

Executes integer arithmetic and related operations with JVM-defined semantics.

Status: **Implemented**  
Specification: JVMS 2.11.3  
Snapshot tests: 2

#### Criteria

- Wraps addition, subtraction, multiplication, and negation on overflow.
- Implements signed division and remainder, including the minimum-value edge case.
- Throws ArithmeticException when an integer divisor is zero.
- Masks shift distances and distinguishes arithmetic from logical right shifts.
- Implements integer complement, conjunction, disjunction, and exclusive-or operations.
- Narrows integers to byte, short, and char values.
- Implements signed comparisons and increment and decrement expressions.

