# Test Files Reorganization Summary

## Overview
This document describes the reorganization of integration test files in the `vm/tests/testdata/java` directory. The reorganization improves code organization, naming consistency, and makes tests easier to find and understand.

## Key Changes

### 1. **Improved Directory Structure**
Tests are now organized by functionality with clear categorization:

```
vm/tests/testdata/java/
├── arrays/
│   ├── objects/
│   │   ├── basic/          # Functional tests for object arrays
│   │   └── bounds/         # Boundary violation tests
│   └── primitives/
│       ├── basic/          # Functional tests for primitive arrays
│       └── bounds/         # Boundary violation tests
├── exceptions/
│   ├── assert/             # Java assert mechanism tests
│   ├── handling/           # Exception handling tests (try-catch-finally)
│   └── propagation/        # Exception propagation and stack trace tests
├── hello_world/            # Basic "Hello World" test
├── natives/
│   └── system/
│       └── arraycopy/      # System.arraycopy native method tests
└── primitives/
    └── int/
        ├── arithmetic/     # Integer arithmetic operations
        └── errors/         # Error cases (division by zero, etc.)
```

### 2. **Simplified File Names**
File names have been shortened while maintaining clarity:

**Before:** `CreateArrayWithNegativeSizeShouldThrowErrMain.java`  
**After:** `NegativeSizeErrMain.java`

**Before:** `SimplestCatchShouldWorkOkMain.java`  
**After:** `SimpleCatchOkMain.java`

**Before:** `SystemArrayCopyShouldThrowWithNullSrcErrMain.java`  
**After:** `NullSrcErrMain.java`

### 3. **Consistent Package Naming**
Package names follow the pattern: `<category>.<subcategory>.<descriptive_name>`

Examples:
- `arrays.primitives.basic.byte_array`
- `exceptions.handling.simple_catch`
- `primitives.int.errors.division_by_zero`
- `natives.system.arraycopy.basic`

### 4. **Naming Convention Maintained**
- **`OkMain` suffix:** Tests that expect exit code 0 (successful execution)
- **`ErrMain` suffix:** Tests that expect non-zero exit code (error/exception)
- **Class name = File name:** Every class name matches its filename

## Detailed Mapping

### Arrays Tests

#### Primitives - Basic (Functional Tests)
| Old Name | New Name |
|----------|----------|
| `ByteArrayTestOkMain.java` | `basic/ByteArrayOkMain.java` |
| `CharArrayTestOkMain.java` | `basic/CharArrayOkMain.java` |

#### Primitives - Bounds (Error Cases)
| Old Name | New Name |
|----------|----------|
| `CreateArrayWithNegativeSizeShouldThrowErrMain.java` | `bounds/NegativeSizeErrMain.java` |
| `GetNegativeArrayIndexShouldThrowErrMain.java` | `bounds/GetNegativeIndexErrMain.java` |
| `GetOutOfBoundArrayIndexShouldThrowErrMain.java` | `bounds/GetOutOfBoundsErrMain.java` |
| `SetNegativeArrayIndexShouldThrowErrMain.java` | `bounds/SetNegativeIndexErrMain.java` |
| `SetOutOfBoundArrayIndexShouldThrowErrMain.java` | `bounds/SetOutOfBoundsErrMain.java` |

#### Objects - Basic & Bounds
Similar pattern as primitives, with tests organized into `basic/` and `bounds/` subdirectories.

### Exception Tests

#### Handling (Try-Catch-Finally)
| Old Name | New Name |
|----------|----------|
| `SimplestCatchShouldWorkOkMain.java` | `handling/SimpleCatchOkMain.java` |
| `CatchSpecificExceptionShouldWorkOkMain.java` | `handling/CatchSpecificOkMain.java` |
| `CatchSubclassExceptionShouldWorkOkMain.java` | `handling/CatchSubclassOkMain.java` |
| `CatchExceptionAccessMessageShouldWorkOkMain.java` | `handling/AccessMessageOkMain.java` |
| `MultipleCatchBlocksShouldWorkOkMain.java` | `handling/MultipleCatchOkMain.java` |
| `CatchWithoutThrowShouldWorkOkMain.java` | `handling/CatchWithoutThrowOkMain.java` |
| `FinallyAfterCatchShouldWorkOkMain.java` | `handling/FinallyOkMain.java` |
| `ExceptionInCatchBlockShouldWorkOkMain.java` | `handling/NestedExceptionOkMain.java` |
| `NestedMethodCallsCatchShouldWorkOkMain.java` | `handling/NestedCallsOkMain.java` |
| `WrongExceptionTypeShouldFailErrMain.java` | `handling/WrongTypeErrMain.java` |
| `NoMatchingHandlerInMultipleCatchesErrMain.java` | `handling/NoMatchingHandlerErrMain.java` |

#### Propagation (Uncaught Exceptions)
| Old Name | New Name |
|----------|----------|
| `ExceptionOutsideTryCatchErrMain.java` | `propagation/UncaughtErrMain.java` |
| `ExceptionInCatchBlockPropagatesErrMain.java` | `propagation/FromCatchBlockErrMain.java` |
| `ExceptionShouldPrintStackTraceWithMultipleElementsErrMain.java` | `propagation/StackTraceErrMain.java` |

#### Assert (Java Assertions)
| Old Name | New Name |
|----------|----------|
| `JavaAssertShouldThrowErrMain.java` | `assert/FailedAssertErrMain.java` |
| `JavaAssertNotMatchedOkMain.java` | `assert/PassedAssertOkMain.java` |

### Primitive Tests

#### Integer Arithmetic
| Old Name | New Name |
|----------|----------|
| `IntArithmeticOkMain.java` | `arithmetic/ArithmeticOkMain.java` |

#### Integer Errors
| Old Name | New Name |
|----------|----------|
| `IntZeroDivisionShouldThrowErrMain.java` | `errors/DivisionByZeroErrMain.java` |
| `IntZeroModuloShouldThrowErrMain.java` | `errors/ModuloByZeroErrMain.java` |

### Native Method Tests

#### System.arraycopy
| Old Name | New Name |
|----------|----------|
| `SystemArrayCopyOkMain.java` | `arraycopy/ArrayCopyOkMain.java` |
| `SystemArrayCopyShouldThrowWithNullSrcErrMain.java` | `arraycopy/NullSrcErrMain.java` |
| `SystemArrayCopyShouldThrowWithNullDestErrMain.java` | `arraycopy/NullDestErrMain.java` |
| `SystemArrayCopyShouldThrowWithNonArrSrcErrMain.java` | `arraycopy/NonArraySrcErrMain.java` |
| `SystemArrayCopyShouldThrowWithNonArrDestErrMain.java` | `arraycopy/NonArrayDestErrMain.java` |

### Hello World
| Old Name | New Name |
|----------|----------|
| `PrintHelloWorldOkMain.java` | `HelloWorldOkMain.java` |

## Benefits

1. **Better Organization:** Tests are grouped by functionality and purpose
2. **Easier Navigation:** Clear folder structure makes finding tests intuitive
3. **Reduced Redundancy:** Shorter file names without losing clarity
4. **Consistent Patterns:** All tests follow the same organizational principles
5. **Scalability:** Easy to add new tests in the appropriate category
6. **Maintainability:** Clear separation of concerns makes updates easier

## Technical Notes

- All package declarations have been updated to match the new folder structure
- Class names have been updated to match new file names
- The `OkMain`/`ErrMain` suffix convention is preserved
- Build system (`build.rs`) automatically compiles all Java files recursively
- Snapshot files will need to be regenerated to reflect new package paths
