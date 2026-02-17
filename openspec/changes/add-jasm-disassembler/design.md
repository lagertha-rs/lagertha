## Context

The lagertha-vm workspace has a "god crate" problem in `jclass`: it handles parsing, serialization, and javap-style
formatting all in one place, with formatting code scattered across every type via `#[cfg(feature = "javap_print")]`
blocks. The jasm crate needs a disassembler and e2e tests to mature beyond its HelloWorld milestone.

## Goals
- Clean separation between data/parsing and formatting concerns in jclass
- Roundtrip-capable disassembler in jasm (`disassemble(.class) -> .ja -> assemble(.ja) -> .class`)
- Robust e2e test infrastructure for the assembler
- Maintain backward compatibility for all existing consumers (runtime, javap, jasm)

## Non-Goals
- Moving javap formatting out of jclass into a separate crate (too aggressive for now)
- Adding new opcodes (follow-on work, separate changes)
- Changing the javap output format or fixing javap output bugs
- Making the disassembler feature-complete for all class file features

## Decisions

### javap_fmt reorganization: Dedicated `fmt` module vs visitor pattern
- **Decision**: Dedicated `jclass/src/fmt/` module with free functions or extension traits
- **Why**: Visitor pattern adds complexity (trait definitions, double dispatch) for a codebase where the type hierarchy
  is stable. A dedicated module achieves the same separation with less ceremony.
- **Alternatives considered**: Visitor pattern (too much boilerplate), separate crate (too aggressive, breaks feature
  flag approach), keep as-is (the whole point is to fix this)

### Disassembler location: jasm subcommand vs separate crate
- **Decision**: Subcommand of jasm binary (`jasm disassemble <file.class>`)
- **Why**: Assembler and disassembler are natural inverses, share the `.ja` format definition, and belong together.
  Separate crate would duplicate format knowledge.

### E2E test strategy: assemble -> parse back -> assert
- **Decision**: Assemble `.ja` to bytes, parse bytes back with `ClassFile::try_from()`, assert on the `ClassFile`
  structure using insta snapshots
- **Why**: Self-contained (no external tool dependency), validates the full pipeline, doesn't require the disassembler
  to exist first. Roundtrip tests (`disassemble -> assemble -> compare`) come later as a bonus.

### Roundtrip identity
- **Decision**: `disassemble(.class) -> .ja -> assemble(.ja) -> .class` is idempotent after the first disassembly
- **Why**: The `.ja` format is our canonical text representation. The first disassembly normalizes any class file into
  canonical `.ja` form. After that, the roundtrip should be stable. Strict identity with the original `.ja` source is
  not guaranteed (the disassembler may emit explicit defaults like `.super java/lang/Object`).

## Risks / Trade-offs

- **Refactoring javap_fmt is mechanical but large**: ~1000+ lines of formatting code scattered across 15+ files need
  to move. Risk of breaking javap output. Mitigation: run existing javap integration tests after each move.
- **Disassembler format not yet battle-tested**: The `.ja` format may need evolution as more features are added.
  Mitigation: start with the minimal subset matching current assembler capability, evolve incrementally.
- **Feature flag complexity**: The `javap_print` feature flag now gates a module instead of scattered blocks. This is
  simpler but means the `fmt` module is entirely gated, not individual methods. Trade-off is acceptable.

## Open Questions
- Should the `fmt` module use extension traits (e.g., `impl JavapFmt for ConstantEntry`) or free functions that take
  references? Extension traits are more ergonomic but add trait definitions. Free functions are simpler but less Rusty.
  Decision can be made during implementation.
