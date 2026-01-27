# Project Context

## Purpose

Educational and fun Rust implementation of a Java Virtual Machine (JVM) targeting **Java 25** specification. This project explores JVM internals through pure Rust implementation, serving as both a learning resource and experimental platform for understanding Java runtime mechanics.

## Tech Stack

- **Language**: Rust 2024 edition
- **Build System**: Cargo workspace with 6 member crates
- **Logging**: Tracing ecosystem (`tracing`, `tracing-subscriber`, `tracing-log`) - used for structured logging only, not telemetry
- **Performance Profiling**: `hotpath` for experimental performance instrumentation (not heavily used, not fully configured)
- **Async Runtime**: Tokio for JDWP debugging agent (early stage, not functional yet)
- **Testing**: Insta (snapshot testing), rstest (parameterized testing), assert_cmd
- **Parsing Utilities**: `num_enum`, `itertools`, `memmap2` (jimage)
- **Concurrency**: `dashmap`, `lasso` (string interning), `smallvec`

## Project Conventions

### Code Style

- **Error Handling**: Hierarchical error enums with extensive `From<T>` trait implementations for seamless error conversion. **Note**: All error types are provisional and will be refactored in the near future as the project evolves.
- **Parsing Patterns**: `TryFrom<&str>` for text parsing, `ByteCursor` for binary parsing with endianness support
- **Immutable Core**: Class file data structures are immutable after parsing
- **Feature Flags**: `pretty_print` for disassembly output, `hotpath-*` for profiling (experimental), `log-runtime-traces` for debugging
- **Naming Conventions**: `Err` suffix for error types, `Info` for data structures, `Ref` for reference types

### Architecture Patterns

- **Separation of Concerns**: `jclass` for parsing vs `runtime` for execution, `common` for shared utilities
- **Module Organization**: Each crate has clear single responsibility with well-defined interfaces
- **Thread Safety**: `RwLock<T>` for shared structures, thread-local data for execution state
- **Extensibility**: Pluggable native method registry, modular attribute parsing

### Testing Strategy

- **Multi-Layer Approach**: Unit tests (embedded), integration tests (end-to-end), snapshot testing (insta)
- **Parameterized Testing**: `rstest` for file-based test discovery with automatic fixture management
- **CI Pipeline**: GitHub Actions with matrix testing (Rust stable + JDK 25.0.1)
- **Feature Matrix**: `cargo hack` for comprehensive feature combination testing
- **Snapshot Verification**: Output comparison against stored snapshots for regression testing
- **Test Fixtures**: `prepare_fixtures.py` script compiles Java test classes and extracts JDK classes

### Git Workflow

- **Branching**: Feature branches with descriptive names
- **Commits**: Conventional commits with clear scope and description
- **CI Enforcement**: All checks (fmt, clippy, tests) must pass before merge
- **Documentation**: README updates accompany significant changes

## Domain Context

### JVM Specification

- **Target Version**: Java 25 (JVM Specification SE 25)
- **Key Areas**: Class file format (Chapter 4), constant pool, fields/methods, attributes, bytecode instructions
- **Type System**: Primitive types, reference types, generic signatures, array dimensions
- **Execution Model**: Stack-based architecture, frame management, exception handling

### Java Runtime Concepts

- **Class Loading**: Binary parsing, verification, preparation, resolution, initialization
- **Memory Management**: Heap (objects), method area (metadata), string pool (interned strings). **No garbage collection yet.**
- **Bytecode Semantics**: 200+ opcodes with specific stack and local variable effects
- **Native Integration**: JNI-like interface for native method invocation

### Development Status

- **Current**: Complete class file parsing, basic bytecode execution, JDWP framework (early stage, not functional)
- **Gaps**: Garbage collection, full JDK compatibility, JIT compilation, complete concurrency
- **Threading**: Only 2 threads - Rust main thread mapped to Java main thread, and tokio async runtime for JDWP agent. Monitor opcodes are currently no-op.
- **Milestone**: "Hello World" execution as primary goal
- **API Stability**: The API changes frequently (almost each commit) as the project evolves

## Important Constraints

- **Educational Focus**: Code clarity and correctness over performance optimization
- **Incremental Development**: Features implemented progressively with working intermediate states
- **Specification Compliance**: Adherence to JVM spec where possible, with clear deviations documented
- **No Production Use**: Experimental implementation not intended for production workloads
- **JDK Dependency**: Requires Java 25 JDK for test compilation and jimage access

## External Dependencies

### Java Dependencies

- **JDK 25.0.1**: Required for test compilation and jimage file access
- **javac**: Compiles test Java sources into .class files
- **javap**: Reference implementation for output comparison (jclass/javap testing)

### Rust Dependencies (Workspace)

- `tracing = { version = "0.1", features = ["attributes", "log", "max_level_debug", "release_max_level_info"] }`
- `tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }` - used for logging only
- `tracing-log = "0.2"`
- `hotpath = "0.8.0"` - experimental, not heavily used

### Crate-Specific Dependencies

See each crate section below for detailed dependency information and purpose.

## Workspace Structure

### Dependency Graph

```
workspace (lagertha-vm)
├── common (foundation: error types, parsing utilities, logging)
│   ├── jclass (Java .class file parser with pretty_print feature)
│   ├── jimage (Java module system jimage reader)
│   ├── runtime (core JVM: interpreter, memory, debugging)
│   ├── vm (CLI interface and VM launcher)
│   └── javap (Java disassembler tool)
├── jclass → runtime, javap
├── jimage → runtime
└── runtime → vm
```

### Crate Navigation

- **Cross-Crate References**: Use `../crate-name` paths (e.g., `../jclass/src/lib.rs:42`)
- **Source Organization**: Each crate has clear `src/` module hierarchy
- **Test Locations**: Unit tests embedded in source files, integration tests in `tests/` directories

### Development Workflow

1. **Local Testing**: `cargo test` in individual crates or root for workspace testing
2. **Fixture Preparation**: Run `prepare_fixtures.py` to compile test Java classes
3. **Snapshot Updates**: `cargo insta review` to update snapshots after intentional changes
4. **Feature Testing**: `cargo hack` for comprehensive feature combination validation
5. **CI Validation**: GitHub Actions runs full matrix on push/PR

---

# Crate Details

## common

### Purpose

Foundation crate providing shared utilities, error types, parsing helpers, and logging infrastructure for the Rust Java VM workspace. This crate establishes common patterns and types used by all other crates in the workspace.

### Tech Stack

- **Core Dependencies**: `num_enum` (0.7.4) for safe enum conversions
- **Workspace Dependencies**: `tracing-subscriber` for logging integration (logging only, not telemetry)
- **No External Runtime Dependencies**: Pure utility crate focused on type definitions and parsing

### Workspace Context

- **This Crate Role**: Foundational layer - all other crates depend on `common`
- **Internal Dependencies**: None (base layer)
- **Internal Dependents**: `jclass`, `jimage`, `runtime`, `vm`, `javap` (ALL other crates)
- **Navigation**: Other crates reference this via `common::` namespace (e.g., `common::error::ClassFormatErr`)

### Conventions

#### Error Handling Pattern

- **Hierarchical Error Enums**: Structured error types in `error.rs` with clear categorization
- **From<T> Implementations**: Extensive trait implementations for seamless error conversion between layers
- **Error Categories**:
  - `SignatureErr`, `TypeDescriptorErr`, `MethodDescriptorErr` - parsing errors
  - `InstructionErr` - bytecode instruction errors
  - `LinkageError`, `RuntimePoolError` - runtime linking errors
  - `ClassFormatErr` - class file validation errors

**Note**: All error types are provisional and will be refactored in the near future as the project evolves.

#### Parsing Utilities

- **ByteCursor**: Binary parsing with big/little endian support and error tracking
- **Recursive Parsing**: `try_recursive()` pattern for nested structure parsing
- **Descriptor Parsing**: Complete JVM type descriptor and signature parsing
- **Validation**: Early error detection with detailed error messages

#### Type System

- **JavaType Enum**: Comprehensive Java type representation (primitives, objects, arrays, generics)
- **Instruction Definitions**: Complete JVM opcode set with metadata (byte size, branching behavior)
- **Allocation Information**: Memory layout details for runtime allocation planning

#### Pretty Printing

- **IndentWriter**: Utilities for formatted output with consistent indentation
- **Macro Support**: `pretty_try!` and `pretty_class_name_try!` macros for error-handling during display
- **Logging Integration**: `init_tracing()` for structured logging setup

### Testing Approach

- **Unit Tests**: Embedded in each module (`#[cfg(test)]`) focusing on parsing validation
- **Parsing Validation**: Extensive tests for descriptor and signature parsing edge cases
- **Error Coverage**: Tests verify correct error propagation through From trait conversions
- **No Integration Tests**: Pure utility crate tested through dependent crates' usage

### Module Structure

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
    └── logging.rs (tracing setup)
```

### Important Constraints

- **Stability Critical**: Changes affect ALL other crates - must maintain backward compatibility
- **Minimal Dependencies**: Avoid adding dependencies that would bloat dependent crates
- **Performance Neutral**: Utilities should not introduce significant overhead
- **Clear Documentation**: Type definitions must be well-documented as they're public API

---

## jclass

### Purpose

Complete **Java 25** `.class` file parser implementing JVM Specification SE 25 Chapter 4. Provides structured representation of class files with validation, constant pool resolution, and pretty-printing capabilities similar to `javap -v -p`.

### Tech Stack

- **Core Dependencies**: `num_enum` (0.7.4), `itertools` (0.14.0)
- **Optional Dependency**: `either` (1.15.0) only under `pretty_print` feature
- **Internal Dependency**: `common` (path) for error types and parsing utilities
- **Feature Flags**: `pretty_print` enables formatted Display implementation

### Workspace Context

- **This Crate Role**: Class file parsing layer - converts binary .class files to structured Rust types
- **Internal Dependencies**: `common` (foundational utilities)
- **Internal Dependents**: `runtime` (execution), `javap` (disassembly tool)
- **Navigation**: Reference `common` via `common::` namespace; dependents use `jclass::ClassFile`

### Conventions

#### Class File Representation

- **Immutable Structure**: `ClassFile` and all components are immutable after parsing
- **Complete Coverage**: Implements all Java 25 class file structures per specification
- **Constant Pool**: Type-safe access with validation of constant types
- **Attribute Hierarchy**: Separate attribute types for class, field, and method contexts

#### Parsing Architecture

- **TryFrom Pattern**: `ClassFile::try_from(Vec<u8>)` for binary parsing with validation
- **Magic Number Validation**: Enforces 0xCAFEBABE magic number check
- **Constant Pool Handling**: Properly skips double-width entries (Long/Double)
- **Trailing Byte Detection**: Validates no extra bytes after parsing completes

#### Pretty Printing System

- **Feature-Gated**: `pretty_print` feature enables comprehensive Display implementations
- **Internal javap Tool Compatibility**: Output designed to match Oracle's `javap -v -p` format for comparison with the internal `javap` crate
- **Structured Formatting**: Consistent column widths, comments, and indentation
- **Generic Support**: Displays generic signatures when Signature attribute present

#### Error Handling

- **ClassFormatErr**: Comprehensive error enum covering all parsing failure modes
- **Type Validation**: Constant pool entry type checking with helpful error messages
- **Error Conversion**: Leverages `common::error` hierarchy with From trait implementations

### Testing Approach

- **Minimal Unit Tests**: Basic parsing tests for individual structures
- **Integration Validation**: Primary validation through comparison with Oracle's `javap -v -p` output via the internal `javap` crate
- **No Dedicated Unit Test Suite**: The `jclass` crate currently has minimal unit tests and is primarily validated through the `javap` crate's integration tests
- **Java 25 Focus**: Currently targets Java 25 class file format

### Module Structure

```
src/
├── lib.rs (ClassFile definition and parsing)
├── flags.rs (access flag definitions and pretty printing)
├── constant/
│   ├── mod.rs (ConstantInfo enum)
│   └── pool.rs (ConstantPool struct)
├── field.rs (FieldInfo parsing and display)
├── method.rs (MethodInfo parsing and display)
├── attribute/
│   ├── mod.rs (shared attribute types)
│   ├── class.rs (class-level attributes)
│   ├── field.rs (field attributes)
│   └── method.rs (method attributes)
└── print.rs (pretty printing, under pretty_print feature)
```

### Important Constraints

- **Specification Compliance**: Must correctly parse valid Java 25 class files
- **Error Recovery**: Should fail gracefully with helpful error messages for invalid input
- **Performance**: Efficient parsing suitable for runtime class loading
- **Memory Usage**: Avoid excessive copying; use references to original data where possible
- **Feature Isolation**: `pretty_print` should not affect parsing performance or memory layout

---

## jimage

### Purpose

Java module system `jimage` file reader for Java 9+ runtime images. Provides memory-mapped access to Java runtime modules (like `/java.base`) for efficient class loading without filesystem extraction. Implements the jimage format specification for locating and extracting resources from Java runtime images.

**Note**: The jimage format is not officially documented by Oracle. This implementation is based on publicly available sources and AI-assisted analysis.

### Tech Stack

- **Core Dependency**: `memmap2` (0.9.8) for memory-mapped file access
- **Internal Dependency**: `common` (path) for error types and utilities
- **Minimal Dependencies**: Focused crate with single responsibility

### Workspace Context

- **This Crate Role**: System class loading layer - reads JDK classes from Java runtime images
- **Internal Dependencies**: `common` (error types)
- **Internal Dependents**: `runtime` (for bootstrap class loading)
- **Navigation**: Reference `common` via `common::` namespace; used by `runtime` for system classes

### Conventions

#### Memory-Mapped Architecture

- **Zero-Copy Access**: Uses `memmap2` to map jimage files directly into memory
- **Efficient Lookup**: Implements jimage hash algorithm for O(1) resource location
- **Resource Extraction**: Reads resource data directly from mapped memory regions
- **Lazy Loading**: Only accesses resources when requested, not at file open

#### JImage Format Implementation

- **Header Validation**: Checks magic number (0xCAFEDADA) and version compatibility
- **Table Parsing**: Reads redirect, offsets, and locations tables from file header
- **Hash Algorithm**: Implements jimage specific hash for resource path lookup
- **Resource Access**: Provides clean API for opening resources by module/path

#### Error Handling

- **File System Errors**: Handles missing files, permission issues, corrupt headers
- **Format Validation**: Validates jimage structure with helpful error messages
- **Resource Not Found**: Returns appropriate error when resource doesn't exist
- **Error Conversion**: Uses `common::error` types where appropriate

### Testing Approach

- **Integration Focus**: Tests with real jimage files from JDK installation
- **System Dependency**: Requires JDK installation with jimage files
- **Resource Validation**: Verifies correct extraction of known resources (e.g., `java/lang/String.class`)
- **Error Cases**: Tests invalid jimage files, missing resources, corrupt headers
- **Environment Awareness**: Respects `JAVA_HOME` environment variable for test data

### Module Structure

```
src/
└── lib.rs (single module with JImage struct and public API)
```

### Important Constraints

- **System Dependency**: Requires JDK installation with jimage files for full functionality
- **Platform Specific**: Memory mapping behavior may vary by operating system
- **Read-Only**: jimage files are read-only; no writing or modification supported
- **JDK Version**: Tied to specific Java version's jimage format
- **Safety Critical**: Must ensure safe memory access patterns to avoid undefined behavior

---

## runtime

### Purpose

Core JVM implementation providing class loading, memory management, bytecode interpretation, native method registration, and JDWP debugging support. This crate is the heart of the Java Virtual Machine where bytecode execution happens.

**Note**: JDWP debugging is in early stage and does not work yet.

### Tech Stack

- **Core Dependencies**: `once_cell`, `libc`, `lasso`, `dashmap`, `tokio` (sync/net/rt), `byteorder`, `num_enum`, `smallvec`, `itertools`, `walkdir`
- **Workspace Dependencies**: `hotpath` (experimental performance profiling, not heavily used or properly configured), `tracing-log` (structured logging)
- **Internal Dependencies**: `jclass`, `jimage`, `common` (all workspace crates)

### Workspace Context

- **This Crate Role**: Core execution engine - implements JVM specification for bytecode execution
- **Internal Dependencies**: `jclass` (class file parsing), `jimage` (module system), `common` (shared utilities)
- **Internal Dependents**: `vm` (CLI launcher)
- **Navigation**: Other crates reference this via `runtime::` namespace (e.g., `runtime::VirtualMachine`)

### Conventions

#### VirtualMachine Architecture

- **Main Struct**: `VirtualMachine` manages heap, method area, native registry, and debug state
- **VmConfig**: Configuration for heap size, classpath, JDWP port, Java version validation
- **Memory Management**: Heap using `mmap`, object allocation with headers, string interning via `lasso`
- **Threading Model**: Only 2 threads currently:
  1. Rust main thread mapped to Java main thread
  2. Tokio async runtime for JDWP agent
  - `dashmap` for concurrent data structures
  - **Monitor opcodes are currently no-op** - only the Rust main thread is mapped to Java main thread

#### Module Organization

```
src/
├── lib.rs                    # Main entry point, VirtualMachine struct, VmConfig
├── class_loader/            # System class loader with JImage support
├── heap/                    # Memory management, object allocation, arrays
│   └── method_area.rs       # Method area implementation
├── rt/                      # Runtime representations
│   ├── class.rs            # Class runtime representation
│   ├── method.rs           # Method runtime representation  
│   ├── field.rs            # Field runtime representation
│   ├── interface.rs        # Interface runtime representation
│   ├── array.rs            # Array operations
│   └── constant_pool/      # Runtime constant pool
├── interpreter/            # Bytecode execution engine
│   ├── handlers.rs         # Instruction handlers
│   └── return_handlers.rs  # Method return handling
├── native/                 # Native method implementations
│   ├── preregistered/      # Built-in JDK native methods
│   └── registrable/        # Registerable native methods
├── thread/                 # Thread state management
├── vm/                     # VM core types
│   ├── bootstrap_registry.rs # Bootstrap class registry
│   ├── stack.rs           # Stack frame management
│   └── throw.rs           # Exception throwing
├── jdwp/                   # Java Debug Wire Protocol agent (early stage, not functional)
├── error.rs               # Comprehensive error handling
├── keys.rs                # ID types (MethodId, ClassId, etc.)
└── log_traces.rs          # Debug logging utilities
```

#### Feature Flags

- **`log-runtime-traces`**: Enables detailed execution tracing via `debug_log!` macros
- **`hotpath`**: Experimental performance measurement for interpreter hot paths (not heavily used, not properly configured)
- **`hotpath-alloc`**: Experimental performance measurement for allocation paths (not heavily used)
- **`hotpath-off`**: Disables all hotpath instrumentation

#### Error Handling

- **JvmError**: Main error type with variants for different failure modes
- **JavaExceptionFromJvm**: Conversion from JVM errors to Java exceptions
- **Comprehensive Coverage**: Class loading, bytecode interpretation, native method, memory allocation errors

### Testing Approach

- **Integration Focus**: Tested through `vm` crate integration tests rather than unit tests
- **Debug Logging**: `log-runtime-traces` feature enables detailed execution tracing for debugging
- **Error Scenarios**: Comprehensive error handling tested through integration test error cases
- **No Dedicated Test Directory**: Relies on higher-level testing in dependent crates
- **API Stability**: Frequent API changes (almost each commit) as the project evolves

### Important Constraints

#### JVM Specification Compliance

- **Java 25 Target**: Must comply with JVM Specification SE 25
- **Bytecode Verification**: Must reject invalid bytecode per specification
- **Class File Validation**: Strict adherence to class file format constraints

#### Memory Safety

- **Rust Safety Guarantees**: Must maintain memory safety while implementing JVM semantics
- **Concurrent Access**: Thread-safe data structures for shared runtime state
- **Resource Management**: Proper cleanup of allocated memory and system resources

#### Current Limitations

- **No Garbage Collection**: Memory is allocated but not collected yet
- **Limited Threading**: Only Rust main thread mapped to Java main thread; monitor opcodes are no-op
- **JDWP Not Functional**: Early stage debugging support, not working yet

---

## vm

### Purpose

Command-line interface that parses arguments, configures the runtime environment, and launches the JVM. Acts as the main entry point for running Java programs, providing integration testing for the entire Java VM workspace.

### Tech Stack

- **Core Dependencies**: `clap` (with derive feature) for command-line argument parsing
- **Workspace Dependencies**: `hotpath` (experimental performance profiling, not heavily used), `tracing-log` (structured logging)
- **Internal Dependencies**: `runtime` (core JVM), `common` (shared utilities)
- **Testing Dependencies**: `assert_cmd`, `insta`, `rstest` for integration testing

### Workspace Context

- **This Crate Role**: CLI launcher and integration test harness - exercises the full JVM stack
- **Internal Dependencies**: `runtime` (core execution engine), `common` (shared utilities)
- **Internal Dependents**: None (top-level executable)
- **Navigation**: References other crates via `runtime::` and `common::` namespaces

### Conventions

#### CLI Architecture

- **Args Struct**: Uses `clap::Parser` with basic argument parsing
- **Limited Args**: Currently only supports class-path location; too early in development to add more arguments
- **Classpath Resolution**: Supports both dotted and slash-separated class names
- **Java Home Detection**: Reads `JAVA_HOME` environment variable and `release` file (used to assert Java version is 25)
- **Configuration Building**: `create_vm_configuration()` builds `runtime::VmConfig` from CLI args

#### Build System Integration

- **Build Script**: `build.rs` compiles Java test fixtures from `tests/testdata/java` to `tests/testdata/compiled`
- **Test Fixture Management**: Java source files compiled on-the-fly for integration testing
- **Snapshot Management**: Test outputs compared against stored snapshots in `../snapshots/`

#### Feature Flags

- **`log-runtime-traces`**: Propagates to `runtime` crate for execution tracing
- **`hotpath`**: Experimental performance measurement for VM launch paths (not heavily used)
- **`hotpath-alloc`**: Experimental performance measurement for allocation paths (not heavily used)
- **`hotpath-off`**: Disables all hotpath instrumentation

### Testing Approach

#### Integration Testing Philosophy

- **End-to-End Testing**: Tests the complete JVM stack from CLI invocation to program execution
- **Snapshot Verification**: Compares VM output against expected snapshots using `insta`
- **Parameterized Testing**: Uses `rstest` to test multiple test cases with different fixtures
- **Error Case Coverage**: Separate test suites for successful execution and error scenarios

#### Test Categories

1. **Non-Error Cases** (`non_error_cases`): Tests successful program execution
   - Matches files named `*OkMain.class` in compiled test fixtures
   - Compares stdout/stderr against snapshots
   - Verifies exit code 0

2. **Error Cases** (`error_cases`): Tests error handling and failure modes
   - Matches files named `*ErrMain.class` in compiled test fixtures
   - Verifies non-zero exit codes
   - Tests error message formatting

#### Testing Structure

```
tests/
├── integration_test.rs          # Main integration test suite
├── testdata/
│   ├── java/                   # Java source fixtures
│   └── compiled/              # Compiled .class files (generated)
└── (snapshots in ../snapshots) # Expected output snapshots
```

### Important Constraints

- **CLI Compatibility**: Must match expected JVM command-line interface patterns
- **Error Messages**: Should provide helpful diagnostics for common configuration errors
- **Exit Codes**: Must follow Unix conventions (0 for success, non-zero for errors)
- **Deterministic Output**: Tests must produce consistent output for snapshot verification
- **Environment Independence**: Should work with any valid JAVA_HOME installation

---

## javap

### Purpose

Java class file disassembler tool that prints the structure of `.class` files (equivalent to `javap -v -p`). Demonstrates and tests the `jclass` crate's parsing capabilities while providing a useful standalone utility for examining Java bytecode. Targets **Java 25** class file format.

### Tech Stack

- **Core Dependencies**: `jclass` (with `pretty_print` feature), `sha2` (SHA-256 checksums), `chrono` (timestamp formatting)
- **Build Dependencies**: `serde`, `toml`, `tempfile` (fixture configuration parsing)
- **Dev Dependencies**: `assert_cmd`, `insta`, `rstest` (integration testing)

### Workspace Context

- **This Crate Role**: Tool and test harness - validates `jclass` crate against Oracle's javap
- **Internal Dependencies**: `jclass` (class file parsing with `pretty_print` feature)
- **Internal Dependents**: None (standalone tool)
- **Navigation**: References `jclass::` namespace for class file parsing and display

### Conventions

#### Tool Architecture

- **Class File Processing**: Reads `.class` files, computes SHA-256 checksum, extracts metadata
- **Path Resolution**: Handles both dotted (`java.lang.Object`) and slash-separated (`java/lang/Object`) class names
- **Pretty Printing**: Uses `jclass` crate's `Display` implementation for formatted output
- **Metadata Display**: Shows file size, modification timestamp, SHA-256 checksum, source file attribute

#### Build System Integration

- **Test Fixture Preparation**: `build.rs` (located in `tests/` folder) extracts JDK module class files from `.jmod` archives based on `fixtures.toml` manifest **solely for test purposes**
- **Fixture Manifest**: `tests/testdata/fixtures.toml` defines which classes to extract from JDK modules (located in the integration tests folder)
- **JMOD Extraction**: Uses `jmod` tool to extract classes from JDK module archives

### Testing Approach

#### Integration Testing Philosophy

- **Oracle Comparison**: Compares output line-by-line with Oracle's `javap -v -p` (whitespace normalized)
- **Comprehensive Coverage**: Tests all class files defined in `fixtures.toml` manifest
- **No Snapshot Storage**: Uses direct comparison with Oracle javap output instead of snapshot files. Oracle's `javap` is expected to be deterministic, so snapshots are not needed.
- **Parameterized Testing**: Uses `rstest` to test all `.class` files in compiled fixtures directory

#### Test Execution

1. **Fixture Extraction**: Build script extracts classes from JDK `.jmod` files
2. **Tool Execution**: Runs `javap` tool on each extracted `.class` file
3. **Oracle Comparison**: Runs Oracle `javap -v -p` on same file
4. **Line-by-Line Verification**: Compares normalized output (whitespace removed)
5. **Error Reporting**: Detailed mismatch reporting with line numbers and actual content

#### Testing Structure

```
tests/
├── integration_test.rs          # Comparison with Oracle javap
├── testdata/
│   ├── fixtures.toml           # Class manifest for extraction (in test folder)
│   └── compiled/              # Extracted .class files (generated)
└── (no snapshots)              # Line-by-line comparison with Oracle javap (deterministic output)
```

### Important Constraints

- **Output Format**: Must match `javap -v -p` output format (excluding whitespace differences)
- **Error Handling**: Should match javap exit codes and error messages for missing files
- **Path Resolution**: Must support both dotted and slash-separated class name formats
- **JDK Dependency**: Requires JDK 9+ with JMOD files for fixture extraction
- **Environment Setup**: Depends on `JAVA_HOME` environment variable
