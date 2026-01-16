# Project Context

## Purpose
Foundation crate providing shared utilities, error types, parsing helpers, and telemetry infrastructure for the Rust Java VM workspace. This crate establishes common patterns and types used by all other crates in the workspace.

## Tech Stack
- **Core Dependencies**: `num_enum` (0.7.4) for safe enum conversions
- **Workspace Dependencies**: `tracing-subscriber` for telemetry integration
- **No External Runtime Dependencies**: Pure utility crate focused on type definitions and parsing

## Workspace Context
- **This Crate Role**: Foundational layer - all other crates depend on `common`
- **Internal Dependencies**: None (base layer)
- **Internal Dependents**: `jclass`, `jimage`, `runtime`, `vm`, `javap` (ALL other crates)
- **Navigation**: Other crates reference this via `common::` namespace (e.g., `common::error::ClassFormatErr`)

## Crate-Specific Conventions

### Error Handling Pattern
- **Hierarchical Error Enums**: Structured error types in `error.rs` with clear categorization
- **From<T> Implementations**: Extensive trait implementations for seamless error conversion between layers
- **Error Categories**:
  - `SignatureErr`, `TypeDescriptorErr`, `MethodDescriptorErr` - parsing errors
  - `InstructionErr` - bytecode instruction errors
  - `LinkageError`, `RuntimePoolError` - runtime linking errors
  - `ClassFormatErr` - class file validation errors

### Parsing Utilities
- **ByteCursor**: Binary parsing with big/little endian support and error tracking
- **Recursive Parsing**: `try_recursive()` pattern for nested structure parsing
- **Descriptor Parsing**: Complete JVM type descriptor and signature parsing
- **Validation**: Early error detection with detailed error messages

### Type System
- **JavaType Enum**: Comprehensive Java type representation (primitives, objects, arrays, generics)
- **Instruction Definitions**: Complete JVM opcode set with metadata (byte size, branching behavior)
- **Allocation Information**: Memory layout details for runtime allocation planning

### Pretty Printing
- **IndentWriter**: Utilities for formatted output with consistent indentation
- **Macro Support**: `pretty_try!` and `pretty_class_name_try!` macros for error-handling during display
- **Telemetry Integration**: `init_tracing()` for structured logging setup

## Testing Approach
- **Unit Tests**: Embedded in each module (`#[cfg(test)]`) focusing on parsing validation
- **Parsing Validation**: Extensive tests for descriptor and signature parsing edge cases
- **Error Coverage**: Tests verify correct error propagation through From trait conversions
- **No Integration Tests**: Pure utility crate tested through dependent crates' usage

## Domain Knowledge Required
- **JVM Type System**: Understanding of Java primitive types, object references, arrays, and generic signatures
- **Class File Structure**: Knowledge of constant pool, descriptors, and basic class file format
- **Bytecode Basics**: Familiarity with JVM instruction set and operand stack model
- **Error Propagation**: Rust error handling patterns with type conversions

## Important Constraints
- **Stability Critical**: Changes affect ALL other crates - must maintain backward compatibility
- **Minimal Dependencies**: Avoid adding dependencies that would bloat dependent crates
- **Performance Neutral**: Utilities should not introduce significant overhead
- **Clear Documentation**: Type definitions must be well-documented as they're public API

## External Dependencies
- **num_enum = "0.7.4"**: For `#[derive(FromPrimitive, TryFromPrimitive)]` on enum types
- **tracing-subscriber** (workspace): Only for telemetry utilities module

## Module Structure
```
src/
├── lib.rs (re-exports)
├── descriptor.rs (type/method descriptor parsing)
├── error.rs (hierarchical error types)
├── instruction.rs (bytecode instruction definitions)
├── jtype.rs (Java type system representation)
├── signature.rs (generic signature parsing)
└── utils/
    ├── cursor.rs (binary parsing)
    ├── indent_write.rs (pretty printing)
    └── telemetry.rs (tracing setup)
```

## Usage Examples
```rust
// Error conversion
use common::error::{ClassFormatErr, SignatureErr};
let err: ClassFormatErr = SignatureErr::UnexpectedEnd.into();

// Type parsing
use common::descriptor::{MethodDescriptor, FieldDescriptor};
let method: MethodDescriptor = "(Ljava/lang/String;)V".try_into()?;

// Bytecode parsing
use common::instruction::Instruction;
let instr = Instruction::new_at(&bytecode, offset)?;
```