# Lagertha

Lagertha is an educational Java Virtual Machine implementation written in Rust.
The long-term target is a functional, broadly compatible JVM for Java 25, not a
generic bytecode interpreter or a JVM for an unspecified Java version. The
project is early-stage and implements that target incrementally.

The Java SE 25 specifications are Lagertha's source of truth:

- Java Virtual Machine Specification:
  https://docs.oracle.com/javase/specs/jvms/se25/html/index.html
- Java Language Specification:
  https://docs.oracle.com/javase/specs/jls/se25/html/index.html

Use the JVMS as the primary behavioral and class-file reference. Use the JLS
when source-language behavior helps define an integration test. Do not describe
a feature as supported only because a parser enum, interpreter arm, native
registration, or stub exists; support requires working behavior and appropriate
evidence.

Feature specification references must be direct links into these SE 25 sources,
not plain article labels. Before adding or changing any specification link, use
`curl --fail --silent --show-error --location` to verify the page exists. For a
fragment link, also inspect the fetched HTML and verify its fragment ID exists;
curl does not send URL fragments to the server. Never add an unverified link.

## Organization And Local Workspace

Organization repositories are cloned as siblings under one workspace root:

```text
lagertha-org-workspace/
├── .github/
├── lagertha/   # this repository
└── runestaff/
```

When it is needed, you can inspect sibling repositories locally when work crosses repository boundaries. 
Read their `AGENTS.md` and relevant documentation before proposing changes. Use
`gh` for remote Issues, Projects, pull requests, and repository metadata; prefer
the local workspace for source inspection.

## Workspace Architecture

The Cargo workspace uses Rust edition 2024 and contains:

| Crate | Responsibility |
|---|---|
| `lvm-common` | JVM descriptors, signatures, Java types, and shared byte utilities |
| `lvm-class` | Java 25 class-file model, parser, bytecode decoder, verifier, and optional writer |
| `jimage` | Memory-mapped reader for JDK runtime-image classes |
| `runtime` | Bootstrap, class loading and linking, heap, interpreter, natives, threads, and JDWP |
| `vm` | CLI launcher and VM integration-test target |

Dependency direction is broadly:

```text
vm -> runtime -> lvm-class -> lvm-common
               -> jimage
runestaff/rns-lang -> lvm-class
```

`lvm-class` is also a published dependency of Runestaff. Public model, parser,
verification, and writer changes may require sibling-repository inspection.
Runestaff provides `rnsc`, which generates exact or intentionally invalid class
files for Lagertha integration tests.

## Development Documentation

Read the relevant guide before working in that area:

- [`docs/BUILDING.md`](docs/BUILDING.md): required toolchain, building, CI-equivalent checks, and running the VM.
- [`docs/TESTING.md`](docs/TESTING.md): focused tests, fixture compilation, integration-test discovery, and snapshot review.
- [`docs/FEATURE_TRACKING.md`](docs/FEATURE_TRACKING.md): feature state and integration-coverage tracking.

Do not duplicate commands or test mechanics in this file. Update the owning
guide when those workflows change.

## Engineering Rules

- Prefer JVMS-defined behavior over assumptions based on Rust, `javac`, or one
  JDK implementation.
- Preserve the distinction between VM failures and Java exceptions. Supported
  bytecode errors should become the specified Java exception where required.
- Treat malformed class files and unusual constant-pool entries as intentional
  test inputs. Do not normalize them merely because `javac` cannot emit them.
- Keep changes within the active Issue's scope. Record discovered work as a
  follow-up instead of silently expanding the change.
- TODO comments are neither requirements nor roadmap commitments. Do not derive
  task scope or create Issues from them alone.
- Avoid manual feature counts and implementation claims. Follow the feature
  tracking guide for capability and integration-test state.
- Do not update generated artifacts by hand.
