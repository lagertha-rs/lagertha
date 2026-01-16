# Project Context

## Purpose
Java module system `jimage` file reader for Java 9+ runtime images. Provides memory-mapped access to Java runtime modules (like `/java.base`) for efficient class loading without filesystem extraction. Implements the jimage format specification for locating and extracting resources from Java runtime images.

## Tech Stack
- **Core Dependency**: `memmap2` (0.9.8) for memory-mapped file access
- **Internal Dependency**: `common` (path) for error types and utilities
- **Minimal Dependencies**: Focused crate with single responsibility

## Workspace Context
- **This Crate Role**: System class loading layer - reads JDK classes from Java runtime images
- **Internal Dependencies**: `common` (error types)
- **Internal Dependents**: `runtime` (for bootstrap class loading)
- **Navigation**: Reference `common` via `common::` namespace; used by `runtime` for system classes

## Crate-Specific Conventions

### Memory-Mapped Architecture
- **Zero-Copy Access**: Uses `memmap2` to map jimage files directly into memory
- **Efficient Lookup**: Implements jimage hash algorithm for O(1) resource location
- **Resource Extraction**: Reads resource data directly from mapped memory regions
- **Lazy Loading**: Only accesses resources when requested, not at file open

### JImage Format Implementation
- **Header Validation**: Checks magic number (0xCAFEDADA) and version compatibility
- **Table Parsing**: Reads redirect, offsets, and locations tables from file header
- **Hash Algorithm**: Implements jimage specific hash for resource path lookup
- **Resource Access**: Provides clean API for opening resources by module/path

### Error Handling
- **File System Errors**: Handles missing files, permission issues, corrupt headers
- **Format Validation**: Validates jimage structure with helpful error messages
- **Resource Not Found**: Returns appropriate error when resource doesn't exist
- **Error Conversion**: Uses `common::error` types where appropriate

## Testing Approach
- **Integration Focus**: Tests with real jimage files from JDK installation
- **System Dependency**: Requires JDK installation with jimage files
- **Resource Validation**: Verifies correct extraction of known resources (e.g., `java/lang/String.class`)
- **Error Cases**: Tests invalid jimage files, missing resources, corrupt headers
- **Environment Awareness**: Respects `JAVA_HOME` environment variable for test data

## Domain Knowledge Required
- **JImage Format**: Understanding of jimage file structure (header, tables, resources)
- **Java Module System**: Knowledge of Java 9+ modules and resource organization
- **Memory Mapping**: Concepts of memory-mapped files and safe access patterns
- **Hash Algorithms**: jimage-specific hash function for resource location
- **JDK Layout**: Understanding of JDK installation structure and module locations

## Important Constraints
- **System Dependency**: Requires JDK installation with jimage files for full functionality
- **Platform Specific**: Memory mapping behavior may vary by operating system
- **Read-Only**: jimage files are read-only; no writing or modification supported
- **JDK Version**: Tied to specific Java version's jimage format
- **Safety Critical**: Must ensure safe memory access patterns to avoid undefined behavior

## External Dependencies
- **memmap2 = "0.9.8"**: Safe memory-mapped file access with platform-specific implementations
- **common** (path): Error types and basic utilities

## Module Structure
```
src/
└── lib.rs (single module with JImage struct and public API)
```

## Key Data Structures
- **JImage**: Main struct providing memory-mapped access to jimage file
- **Header**: Parsed jimage header with table offsets and sizes
- **Resource Location**: Internal representation of resource position within file

## Public API
```rust
// Open jimage file
let jimage = JImage::open("/path/to/lib/modules")?;

// Open resource from java.base module
let resource = jimage.open_java_base_class("java/lang/String.class")?;

// Read resource data
let class_bytes = resource.read_all()?;
```

## Integration with Runtime
- **Bootstrap Loading**: `runtime` uses `jimage` to load core JDK classes (java.lang.*, etc.)
- **System Classpath**: Provides alternative to filesystem classpath for system classes
- **Performance**: Memory-mapped access faster than reading individual .class files
- **Dependency**: `runtime` depends on `jimage` but feature could be optional

## Compression Support
- **TODO**: Current implementation doesn't handle compressed resources in jimage
- **Future**: Would need to implement decompression for compressed resources

## Usage Example in Runtime
```rust
// In runtime class loader
fn load_system_class(&self, name: &str) -> Result<Vec<u8>, Error> {
    let jimage = self.jimage.as_ref().ok_or("No jimage available")?;
    jimage.open_java_base_class(&format!("{}.class", name.replace('.', "/")))
        .and_then(|res| res.read_all())
}
```

## Testing Considerations
- **Environment Variable**: Tests use `JAVA_HOME` to locate jimage files
- **Optional Tests**: Tests may be skipped if JDK not available
- **Cross-Platform**: Memory mapping works differently on Windows vs Unix
- **Safety**: Tests must validate safe access patterns and error handling