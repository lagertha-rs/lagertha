# Project Context

## Purpose
Complete Java 25 `.class` file parser implementing JVM Specification SE 25 Chapter 4. Provides structured representation of class files with validation, constant pool resolution, and pretty-printing capabilities similar to `javap -v -p`.

## Tech Stack
- **Core Dependencies**: `num_enum` (0.7.4), `itertools` (0.14.0)
- **Optional Dependency**: `either` (1.15.0) only under `pretty_print` feature
- **Internal Dependency**: `common` (path) for error types and parsing utilities
- **Feature Flags**: `pretty_print` enables formatted Display implementation

## Workspace Context
- **This Crate Role**: Class file parsing layer - converts binary .class files to structured Rust types
- **Internal Dependencies**: `common` (foundational utilities)
- **Internal Dependents**: `runtime` (execution), `javap` (disassembly tool)
- **Navigation**: Reference `common` via `common::` namespace; dependents use `jclass::ClassFile`

## Crate-Specific Conventions

### Class File Representation
- **Immutable Structure**: `ClassFile` and all components are immutable after parsing
- **Complete Coverage**: Implements all Java 25 class file structures per specification
- **Constant Pool**: Type-safe access with validation of constant types
- **Attribute Hierarchy**: Separate attribute types for class, field, and method contexts

### Parsing Architecture
- **TryFrom Pattern**: `ClassFile::try_from(Vec<u8>)` for binary parsing with validation
- **Magic Number Validation**: Enforces 0xCAFEBABE magic number check
- **Constant Pool Handling**: Properly skips double-width entries (Long/Double)
- **Trailing Byte Detection**: Validates no extra bytes after parsing completes

### Pretty Printing System
- **Feature-Gated**: `pretty_print` feature enables comprehensive Display implementations
- **javap Compatibility**: Output designed to match `javap -v -p` format for comparison
- **Structured Formatting**: Consistent column widths, comments, and indentation
- **Generic Support**: Displays generic signatures when Signature attribute present

### Error Handling
- **ClassFormatErr**: Comprehensive error enum covering all parsing failure modes
- **Type Validation**: Constant pool entry type checking with helpful error messages
- **Error Conversion**: Leverages `common::error` hierarchy with From trait implementations

## Testing Approach
- **Unit Tests**: Embedded parsing tests for individual structures
- **Integration Tests**: Comparison against `javap -v -p` output for real class files
- **Snapshot Testing**: Pretty-printed output captured for regression testing
- **Fixture-Based**: Uses compiled Java test classes and JDK classes as test data
- **Cross-Version Testing**: Verification with multiple Java version class files

## Domain Knowledge Required
- **JVM Class File Format**: Understanding of Chapter 4 structures (magic, version, constant_pool_count, etc.)
- **Constant Pool Types**: All 17 constant types and their representations
- **Attribute System**: Standard and custom attributes, their locations and purposes
- **Access Flags**: Class, field, method, and inner class flag semantics
- **Generic Signatures**: Type parameter syntax and generic type representation
- **Annotation Format**: RuntimeVisible/InvisibleAnnotations structure and element values

## Important Constraints
- **Specification Compliance**: Must correctly parse valid Java 25 class files
- **Error Recovery**: Should fail gracefully with helpful error messages for invalid input
- **Performance**: Efficient parsing suitable for runtime class loading
- **Memory Usage**: Avoid excessive copying; use references to original data where possible
- **Feature Isolation**: `pretty_print` should not affect parsing performance or memory layout

## External Dependencies
- **num_enum = "0.7.4"**: For constant pool tag and attribute type enums
- **itertools = "0.14.0"**: Utility for complex iterations in pretty printing
- **either = "1.15.0"**: Only under `pretty_print` feature for display formatting
- **common** (path): Error types, parsing utilities, type definitions

## Module Structure
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

## Key Data Structures
- **ClassFile**: Complete class file representation with all components
- **ConstantPool**: Type-safe constant pool with validation and access methods
- **FieldInfo/MethodInfo**: Field and method metadata with attributes
- **ClassAttr/FieldAttribute/MethodAttribute**: Attribute type hierarchies

## Usage Examples
```rust
// Parse class file
let bytes = std::fs::read("MyClass.class")?;
let class_file = jclass::ClassFile::try_from(bytes)?;

// Access constant pool
let class_name = class_file.cp.get_class_name(&class_file.this_class)?;

// Pretty print (requires feature)
#[cfg(feature = "pretty_print")]
println!("{}", class_file);

// Iterate methods
for method in &class_file.methods {
    let name = class_file.cp.get_utf8(&method.name_index)?;
    println!("Method: {}", name);
}
```

## Integration Notes
- **Runtime Usage**: `runtime` crate uses `jclass` for class loading and parsing
- **javap Tool**: `javap` crate depends on `jclass` with `pretty_print` feature
- **Error Propagation**: Errors convert to `common::error::ClassFormatErr` for unified handling