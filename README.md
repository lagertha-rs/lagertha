# Toy Java Virtual Machine

This project is an educational implementation of a Java Virtual Machine following
the [Java Virtual Machine Specification](https://docs.oracle.com/javase/specs/jvms/se24/html/). It aims to provide a
simple but functional runtime for executing Java bytecode. I'm targeting fully featured Java 24 support.

## Status

This project is currently in early development. It executes a very limited instruction set, ignoring many important
aspects of the JVM specification.

The first milestone is to execute a "Hello, World!" program compiled with Java 24. It sounds easy, but it is actually
not.

## Project Structure

This workspace consists of several crates:

- **class_file** - Library that parses and maps the binary representation of `.class` files to Rust structures
- **common** - Utility library with shared functionality used across the workspace
- **classp** - Binary tool similar to `javap -v -p` for inspecting class files
- **runtime** - Library implementing the virtual machine that executes Java bytecode
- **vm** - Binary application that launches the runtime

## Documentation

### Implementation Details

TODO

### References

- [JVM Specification SE 24](https://docs.oracle.com/javase/specs/jvms/se24/html/)

### Launch CI locally

This project uses GitHub Actions for continuous integration. It is possible to run the CI pipeline locally
using [act](https://github.com/nektos/act.git)

When the act tool is installed, it is necessary to use the `large` image to have all dependencies available.

```bash
cat ~/.config/act/actrc
-P ubuntu-latest=catthehacker/ubuntu:full-latest
-P ubuntu-22.04=catthehacker/ubuntu:full-22.04
-P ubuntu-20.04=catthehacker/ubuntu:full-20.04
-P ubuntu-18.04=catthehacker/ubuntu:full-18.04
```

I use the default `large` image from act, which is called `catthehacker/ubuntu:full-latest`.

To launch the CI pipeline, in the project root execute:

```bash
act
```
