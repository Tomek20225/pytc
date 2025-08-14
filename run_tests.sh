#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Directories
TEST_DIR="tests"
PYTHON_FILES_DIR="$TEST_DIR/python_files"
EXPECTED_DIR="$TEST_DIR/expected_outputs"
ARTIFACTS_DIR="$TEST_DIR/artifacts"

# Counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

echo -e "${BLUE}🚀 Running pytc compiler tests...${NC}\n"

# Build the compiler first
echo -e "${YELLOW}Building compiler...${NC}"
if ! cargo build --release; then
    echo -e "${RED}❌ Failed to build compiler${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Compiler built successfully${NC}\n"

# Function to run a single test
run_test() {
    local test_file="$1"
    local test_name=$(basename "$test_file" .py)
    local expected_file="$EXPECTED_DIR/${test_name}.expected"
    local ll_file="$ARTIFACTS_DIR/${test_name}.ll"
    local asm_file="$ARTIFACTS_DIR/${test_name}.s"
    local exe_file="$ARTIFACTS_DIR/${test_name}"
    local actual_file="$EXPECTED_DIR/${test_name}.actual"
    
    echo -e "${BLUE}Testing: ${test_name}${NC}"
    
    # Check if expected output file exists
    if [[ ! -f "$expected_file" ]]; then
        echo -e "${RED}  ❌ Missing expected output file: $expected_file${NC}"
        ((FAILED_TESTS++))
        return 1
    fi
    
    # Step 1: Compile Python to LLVM IR
    echo -e "  📝 Compiling to LLVM IR..."
    if ! ./target/release/pytc --input "$test_file" > /dev/null 2>&1; then
        echo -e "${RED}  ❌ Failed to compile to LLVM IR${NC}"
        ((FAILED_TESTS++))
        return 1
    fi
    
    # Move generated files to artifacts directory and clean up source directory
    local test_base="$(basename "$test_file" .py)"
    local test_dir="$(dirname "$test_file")"
    local generated_ll="$test_dir/${test_base}.ll"
    
    if [[ -f "$generated_ll" ]]; then
        mv "$generated_ll" "$ll_file"
    else
        # Try alternative location in current directory
        generated_ll="${test_base}.ll"
        if [[ -f "$generated_ll" ]]; then
            mv "$generated_ll" "$ll_file"
        else
            echo -e "${RED}  ❌ LLVM IR file not generated${NC}"
            echo -e "    Expected: $generated_ll"
            echo -e "    Current dir: $(pwd)"
            echo -e "    Files in test dir: $(ls -la $(dirname "$test_file"))"
            ((FAILED_TESTS++))
            return 1
        fi
    fi
    
    # Clean up other generated files from source directory
    rm -f "$test_dir/${test_base}.instructions"
    rm -f "$test_dir/${test_base}.pyc"
    rm -f "$test_dir/${test_base}.s"
    rm -f "$test_dir/${test_base}" # binary executable
    
    # Step 2: Compile LLVM IR to assembly
    echo -e "  🔧 Compiling to assembly..."
    if ! llc "$ll_file" -o "$asm_file"; then
        echo -e "${RED}  ❌ Failed to compile LLVM IR to assembly${NC}"
        ((FAILED_TESTS++))
        return 1
    fi
    
    # Step 3: Compile assembly to executable
    echo -e "  🔗 Linking executable..."
    if ! gcc "$asm_file" -o "$exe_file"; then
        echo -e "${RED}  ❌ Failed to link executable${NC}"
        ((FAILED_TESTS++))
        return 1
    fi
    
    # Step 4: Run executable and capture output
    echo -e "  ▶️  Running executable..."
    if ! "$exe_file" > "$actual_file" 2>&1; then
        echo -e "${RED}  ❌ Failed to execute program${NC}"
        ((FAILED_TESTS++))
        return 1
    fi
    
    # Step 5: Compare output
    local expected_output=$(cat "$expected_file")
    local actual_output=$(cat "$actual_file")
    
    if [[ "$expected_output" == "$actual_output" ]]; then
        echo -e "${GREEN}  ✅ PASSED - Output: $actual_output${NC}"
        ((PASSED_TESTS++))
        # Clean up actual file on success
        rm -f "$actual_file"
        return 0
    else
        echo -e "${RED}  ❌ FAILED${NC}"
        echo -e "    Expected: '$expected_output'"
        echo -e "    Actual:   '$actual_output'"
        ((FAILED_TESTS++))
        return 1
    fi
}

# Clean artifacts directory and source directory
echo -e "${YELLOW}Cleaning artifacts and source directories...${NC}"
rm -rf "$ARTIFACTS_DIR"/*
# Clean any leftover artifacts from source directory
find "$PYTHON_FILES_DIR" -name "*.ll" -o -name "*.s" -o -name "*.instructions" -o -name "*.pyc" -o \( -name "test_*" ! -name "*.py" ! -name "*.expected" \) -exec rm -f {} \; 2>/dev/null || true
echo ""

# Run all tests
for test_file in "$PYTHON_FILES_DIR"/*.py; do
    if [[ -f "$test_file" ]]; then
        ((TOTAL_TESTS++))
        run_test "$test_file"
        echo ""
    fi
done

# Print summary
echo -e "${BLUE}📊 Test Results Summary${NC}"
echo -e "Total tests: $TOTAL_TESTS"
echo -e "${GREEN}Passed: $PASSED_TESTS${NC}"
if [[ $FAILED_TESTS -gt 0 ]]; then
    echo -e "${RED}Failed: $FAILED_TESTS${NC}"
else
    echo -e "Failed: 0"
fi

# Exit with appropriate code
if [[ $FAILED_TESTS -eq 0 && $TOTAL_TESTS -gt 0 ]]; then
    echo -e "\n${GREEN}🎉 All tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}💥 Some tests failed!${NC}"
    exit 1
fi
