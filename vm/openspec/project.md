# Project Context

## Purpose
Command-line interface that parses arguments, configures the runtime environment, and launches the JVM. Acts as the main entry point for running Java programs, providing integration testing for the entire Java VM workspace.

## Tech Stack
- **Core Dependencies**: `clap` (with derive feature) for command-line argument parsing
- **Workspace Dependencies**: `hotpath` (experimental performance profiling, not heavily used), `tracing-log` (structured logging)
- **Internal Dependencies**: `runtime` (core JVM), `common` (shared utilities)
- **Testing Dependencies**: `assert_cmd`, `insta`, `rstest` for integration testing

## Workspace Context
- **This Crate Role**: CLI launcher and integration test harness - exercises the full JVM stack
- **Internal Dependencies**: `runtime` (core execution engine), `common` (shared utilities)
- **Internal Dependents**: None (top-level executable)
- **Navigation**: References other crates via `runtime::` and `common::` namespaces

## Crate-Specific Conventions

### CLI Architecture
- **Args Struct**: Uses `clap::Parser` with basic argument parsing (currently only class-path support)
- **Classpath Resolution**: Supports both dotted and slash-separated class names
- **Java Home Detection**: Reads `JAVA_HOME` environment variable and `release` file (used to assert Java version 25)
- **Configuration Building**: `create_vm_configuration()` builds `runtime::VmConfig` from CLI args

### Build System Integration
- **Build Script**: `build.rs` compiles Java test fixtures from `tests/testdata/java` to `tests/testdata/compiled`
- **Test Fixture Management**: Java source files compiled on-the-fly for integration testing
- **Snapshot Management**: Test outputs compared against stored snapshots in `../snapshots/`

### Testing Strategy
```
tests/
├── integration_test.rs          # Main integration test suite
├── testdata/
│   ├── java/                   # Java source fixtures
│   └── compiled/              # Compiled .class files (generated)
└── (snapshots in ../snapshots) # Expected output snapshots
```

### Feature Flags
- **`log-runtime-traces`**: Propagates to `runtime` crate for execution tracing
- **`hotpath`**: Experimental performance measurement for VM launch paths (not heavily used)
- **`hotpath-alloc`**: Experimental performance measurement for allocation paths (not heavily used)
- **`hotpath-off`**: Disables all hotpath instrumentation

## Testing Approach

### Integration Testing Philosophy
- **End-to-End Testing**: Tests the complete JVM stack from CLI invocation to program execution
- **Snapshot Verification**: Compares VM output against expected snapshots using `insta`
- **Parameterized Testing**: Uses `rstest` to test multiple test cases with different fixtures
- **Error Case Coverage**: Separate test suites for successful execution and error scenarios

### Test Categories
1. **Non-Error Cases** (`non_error_cases`): Tests successful program execution
   - Matches files named `*OkMain.class` in compiled test fixtures
   - Compares stdout/stderr against snapshots
   - Verifies exit code 0

2. **Error Cases** (`error_cases`): Tests error handling and failure modes
   - Matches files named `*ErrMain.class` in compiled test fixtures
   - Verifies non-zero exit codes
   - Tests error message formatting

### Snapshot Management
- **Snapshot Location**: `../snapshots/` directory relative to crate root
- **Snapshot Naming**: Derived from test class path (e.g., `simple-OkMain`)
- **Snapshot Updates**: Use `cargo insta review` to update snapshots after intentional changes
- **Snapshot Verification**: Line-by-line comparison ignoring whitespace differences

### Test Fixture Compilation
- **Build Script**: `build.rs` compiles `.java` files to `.class` files
- **Java Source Location**: `tests/testdata/java/` directory
- **Compiled Output**: `tests/testdata/compiled/` directory (generated)
- **Compilation Command**: Uses `javac` from `JAVA_HOME` environment

## Domain Knowledge Required

### JVM Launch Protocol
- **Classpath Resolution**: Understanding of Java classpath semantics and directory structure
- **Main Class Specification**: Difference between dotted (com.example.Main) and slashed (com/example/Main) formats
- **Java Home Detection**: Reading JAVA_HOME and parsing release file for version information

### Integration Testing Patterns
- **Snapshot Testing**: Using `insta` for output verification and `cargo insta` for snapshot management
- **Parameterized Tests**: `rstest` patterns for testing multiple fixtures with the same test logic
- **Command Testing**: `assert_cmd` for testing CLI executable behavior

### Java Compilation
- **Test Fixture Design**: Creating Java programs that exercise specific JVM features
- **Compilation Process**: Understanding `javac` output locations and class file naming
- **Class Naming Conventions**: Using `OkMain`/`ErrMain` suffixes for test categorization

## Important Constraints

### CLI Compatibility
- **Argument Parsing**: Must match expected JVM command-line interface patterns
- **Error Messages**: Should provide helpful diagnostics for common configuration errors
- **Exit Codes**: Must follow Unix conventions (0 for success, non-zero for errors)

### Testing Reliability
- **Deterministic Output**: Tests must produce consistent output for snapshot verification
- **Environment Independence**: Should work with any valid JAVA_HOME installation
- **Fixture Management**: Generated class files must be kept in sync with source files

### Performance
- **Launch Overhead**: CLI parsing and configuration should be minimal
- **Test Execution**: Integration tests should run in reasonable time
- **Build Performance**: Java compilation should not significantly slow down test runs

## External Dependencies

### Runtime Dependencies
- **`clap = { version = "4.5.47", features = ["derive"] }`**: Command-line argument parsing

### Workspace Dependencies
- **`hotpath`**: Experimental performance profiling macros (not heavily used)
- **`tracing-log`**: Structured logging integration

### Internal Workspace Dependencies
- **`runtime`**: Core JVM implementation
- **`common`**: Shared utilities and error types

### Dev Dependencies
- **`assert_cmd = "2"`**: Command output assertion
- **`insta = { version = "1.42.2", features = ["yaml"] }`**: Snapshot testing
- **`rstest = "0.26.1"`**: Parameterized testing

### Build Dependencies
- **`walkdir = "2.5.0"`**: Filesystem traversal for test fixture discovery

## Usage Examples

### CLI Usage
```bash
# Basic usage with current directory as classpath
./vm com.example.Main

# Specify classpath
./vm -c /path/to/classes;/other/path com.example.Main

# Enable JDWP debugging on port 5005
./vm -j 5005 com.example.Main

# Use slash-separated class name
./vm com/example/Main
```

### Integration Test Patterns
```rust
// Parameterized test using rstest
#[rstest]
#[trace]
fn non_error_cases(
    #[base_dir = "tests/testdata/compiled"]
    #[files("**/*OkMain.class")]
    path: PathBuf,
) {
    // Test setup
    let class_path = current_dir.join("tests/testdata/compiled");
    let main_class_path = transform_absolute_path_to_package(&path);
    
    // Execute VM
    let mut cmd = Command::cargo_bin("vm").unwrap();
    cmd.arg("-c").arg(class_path).arg(&main_class_path);
    
    // Verify output against snapshot
    with_settings!({
        snapshot_path = DISPLAY_SNAPSHOT_PATH,
        prepend_module_to_snapshot = false,
    }, {
        insta::assert_debug_snapshot!(
            to_snapshot_name(&path),
            cmd.output().unwrap()
        );
    });
}
```

### Build Script Integration
```rust
// build.rs - Compiles Java test fixtures
fn main() {
    let java_src_dir = "tests/testdata/java";
    let compiled_dir = "tests/testdata/compiled";
    
    // Compile all .java files in java_src_dir to compiled_dir
    // Uses javac from JAVA_HOME environment
}
```