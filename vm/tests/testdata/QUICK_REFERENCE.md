# Test Reorganization: Quick Reference Guide

## Overview
This guide provides a quick reference for working with the reorganized test structure in `vm/tests/testdata/java/`.

## Naming Conventions

### File Names
- **OkMain suffix**: Tests expecting exit code 0 (successful execution)
- **ErrMain suffix**: Tests expecting non-zero exit code (error/exception)
- **Class name = File name**: Always matches exactly

### Package Names
Pattern: `category.subcategory.descriptive_name`

Examples:
```java
package arrays.primitives.basic.byte_array;
package exceptions.handling.simple_catch;
package primitives.int.errors.division_by_zero;
```

## Directory Organization

### Arrays Tests (`arrays/`)
```
arrays/
├── primitives/
│   ├── basic/      # Functional tests (ByteArrayOkMain, CharArrayOkMain)
│   └── bounds/     # Boundary violations (NegativeSizeErrMain, GetOutOfBoundsErrMain, etc.)
└── objects/
    ├── basic/      # Functional tests (StringEncodingOkMain)
    └── bounds/     # Boundary violations (NegativeSizeErrMain, GetOutOfBoundsErrMain, etc.)
```

### Exception Tests (`exceptions/`)
```
exceptions/
├── assert/         # Java assert mechanism (FailedAssertErrMain, PassedAssertOkMain)
├── handling/       # Try-catch-finally (SimpleCatchOkMain, FinallyOkMain, etc.)
└── propagation/    # Uncaught exceptions (UncaughtErrMain, StackTraceErrMain, etc.)
```

### Primitive Tests (`primitives/`)
```
primitives/
└── int/
    ├── arithmetic/ # Arithmetic operations (ArithmeticOkMain)
    └── errors/     # Error cases (DivisionByZeroErrMain, ModuloByZeroErrMain)
```

### Native Method Tests (`natives/`)
```
natives/
└── system/
    └── arraycopy/  # System.arraycopy tests (ArrayCopyOkMain, NullSrcErrMain, etc.)
```

### Hello World (`hello_world/`)
```
hello_world/
└── HelloWorldOkMain.java  # Basic "Hello World" test
```

## Adding New Tests

### Step 1: Choose the Right Location

**Functional Test?** → Use `basic/` subdirectory  
**Error/Boundary Test?** → Use `bounds/` or `errors/` subdirectory  
**New Category?** → Create appropriate subdirectories following the pattern

### Step 2: Name Your Test

1. Choose a descriptive base name (e.g., `MultiDimensionalArray`)
2. Add `OkMain` for success cases or `ErrMain` for error cases
3. Example: `MultiDimensionalArrayOkMain.java`

### Step 3: Create the File

```java
package <category>.<subcategory>.<descriptive_name>;

public class YourTestNameOkMain {
    public static void main(String[] args) {
        // Your test code here
        // Use assert statements for validation
        System.out.println("Test passed!");
    }
}
```

### Step 4: Build and Test

```bash
# The build system automatically discovers and compiles new tests
cd vm
cargo build

# Run tests
cargo test
```

## Common Patterns

### Test with Multiple Helper Methods
```java
package arrays.primitives.basic.byte_array;

public class ByteArrayOkMain {
    public static void main(String[] args) {
        test_basic_operations();
        test_edge_cases();
        System.out.println("All tests passed");
    }
    
    static void test_basic_operations() {
        // Test code
        assert condition : "Error message";
    }
    
    static void test_edge_cases() {
        // Test code
    }
}
```

### Error Test (Expected to Fail)
```java
package primitives.int.errors.division_by_zero;

public class DivisionByZeroErrMain {
    public static void main(String[] args) {
        int result = 1 / 0;  // Should throw ArithmeticException
    }
}
```

### Exception Handling Test
```java
package exceptions.handling.simple_catch;

public class SimpleCatchOkMain {
    public static void main(String[] args) {
        try {
            throw new IllegalArgumentException("Test");
        } catch (Throwable e) {
            System.out.println("Caught exception");
        }
    }
}
```

## File Name Length Guidelines

**Target:** 20-30 characters for file names  
**Max:** Try to keep under 35 characters

**Good Examples:**
- `ByteArrayOkMain.java` (19 chars)
- `NegativeSizeErrMain.java` (24 chars)
- `StackTraceErrMain.java` (22 chars)

**Avoid:**
- `CreateArrayWithNegativeSizeShouldThrowErrMain.java` (51 chars) ❌
- Use: `NegativeSizeErrMain.java` (24 chars) ✅

## Package Name Guidelines

**Pattern:** `category.subcategory.descriptive_name`

**Categories:**
- `arrays` - Array-related tests
- `exceptions` - Exception-related tests
- `primitives` - Primitive type tests
- `natives` - Native method tests
- `hello_world` - Basic tests

**Subcategories:**
- `basic` - Functional tests
- `bounds` - Boundary violation tests
- `errors` - Error condition tests
- `handling` - Exception handling mechanisms
- `propagation` - Exception propagation
- `assert` - Assertion tests

**Descriptive Names:**
- Keep concise (2-3 words max)
- Use underscores to separate words
- Describe what the test does

**Examples:**
✅ `arrays.primitives.basic.byte_array`
✅ `exceptions.handling.simple_catch`
✅ `primitives.int.errors.division_by_zero`
❌ `arrays.primitives.create_array_with_negative_size_should_throw_err`

## Test Discovery

Tests are discovered by the build system based on:
1. Location: `vm/tests/testdata/java/**/*.java`
2. Pattern: `**/*OkMain.class` or `**/*ErrMain.class` in compiled directory

The integration test runner in `vm/tests/integration_test.rs` uses these patterns to:
- Find all `*OkMain.class` files and expect exit code 0
- Find all `*ErrMain.class` files and expect non-zero exit code

## Limitations

- **No invokedynamic**: String concatenation is limited, use static strings or StringBuilder
- **Java asserts work**: Tests extensively use `assert` statements
- **Java version**: Currently requires Java 24 for full compatibility

## Documentation

For more details, see:
- `REORGANIZATION_SUMMARY.md` - Complete mapping of all changes
- `BEFORE_AFTER_COMPARISON.md` - Visual comparison and statistics
