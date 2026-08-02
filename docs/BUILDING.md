# Building And Running Lagertha

This guide owns Lagertha's toolchain, build, verification, and VM execution
instructions.

## Required Toolchain

Lagertha targets Java 25. Current builds and runtime validation require JDK
`25.0.1` exactly.

Set `JAVA_HOME` to that JDK. The VM reads `${JAVA_HOME}/release` and
`${JAVA_HOME}/lib/modules`; startup fails without them.

Building the `vm` crate runs `vm/build.rs`, which requires:

- `javac 25.0.1`
- `rnsc 0.2.1`

Install the required fixture assembler:

```bash
cargo install rnsc --version 0.2.1 --locked
```

Do not downgrade class files, fixtures, or behavior to another Java version
unless an Issue explicitly changes the supported-version policy.

## Build

Build the complete workspace:

```bash
cargo build --workspace --locked
```

Build one package:

```bash
cargo build -p <package> --locked
```

Any build containing `vm` also compiles its Java and RNS test fixtures. See
[`TESTING.md`](TESTING.md) for that pipeline.

## CI-Equivalent Checks

CI runs:

```bash
cargo fmt --all -- --check
cargo build --workspace --all-features --verbose --locked
cargo test --workspace --verbose --locked
```

Clippy is currently disabled in CI because existing warnings remain. A Clippy
run does not replace CI-equivalent verification.

Run focused checks while iterating, then run the workspace checks required by
the change. Report commands actually run and any checks not run.

## Running The VM

```bash
cargo run -p vm -- -c <class-directory> <package.Main>
```

Current CLI behavior:

- Classpath entries support directories, not JAR files.
- Multiple classpath entries use `;` as separator, including on Unix.
- Main class names may use dots or slashes and must omit `.class`.
- If no classpath is provided, the current directory is used.

Enable runtime tracing with the `log-runtime-traces` feature:

```bash
cargo run -p vm --features log-runtime-traces -- \
  -c vm/tests/testdata/compiled bootstrap.main.HelloWorldTest
```

Enable a JDWP listener with `--jdwp-port <port>`.

### Java Source Launcher

Use the development launcher to find a Java or RNS source, compile it, and run
its main class on Lagertha:

```bash
./tools/run-java.py HelloWorldTest
./tools/run-java.py path/to/Main.java
./tools/run-java.py path/to/Main.rns
```

The source argument may be a path, filename, or filename without its extension.
Java sources compile with sibling `.java` files; RNS sources compile with
`rnsc`. Duplicate filenames produce a numbered path chooser. Omitting the
argument lists all `.java` and `.rns` sources outside `.cache`, `.git`,
`.github`, `docs`, `features`, and `target`.

Direct runs compile into ignored `.cache/java-run` and print the compiler and
Lagertha commands before execution. Sources under `vm/tests/testdata`
also offer the integration harness, which runs Lagertha and the reference JDK.
Use `--mode vm` or `--mode integration` to skip that prompt.
