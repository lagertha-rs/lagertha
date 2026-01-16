# Project Context

## Purpose
Educational and fun Rust implementation of a Java Virtual Machine (JVM) targeting Java 25 specification. This project explores JVM internals through pure Rust implementation, serving as both a learning resource and experimental platform for understanding Java runtime mechanics.

## Tech Stack
- **Language**: Rust 2024 edition
- **Build System**: Cargo workspace with 6 member crates
- **Telemetry**: Tracing ecosystem (tracing, tracing-subscriber, tracing-log)
- **Performance Profiling**: Hotpath for performance instrumentation
- **Async Runtime**: Tokio for JDWP debugging and future networking
- **Testing**: Insta (snapshot testing), rstest (parameterized testing), assert_cmd
- **Parsing Utilities**: num_enum, itertools, memmap2 (jimage)
- **Concurrency**: dashmap, lasso (string interning), smallvec

## Project Conventions

### Code Style
- **Error Handling**: Hierarchical error enums with extensive `From<T>` trait implementations for seamless error conversion
- **Parsing Patterns**: `TryFrom<&str>` for text parsing, `ByteCursor` for binary parsing with endianness support
- **Immutable Core**: Class file data structures are immutable after parsing
- **Feature Flags**: `pretty_print` for disassembly output, `hotpath-*` for profiling, `log-runtime-traces` for debugging
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
- **Memory Management**: Heap (objects), method area (metadata), string pool (interned strings)
- **Bytecode Semantics**: 200+ opcodes with specific stack and local variable effects
- **Native Integration**: JNI-like interface for native method invocation

### Development Status
- **Current**: Complete class file parsing, basic bytecode execution, JDWP framework
- **Gaps**: Garbage collection, full JDK compatibility, JIT compilation, complete concurrency
- **Milestone**: "Hello World" execution as primary goal

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
- `tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }`
- `tracing-log = "0.2"`
- `hotpath = "0.8.0"`

### Crate-Specific Dependencies
See each crate's `project.md` for detailed dependency information and purpose.

## Workspace Structure

### Dependency Graph
```
workspace (lagertha-vm)
├── common (foundation: error types, parsing utilities, telemetry)
│   ├── jclass (Java .class file parser with pretty_print feature)
│   ├── jimage (Java module system jimage reader)
│   ├── runtime (core JVM: interpreter, memory, threading, debugging)
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