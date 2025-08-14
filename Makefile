.PHONY: build test clean install help

# Default target
all: build

# Run the compiler with a given input file in the temp directory
# Used during rapid development
run:
	RUST_BACKTRACE=1 cargo run -- --input temp/foo.py 

# Build the compiler in release mode
build:
	@echo "🔨 Building pytc compiler..."
	@cargo build --release

# Run the comprehensive test suite
test: build
	@echo "🧪 Running test suite..."
	@./run_tests.sh

# Run individual test
test-single:
	@if [ -z "$(name)" ]; then \
		echo "Usage: make test-single name=<test_name>"; \
		echo "Available tests:"; \
		ls tests/python_files/test_*.py | xargs -n1 basename | sed 's/\.py$$//' | sed 's/^/  - /'; \
	else \
		echo "🧪 Running single test: $(name)"; \
		mkdir -p tests/artifacts; \
		./target/release/pytc --input tests/python_files/$(name).py; \
		mv tests/python_files/$(name).ll tests/artifacts/$(name).ll 2>/dev/null || true; \
		llc tests/artifacts/$(name).ll -o tests/artifacts/$(name).s; \
		gcc tests/artifacts/$(name).s -o tests/artifacts/$(name); \
		echo "Output: $$(./tests/artifacts/$(name))"; \
		rm -f tests/python_files/$(name).instructions tests/python_files/$(name).pyc tests/python_files/$(name).s tests/python_files/$(name) 2>/dev/null || true; \
	fi

# Clean build artifacts and test outputs
clean:
	@echo "🧹 Cleaning..."
	@cargo clean
	@rm -rf tests/artifacts/*
	@rm -f tests/python_files/*.ll
	@rm -f tests/python_files/*.s
	@rm -f tests/python_files/*.instructions
	@rm -f tests/python_files/*.pyc
	@rm -f tests/python_files/test_addition
	@rm -f tests/python_files/test_basic_arithmetic
	@rm -f tests/python_files/test_multiple_operations
	@rm -f tests/python_files/test_simple_assignment
	@rm -f tests/python_files/test_subtraction
	@rm -f tests/python_files/test_variable_reuse
	@rm -f tests/expected_outputs/*.actual
	@rm -f *.ll *.s *.o *.instructions *.pyc

# Install dependencies (requires LLVM and GCC)
install-deps:
	@echo "📦 Installing dependencies..."
	@echo "Please ensure you have the following installed:"
	@echo "  - Rust toolchain (rustc, cargo)"
	@echo "  - LLVM (llc command)"
	@echo "  - GCC compiler"
	@echo ""
	@echo "On macOS: brew install llvm gcc"
	@echo "On Ubuntu: apt install llvm gcc"

# Development build (with debug info)
dev:
	@echo "🔨 Building pytc compiler (debug)..."
	@cargo build

# Check code without building
check:
	@echo "🔍 Checking code..."
	@cargo check

# Run linter
lint:
	@echo "🔧 Running linter..."
	@cargo clippy

# Format code
fmt:
	@echo "💄 Formatting code..."
	@cargo fmt

# Show help
help:
	@echo "pytc - Python to LLVM Compiler"
	@echo ""
	@echo "Available targets:"
	@echo "  build          Build the compiler in release mode"
	@echo "  test           Run the full test suite"
	@echo "  test-single    Run a single test (use: make test-single name=test_addition)"
	@echo "  clean          Clean build and test artifacts"
	@echo "  dev            Build in debug mode"
	@echo "  check          Check code without building"
	@echo "  lint           Run clippy linter"
	@echo "  fmt            Format code with rustfmt"
	@echo "  install-deps   Show dependency installation instructions"
	@echo "  help           Show this help message"
	@echo ""
	@echo "Examples:"
	@echo "  make build"
	@echo "  make test"
	@echo "  make test-single name=test_addition"
	@echo "  make clean"
