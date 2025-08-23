# Toy Java Virtual Machine

This project is an educational implementation of a Java Virtual Machine following
the [Java Virtual Machine Specification](https://docs.oracle.com/javase/specs/jvms/se24/html/). It aims to provide a
simple but functional runtime for executing Java bytecode.

## Status

This project is currently under development. Executes nothing right now.

## Project Structure

This workspace consists of several crates:

- **class_file** - Library that parses and maps the binary representation of `.class` files to Rust structures
- **common** - Utility library with shared functionality used across the workspace
- **classp** - Binary tool similar to `javap -v` for inspecting class files
- **runtime** - Library implementing the virtual machine that executes Java bytecode
- **vm** - Binary application that launches the runtime

## Documentation

### Implementation Details

TODO

### References

- [JVM Specification SE 24](https://docs.oracle.com/javase/specs/jvms/se24/html/)
