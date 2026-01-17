# Project Context

## Purpose
Java class file disassembler tool that prints the structure of `.class` files (equivalent to `javap -v -p`). Demonstrates and tests the `jclass` crate's parsing capabilities while providing a useful standalone utility for examining Java bytecode. Targets Java 25 class file format.

## Tech Stack
- **Core Dependencies**: `jclass` (with `pretty_print` feature), `sha2` (SHA-256 checksums), `chrono` (timestamp formatting)
- **Build Dependencies**: `serde`, `toml`, `tempfile` (fixture configuration parsing)
- **Dev Dependencies**: `assert_cmd`, `insta`, `rstest` (integration testing)

## Workspace Context
- **This Crate Role**: Tool and test harness - validates `jclass` crate against Oracle's javap
- **Internal Dependencies**: `jclass` (class file parsing with `pretty_print` feature)
- **Internal Dependents**: None (standalone tool)
- **Navigation**: References `jclass::` namespace for class file parsing and display

## Crate-Specific Conventions

### Tool Architecture
- **Class File Processing**: Reads `.class` files, computes SHA-256 checksum, extracts metadata
- **Path Resolution**: Handles both dotted (`java.lang.Object`) and slash-separated (`java/lang/Object`) class names
- **Pretty Printing**: Uses `jclass` crate's `Display` implementation for formatted output
- **Metadata Display**: Shows file size, modification timestamp, SHA-256 checksum, source file attribute

### Build System Integration
- **Test Fixture Preparation**: `build.rs` (located in `tests/` folder) extracts JDK module class files from `.jmod` archives based on `fixtures.toml` manifest solely for test purposes
- **Fixture Management**: `tests/testdata/fixtures.toml` defines which classes to extract from JDK modules
- **JMOD Extraction**: Uses `jmod` tool to extract classes from JDK module archives

### Testing Strategy
```
tests/
├── integration_test.rs          # Comparison with Oracle javap
├── testdata/
│   ├── fixtures.toml           # Class manifest for extraction
│   └── compiled/              # Extracted .class files (generated)
└── (no snapshots)              # Line-by-line comparison with Oracle javap (deterministic output)
```

### Feature Flags
- **No Crate-Specific Features**: Relies on `jclass` crate's `pretty_print` feature
- **JClass Dependency**: Always uses `jclass` with `pretty_print` feature enabled

## Testing Approach

### Integration Testing Philosophy
- **Oracle Comparison**: Compares output line-by-line with Oracle's `javap -v -p` (whitespace normalized)
- **Comprehensive Coverage**: Tests all class files defined in `fixtures.toml` manifest
- **No Snapshot Storage**: Uses direct comparison with Oracle javap output (deterministic) instead of snapshot files to avoid drift
- **Parameterized Testing**: Uses `rstest` to test all `.class` files in compiled fixtures directory

### Test Execution
1. **Fixture Extraction**: Build script extracts classes from JDK `.jmod` files
2. **Tool Execution**: Runs `javap` tool on each extracted `.class` file
3. **Oracle Comparison**: Runs Oracle `javap -v -p` on same file
4. **Line-by-Line Verification**: Compares normalized output (whitespace removed)
5. **Error Reporting**: Detailed mismatch reporting with line numbers and actual content

### Fixture Management
- **Manifest File**: `tests/testdata/fixtures.toml` defines classes to extract per JDK module
- **JMOD Source**: Extracts from `$JAVA_HOME/jmods/*.jmod` files
- **Extraction Process**: Uses `jmod extract` command to temporarily extract classes
- **Output Location**: `tests/testdata/compiled/` directory (generated, not committed)

### Comparison Strategy
- **Normalization**: Removes all whitespace before comparison (handles formatting differences)
- **Line Alignment**: Compares corresponding lines between tool and Oracle javap
- **Error Context**: Reports file path, line number, and actual line content on mismatch

## Domain Knowledge Required

### Class File Format
- **Constant Pool Structure**: Understanding of JVM constant pool entries and references
- **Attribute Parsing**: Knowledge of standard and custom class file attributes
- **Bytecode Display**: Familiarity with JVM instruction formatting and operand display
- **Access Flags**: Interpretation of class, method, and field access modifiers

### JDK Module System
- **JMOD Archives**: Understanding of JDK 9+ module archive format
- **Module Layout**: Knowledge of `classes/` directory structure within JMOD files
- **Extraction Tools**: Familiarity with `jmod` command-line tool for extraction

### Testing Patterns
- **Oracle Tool Comparison**: Strategy for comparing tool output against reference implementation
- **Whitespace Normalization**: Techniques for handling formatting differences in tool output
- **Fixture Management**: Approaches to managing test data extracted from external sources

## Important Constraints

### Tool Compatibility
- **Output Format**: Must match `javap -v -p` output format (excluding whitespace differences)
- **Error Handling**: Should match javap exit codes and error messages for missing files
- **Path Resolution**: Must support both dotted and slash-separated class name formats

### Testing Reliability
- **JDK Dependency**: Requires JDK 9+ with JMOD files for fixture extraction
- **Environment Setup**: Depends on `JAVA_HOME` environment variable
- **Tool Availability**: Requires `jmod` command in PATH for fixture extraction

### Performance
- **File Reading**: Efficient reading and parsing of potentially large `.class` files
- **SHA-256 Computation**: Checksum calculation should not dominate tool execution time
- **Fixture Extraction**: Build-time extraction should not significantly slow down development

## External Dependencies

### Runtime Dependencies
- **`jclass = { path = "../jclass", features = ["pretty_print"] }`**: Class file parsing with pretty printing
- **`sha2 = "0.10.9"`**: SHA-256 checksum computation for class files
- **`chrono = "0.4"`**: Timestamp formatting for file modification times

### Build Dependencies
- **`serde = { version = "1", features = ["derive"] }`**: TOML parsing for fixture manifest
- **`toml = "0.8"`**: TOML file format support
- **`tempfile = "3"`**: Temporary directory management for JMOD extraction

### Dev Dependencies
- **`assert_cmd = "2"`**: Command output assertion
- **`insta = { version = "1.42.2", features = ["yaml"] }`**: Snapshot testing (unused in current tests)
- **`rstest = "0.26.1"`**: Parameterized testing

## Usage Examples

### Tool Usage
```bash
# Basic usage with dotted class name
./javap java.lang.Object

# With slash-separated class name  
./javap java/lang/Object

# Output includes metadata and formatted class structure
# Classfile /path/to/Object.class
#   Last modified Jan 16, 2025; size 1234 bytes
#   SHA-256 checksum abc123...
#   Compiled from "Object.java"
# [formatted class output]
```

### Integration Test Patterns
```rust
#[rstest]
fn compare_with_javap(
    #[base_dir = "tests/testdata/compiled"]
    #[files("**/*.class")]
    path: PathBuf,
) {
    // Execute our javap tool
    let mut cmd = Command::cargo_bin("javap").unwrap();
    cmd.arg(&path);
    let output = cmd.assert().success().get_output().clone();
    let my_output = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    
    // Execute Oracle javap
    let javap_output = std::process::Command::new("javap")
        .arg("-v")
        .arg("-p")
        .arg(&path)
        .output()
        .unwrap()
        .stdout
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    
    // Compare normalized lines
    for (i, (my, javap)) in my_output.iter().zip(javap_output.iter()).enumerate() {
        let my_line_normalized: String = my.chars().filter(|c| !c.is_whitespace()).collect();
        let javap_line_normalized: String = javap.chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!(
            my_line_normalized,
            javap_line_normalized,
            "Mismatch at line {} of file {:?}",
            i + 1,
            path
        );
    }
}
```

### Fixture Manifest Example
```toml
# tests/testdata/fixtures.toml
[modules."java.base"]
classes = [
    "java.lang.Object",
    "java.lang.String",
    "java.lang.Class",
    # ... many more classes
]

[modules."java.desktop"]
classes = [
    "java.awt.Component",
    # ... desktop classes
]
```

### Build Script Integration
```rust
// build.rs - Extracts classes from JDK JMOD files
fn extract_from_jmods(fixtures: &Fixtures, jmods_dir: &Path, out_root: &Path) {
    for (module, payload) in &fixtures.modules {
        let jmod_path = jmods_dir.join(format!("{}.jmod", module));
        
        // Extract JMOD to temporary directory
        let status = Command::new("jmod")
            .args(["extract", "--dir"])
            .arg(&extract_dir)
            .arg(&jmod_path)
            .status()
            .expect("failed to run jmod");
        
        // Copy specified classes to output directory
        for fqn in &payload.classes {
            let rel = fqn_to_rel(fqn); // Converts "java.lang.Object" to "java/lang/Object.class"
            let src = classes_root.join(&rel);
            let dst = out_root.join(&rel);
            fs::copy(&src, &dst).unwrap();
        }
    }
}
```