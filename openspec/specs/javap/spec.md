# javap - Java Class File Disassembler

## Purpose

The `javap` crate provides a Java class file disassembler tool that displays the structure of `.class` files. It targets
compatibility with Oracle's `javap -v -p` command output for Java 25 class files, with implicit `-v` (verbose) and
`-p` (private) flags always enabled.

## Requirements

### Requirement: Class File Disassembly

The system SHALL parse and display Java `.class` files in a format compatible with Oracle's `javap -v -p` output,
targeting Java 25 class file format.

#### Scenario: Successful disassembly

- **WHEN** a valid Java 25 `.class` file is provided as input
- **THEN** the tool outputs structured disassembly including class metadata, constant pool, fields, methods, and
  attributes

#### Scenario: Invalid file handling

- **WHEN** an invalid or corrupted `.class` file is provided
- **THEN** the tool exits with a non-zero exit code and displays an appropriate error message

#### Scenario: File not found

- **WHEN** a non-existent `.class` file path is provided
- **THEN** the tool exits with exit code 2 and displays "Error: class not found: {path}"

### Requirement: Output Format Compatibility

The system SHALL produce output that matches Oracle's `javap -v -p` format when compared with whitespace normalization.

#### Scenario: Whitespace-normalized comparison

- **WHEN** the tool's output is compared with Oracle's `javap -v -p` output
- **THEN** the outputs match after removing all whitespace characters

### Requirement: Class File Path Resolution

The system SHALL support both dotted and slash-separated class name formats for input paths.

#### Scenario: Dotted class name

- **WHEN** input is `java.lang.String.class`
- **THEN** the tool resolves to `java/lang/String.class`

#### Scenario: Slash-separated path

- **WHEN** input is `java/lang/String.class`
- **THEN** the tool uses the path directly

#### Scenario: Path with .class extension

- **WHEN** input includes or omits the `.class` extension
- **THEN** the tool normalizes to include the `.class` extension

### Requirement: Class Metadata Display

The system SHALL display class file metadata at the beginning of the output.

#### Scenario: Metadata fields displayed

- **WHEN** a class file is disassembled
- **THEN** the output includes:
    - Absolute file path (line: `Classfile {path}`)
    - Last modification timestamp (format: `Last modified {Mon DD, YYYY}; size {N} bytes`)
    - SHA-256 checksum (line: `SHA-256 checksum {hex}`)
    - Source file attribute if present (line: `Compiled from "{filename}"`)

### Requirement: Class Header Display

The system SHALL display class declaration information after metadata.

#### Scenario: Class header information

- **WHEN** a class file is disassembled
- **THEN** the output includes:
    - Access modifiers in Java-like syntax (public, abstract, final, interface, etc.)
    - Class name with generic type parameters if present
    - Superclass (extends clause)
    - Implemented interfaces (implements clause)
    - Class version information (minor.major version)
    - Access flags with `ACC_*` symbolic names

### Requirement: Constant Pool Display

The system SHALL display all constant pool entries with their indices, types, and resolved values.

#### Scenario: Constant pool entry formatting

- **WHEN** the constant pool is displayed
- **THEN** each entry shows:
    - Index prefixed with `#`
    - Equals sign and type name
    - Value with resolved references as comments
    - Double-width entries (Long/Double) occupy two slots

### Requirement: Field Display

The system SHALL display all fields including private fields with their descriptors, flags, and attributes.

#### Scenario: Field information displayed

- **WHEN** fields are displayed
- **THEN** each field shows:
    - Access modifiers (public, private, protected, static, final, volatile, transient)
    - Type and name in Java-like syntax
    - Raw descriptor
    - Access flags with `ACC_*` symbolic names
    - Field attributes (ConstantValue, Signature, RuntimeVisibleAnnotations, etc.)

### Requirement: Method Display

The system SHALL display all methods including private methods with their descriptors, flags, bytecode, and attributes.

#### Scenario: Method information displayed

- **WHEN** methods are displayed
- **THEN** each method shows:
    - Access modifiers in Java-like syntax
    - Return type and method name (with `<init>` for constructors, `<clinit>` for static initializers)
    - Parameter types (with varargs as `...` when applicable)
    - Throws clause (from Signature attribute or Exceptions attribute)
    - Raw descriptor
    - Access flags with `ACC_*` symbolic names
    - Code attribute with bytecode instructions (if not abstract/native)
    - Method attributes (LineNumberTable, LocalVariableTable, StackMapTable, Exceptions, etc.)

### Requirement: Generic Signature Display

The system SHALL display generic type signatures when the Signature attribute is present.

#### Scenario: Generic method signature

- **WHEN** a method has a Signature attribute with generic type parameters
- **THEN** the method header displays type parameters (e.g., `<T extends Comparable>`) and generic types in parameters
  and return type

#### Scenario: Generic class signature

- **WHEN** a class has a Signature attribute
- **THEN** the class header displays type parameters and generic superclass/interfaces

#### Scenario: Generic field signature

- **WHEN** a field has a Signature attribute
- **THEN** the field type displays the generic type instead of the erased type

### Requirement: Bytecode Instruction Display

The system SHALL display bytecode instructions with their offsets, mnemonics, operands, and constant pool references.

#### Scenario: Instruction formatting

- **WHEN** bytecode instructions are displayed
- **THEN** each instruction shows:
    - Byte offset from method start (right-aligned)
    - Instruction mnemonic
    - Operands (constant pool indices prefixed with `#`, branch targets as absolute offsets)
    - Comment with resolved constant pool values where applicable

#### Scenario: Wide instruction handling

- **WHEN** a `wide` instruction is encountered
- **THEN** the instruction and its widened operand are displayed correctly

#### Scenario: Tableswitch and lookupswitch

- **WHEN** switch instructions are encountered
- **THEN** the instruction displays with properly formatted case tables

### Requirement: Stack Map Table Display

The system SHALL display StackMapTable entries with correct frame types according to JVM specification.

#### Scenario: Frame type values

- **WHEN** StackMapTable frames are displayed
- **THEN** frame_type values match JVM specification:
    - `same_frame`: 0-63
    - `same_locals_1_stack_item_frame`: 64-127
    - `same_locals_1_stack_item_frame_extended`: 247
    - `chop_frame`: 248-250
    - `same_frame_extended`: 251
    - `append_frame`: 252-254
    - `full_frame`: 255

#### Scenario: Verification type display

- **WHEN** verification types are displayed in StackMapTable
- **THEN** types show as Top, Integer, Float, Long, Double, Null, UninitializedThis, Object (with class reference), or
  Uninitialized (with offset)

### Requirement: Annotation Display

The system SHALL display runtime annotations including class, field, method, and parameter annotations.

#### Scenario: Class and member annotations

- **WHEN** a class, field, or method has RuntimeVisibleAnnotations or RuntimeInvisibleAnnotations
- **THEN** annotations are displayed with their type and element-value pairs

#### Scenario: Parameter annotations

- **WHEN** a method has RuntimeVisibleParameterAnnotations or RuntimeInvisibleParameterAnnotations
- **THEN** annotations are displayed with parameter indices (0-based) and annotation values

#### Scenario: Type annotations

- **WHEN** a class, field, or method has RuntimeVisibleTypeAnnotations or RuntimeInvisibleTypeAnnotations
- **THEN** type annotations are displayed with their target info and type path

### Requirement: Inner Classes Display

The system SHALL display InnerClasses attribute with proper access modifiers.

#### Scenario: Named inner class

- **WHEN** an InnerClasses entry has both outer_class and inner_name
- **THEN** the entry displays as
  `{flags} #{inner_name_idx}= #{inner_class_idx} of #{outer_class_idx}; // {name}={inner_class} of {outer_class}`

#### Scenario: Anonymous inner class

- **WHEN** an InnerClasses entry has outer_class_info_index=0 and inner_name_index=0
- **THEN** the entry displays as `{flags} #{inner_class_idx}; // {inner_class}`
- **AND** access flags like `static` are included when present

#### Scenario: Local or member class without outer

- **WHEN** an InnerClasses entry has outer_class_info_index=0 but has inner_name
- **THEN** the entry displays as `#{inner_name_idx}= #{inner_class_idx}; // {name}={inner_class}`

### Requirement: Bootstrap Methods Display

The system SHALL display BootstrapMethods attribute for invokedynamic support.

#### Scenario: Bootstrap method entry

- **WHEN** a class has BootstrapMethods attribute
- **THEN** each bootstrap method displays:
    - Bootstrap method index
    - Method handle reference with resolved value
    - Method arguments with their types and values

#### Scenario: Bootstrap method argument types

- **WHEN** bootstrap method arguments are displayed
- **THEN** Class constants display without "class " prefix (just the class name)
- **AND** other constant types display with appropriate type prefixes

### Requirement: Exception Table Display

The system SHALL display exception handler tables in the Code attribute.

#### Scenario: Exception handler formatting

- **WHEN** a method has exception handlers
- **THEN** each handler displays:
    - Start PC (from)
    - End PC (to)
    - Handler PC (target)
    - Catch type (class name or "any" for finally blocks)

### Requirement: Line Number Table Display

The system SHALL display LineNumberTable attribute mapping bytecode offsets to source lines.

#### Scenario: Line number entries

- **WHEN** a method has LineNumberTable attribute
- **THEN** each entry displays: `line {source_line}: {bytecode_offset}`

### Requirement: Local Variable Table Display

The system SHALL display LocalVariableTable and LocalVariableTypeTable attributes.

#### Scenario: Local variable entries

- **WHEN** a method has LocalVariableTable attribute
- **THEN** each entry displays: `Start  Length  Slot  Name   Signature`

#### Scenario: Local variable type entries

- **WHEN** a method has LocalVariableTypeTable attribute (for generics)
- **THEN** generic type signatures are displayed for local variables

## Known Limitations

The following features are not yet implemented and will cause runtime panics if encountered:

- Module attribute (module-info.class files)
- ModulePackages attribute
- ModuleMainClass attribute
- Record attribute (Java records)
- SourceDebugExtension attribute
- Synthetic attribute display
- AnnotationDefault attribute for annotation types
- RuntimeVisibleTypeAnnotations/RuntimeInvisibleTypeAnnotations in Code attribute

## Testing Approach

The `javap` crate uses integration testing that compares output line-by-line with Oracle's `javap -v -p` on 350 JDK
class files extracted from `.jmod` archives. Whitespace is normalized (removed) before comparison to allow minor
formatting differences.

Test fixtures are defined in `tests/testdata/fixtures.toml` and extracted during build from the JDK installation
specified by `JAVA_HOME`.

## Dependencies

- `jclass` crate with `javap_print` feature for class file parsing and formatting
- `sha2` for SHA-256 checksum computation
- `chrono` for timestamp formatting
- JDK 25 installation for test fixture extraction and Oracle javap comparison
