# Lagertha

Lagertha is an early-stage educational Java Virtual Machine implementation written in Rust. It incrementally targets
Java 25, using the Java SE 25 specifications as its source of truth.

The project focuses on understanding and implementing JVM behavior directly:
class-file processing, runtime linking, heap representation, bytecode execution, native integration, and observable Java
semantics. Lagertha is not yet broadly compatible with Java applications.

## Project Status

Support is capability-specific. A parser entry, interpreter arm, native registration, or stub does not by itself mean
that a feature is supported.

- [Feature support](docs/features/README.md) presents capabilities recorded in the feature registry, including declared
  scope, Java SE 25 references, and known limitations.
- [Integration test coverage](docs/features/TEST_COVERAGE.md) maps those capabilities to passing Lagertha and
  reference-JDK snapshot tests.

These generated reports cover the current registry, not the complete Java 25 feature set.

## Related Project

[Runestaff](https://github.com/lagertha-rs/runestaff) provides `rnsc`, which generates exact or intentionally invalid
class files for Lagertha integration tests.

## Build And Run

Lagertha currently requires JDK `25.0.1`, with `JAVA_HOME` pointing to that JDK, and `rnsc 0.2.1`. See
the [building guide](docs/BUILDING.md) for complete setup and CI-equivalent verification instructions.

```bash
cargo install rnsc --version 0.2.1 --locked
cargo build --workspace --locked
cargo run -p vm -- -c <class-directory> <package.Main>
```

Current launcher constraints:

- Classpath entries must be directories; JAR classpaths are not supported.
- Multiple classpath entries use `;` as separator, including on Unix.
- Main class names may use dots or slashes and must omit `.class`.
- Java program arguments are not currently exposed by the launcher.

Run `cargo run -p vm -- --help` for available launcher options.

## Workspace

| Crate              | Responsibility                                                                      |
|--------------------|-------------------------------------------------------------------------------------|
| `lvm-common`       | JVM descriptors, signatures, Java types, and shared byte utilities                  |
| `lvm-class`        | Java 25 class-file model, parser, bytecode decoder, verifier, and optional writer   |
| `jimage`           | Memory-mapped reader for classes stored in the JDK runtime image                    |
| `runtime`          | Bootstrap, class loading and linking, heap, interpreter, natives, threads, and JDWP |
| `vm`               | CLI launcher and VM integration-test target                                         |
| `feature-tracking` | Internal feature-registry validation and report generation                          |

## Documentation

- [Building and running](docs/BUILDING.md)
- [Testing](docs/TESTING.md)
- [Feature tracking](docs/FEATURE_TRACKING.md)
- [Feature support](docs/features/README.md)
- [Integration test coverage](docs/features/TEST_COVERAGE.md)
- [Java Virtual Machine Specification, Java SE 25](https://docs.oracle.com/javase/specs/jvms/se25/html/index.html)
- [Java Language Specification, Java SE 25](https://docs.oracle.com/javase/specs/jls/se25/html/index.html)
