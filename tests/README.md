# pytc Test Suite

This directory contains the test suite for the pytc Python-to-LLVM compiler.

## Directory Structure

```
tests/
├── python_files/          # Python source files for testing
├── expected_outputs/      # Expected output for each test
├── artifacts/             # Generated build artifacts (gitignored)
└── README.md              # This file
```

## Test Cases

### Basic Functionality Tests

1. **test_simple_assignment.py** - Tests basic variable assignment and printing
2. **test_addition.py** - Tests addition operation
3. **test_subtraction.py** - Tests subtraction operation
4. **test_multiple_operations.py** - Tests chained arithmetic operations
5. **test_basic_arithmetic.py** - Complex arithmetic with multiple variables
6. **test_variable_reuse.py** - Tests variable reassignment

## Running Tests

### Option 1: Makefile (Recommended)

```bash
# Run all tests
make test

# Run a single test
make test-single name=test_addition

# Build compiler
make build

# Clean artifacts
make clean
```

### Option 2: Shell Script (Direct)

```bash
./run_tests.sh
```

This script:

- Builds the compiler with `cargo build --release`
- Compiles each Python test file to LLVM IR
- Compiles LLVM IR to assembly with `llc`
- Links assembly to executable with `gcc`
- Runs each executable and compares output
- Provides detailed colored output and summary

### Option 3: Manual test execution

```bash
# Build compiler first
cargo build --release

# Run specific test manually
./target/release/pytc --input tests/python_files/test_addition.py
llc tests/python_files/test_addition.ll -o tests/artifacts/test_addition.s
gcc tests/artifacts/test_addition.s -o tests/artifacts/test_addition
./tests/artifacts/test_addition
```

## Adding New Tests

To add a new test case:

1. Create a Python file in `tests/python_files/` with a descriptive name (e.g., `test_new_feature.py`)
2. Create the expected output file in `tests/expected_outputs/` (e.g., `test_new_feature.expected`)
3. The test runner will automatically pick up new tests - no code changes needed!

## Test Requirements

- **LLVM tools**: `llc` must be available in PATH
- **C compiler**: `gcc` must be available in PATH
- **Rust toolchain**: For building the compiler
- **Release build**: Tests use the release build of the compiler for performance

## Artifacts

Build artifacts are automatically managed and include:

**In `tests/artifacts/` (organized, gitignored):**

- `.ll` files (LLVM IR)
- `.s` files (Assembly)
- Executable files
- `.actual` files (actual output for failed tests)

**In `tests/python_files/` (generated, gitignored):**

- `.ll` files (LLVM IR from compiler)
- `.s` files (Assembly files)
- `.instructions` files (Python bytecode analysis)
- `.pyc` files (Python compiled bytecode)
- Binary executables (test executables)

All artifacts are automatically gitignored and cleaned with `make clean`.
