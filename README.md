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

- **jclass** - Library that parses and maps the binary representation of `.class` files to Rust structures
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

# Test data

## Description

In my opinion the tests right now are poorly organized. I want to improve that in the future. But right now
it is really important to have at least something, because the project is evolving fast and I want to be sure
that I don't break anything.

## Contents

- `prepare_fixtures.py`: A script to prepare all test fixtures. It is used for the CI pipeline and for local testing.
  It compiles all Java files, from all crates and organizes them into the appropriate directory structure. For the
  classes from `fixtures.toml` it extracts the required classes from the JDK. For the rest of java source files. It
  compiles all of them using `javac` with java 24.
- `jclass` tests reads all `.class` complied from all classes from `fixtures.toml`, all classes used in `runtime`
  tests, and all classes from `vm/tests`. It checks that it is parsed correctly. Right now it uses both snapshots
  and compare against `javap -v -p` output. In the future I want to remove one of them, because testing the same thing
  twice looks redundant, on the one hand snapshots are more predictable, on the other hand comparing against `javap` is
  more sure that the parsing is correct.
- `jclass/fixtures.toml`: A configuration file defining test fixtures from java standard libraries. Used only by
  `jclass` to test parsing of real-world class files. There is an assertion somewhere in `runtime` crate, whenever
  a jdk class is loaded, it is checked if it is present in this file. This is to ensure that the class file is correctly
  parsed and mapped to Rust structures.
- `runtime/testdata`: A directory containing Java source files for runtime tests. Each subdirectory represents a test
  case. This layer should cover a wide range of Java features and edge cases, taking a snapshots of the vm state after
  execution, and checking the heap, string pool and the top of the frame stack.
- `vm/tests`: Integration tests for the `vm` binary. These tests execute the `vm` binary with various Java classes
  and check the output against expected results.

## Usage

To prepare the test fixtures, run the `prepare_fixtures.py` script. This will compile the Java files and set up
the necessary directory structure for testing. The target directory for the compiled classes is `target/test-classes`.

## TODO:

- delete snapshots for custom cases without Main postfix
