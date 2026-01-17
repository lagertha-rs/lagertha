# Project Context

## Purpose
Core JVM implementation providing class loading, memory management, bytecode interpretation, native method registration, and JDWP debugging support (JDWP is early stage and doesn't work yet). This crate is the heart of the Java Virtual Machine where bytecode execution happens.

## Tech Stack
- **Core Dependencies**: `once_cell`, `libc`, `lasso`, `dashmap`, `tokio` (sync/net/rt), `byteorder`, `num_enum`, `smallvec`, `itertools`, `walkdir`
- **Workspace Dependencies**: `hotpath` (experimental performance profiling, not heavily used), `tracing-log` (structured logging)
- **Internal Dependencies**: `jclass`, `jimage`, `common` (all workspace crates)

## Workspace Context
- **This Crate Role**: Core execution engine - implements JVM specification for bytecode execution
- **Internal Dependencies**: `jclass` (class file parsing), `jimage` (module system), `common` (shared utilities)
- **Internal Dependents**: `vm` (CLI launcher)
- **Navigation**: Other crates reference this via `runtime::` namespace (e.g., `runtime::VirtualMachine`)

## Crate-Specific Conventions

### VirtualMachine Architecture
- **Main Struct**: `VirtualMachine` manages heap, method area, native registry, and debug state
- **VmConfig**: Configuration for heap size, classpath, JDWP port, Java version validation
- **Memory Management**: Heap using `mmap`, object allocation with headers, string interning via `lasso`
- **Threading Model**: Only 2 threads (Rust main thread + tokio async JDWP agent); `dashmap` for concurrent data structures

### Module Organization
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
├── jdwp/                   # Java Debug Wire Protocol agent
├── error.rs               # Comprehensive error handling
├── keys.rs                # ID types (MethodId, ClassId, etc.)
└── log_traces.rs          # Debug logging utilities
```

### Feature Flags
- **`log-runtime-traces`**: Enables detailed execution tracing via `debug_log!` macros
- **`hotpath`**: Experimental performance measurement for interpreter hot paths (not heavily used)
- **`hotpath-alloc`**: Experimental performance measurement for allocation paths (not heavily used)
- **`hotpath-off`**: Disables all hotpath instrumentation

### Error Handling
- **JvmError**: Main error type with variants for different failure modes
- **JavaExceptionFromJvm**: Conversion from JVM errors to Java exceptions
- **Comprehensive Coverage**: Class loading, bytecode interpretation, native method, memory allocation errors

### Performance Considerations
- **String Interning**: `lasso` for symbol management
- **Concurrent Data Structures**: `dashmap` for shared method area and heap structures
- **Memory Layout**: Careful object header design to match JVM specification
- **Hot Path Optimization**: Feature-gated performance measurement for critical paths

## Testing Approach
- **Integration Focus**: Tested through `vm` crate integration tests rather than unit tests
- **Debug Logging**: `log-runtime-traces` feature enables detailed execution tracing for debugging
- **Error Scenarios**: Comprehensive error handling tested through integration test error cases
- **No Dedicated Test Directory**: Relies on higher-level testing in dependent crates
- **API Stability**: Frequent API changes (almost each commit) as the project evolves

## Domain Knowledge Required

### JVM Specification Compliance
- **Class Loading Process**: Loading, linking (verification, preparation, resolution), initialization
- **Bytecode Semantics**: Complete JVM instruction set with operand stack and local variable model
- **Memory Model**: Object header layout, array metadata, heap organization
- **Runtime Data Areas**: Method area, heap, stack, PC registers, native method stacks

### Java Internals
- **String Representation**: Latin-1 vs UTF-16 encoding, string interning
- **Native Methods**: JNI method registration and invocation
- **Exception Handling**: Throw/catch mechanics, exception table parsing
- **Debugging Protocol**: JDWP architecture and event handling

### Performance Considerations
- **Method Dispatch**: Virtual method lookup, interface method resolution
- **Garbage Collection**: No garbage collection yet; only basic allocation strategies
- **Thread Synchronization**: Monitor opcodes are currently no-op; only Rust main thread mapped to Java main thread

## Important Constraints

### JVM Specification Compliance
- **Java 25 Target**: Must comply with JVM Specification SE 25
- **Bytecode Verification**: Must reject invalid bytecode per specification
- **Class File Validation**: Strict adherence to class file format constraints

### Memory Safety
- **Rust Safety Guarantees**: Must maintain memory safety while implementing JVM semantics
- **Concurrent Access**: Thread-safe data structures for shared runtime state
- **Resource Management**: Proper cleanup of allocated memory and system resources

### Performance
- **Interpretation Overhead**: Bytecode interpreter should minimize overhead
- **Allocation Efficiency**: Object allocation must be reasonably performant
- **String Operations**: Efficient symbol interning and string comparison

## External Dependencies

### Runtime Dependencies
- **`once_cell = "1.19"**: One-time initialization of global state
- **`libc = "0.2.177"**: System calls for memory management (`mmap`, `munmap`)
- **`lasso = "0.7.3"**: String interning
- **`dashmap = "6.1.0"**: Concurrent hash maps for shared data structures
- **`tokio = { version = "1.48.0", features = ["sync", "net", "rt", "io-util", "macros"] }**: Async runtime for JDWP agent
- **`byteorder = "1.5"**, `num_enum = "0.7.4"`, `smallvec = "1.15.1"`, `itertools = "0.14.0"`, `walkdir = "2"**: Data manipulation utilities

### Workspace Dependencies
- **`hotpath`**: Experimental performance profiling macros (not heavily used)
- **`tracing-log`**: Structured logging integration

### Internal Workspace Dependencies
- **`jclass`**: Class file parsing and representation
- **`jimage`**: Java module system image access
- **`common`**: Shared error types, parsing utilities, type definitions

## Usage Examples

```rust
// Creating and configuring the VM
use runtime::{VirtualMachine, VmConfig};

let config = VmConfig {
    home: java_home_path,
    version: "25.0.1".to_string(),
    main_class: "com.example.Main".to_string(),
    class_path: vec![".".to_string()],
    initial_heap_size: 64 * 1024 * 1024, // 64MB
    max_heap_size: 256 * 1024 * 1024,    // 256MB
    frame_stack_size: 1024,
    jdwp_port: Some(5005),
};

let vm = VirtualMachine::new(config);
vm.start()?;

// Accessing runtime components
let heap = vm.heap();
let method_area = vm.method_area();
let native_registry = vm.native_registry();

// Debug logging (with log-runtime-traces feature)
debug_log!(vm, "Loading class {}", class_name);
```