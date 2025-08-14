# PyTC

Python compiler written in Rust that transpiles Python bytecode to LLVM IR for optimized native compilation.

The tool's goal is to speed up Python code by:

1. Generating Python bytecode through CPython's .pycache files
2. Parsing and mapping the bytecode structure in Rust
3. Converting the bytecode to LLVM Intermediate Representation (IR)
4. Using LLVM's powerful optimization passes to generate highly optimized native code

This approach leverages LLVM's mature optimization pipeline and cross-platform capabilities, allowing for efficient native code generation across different architectures.

## Requirements:

- Python 3.10.2 (CPython implementation)
- LLVM 16.0.4

## Todo:

- [x] Generation and decompilation of .pycache files based on the .py files
- [x] Variable declarations
- [x] Integer and variable additions
- [x] Print statements
- [ ] Mapping of all primitive and non-primitive types
- [ ] Mapping of all operations
- [ ] User-defined functions
- [ ] Python standard library mapping
- [ ] Imports and modules
- [ ] Comprehensive test suite
- [ ] Benchmarks

## References:

### Examples

- [Python bytecode explanation on example 1](https://reverseengineering.stackexchange.com/questions/21085/the-structure-of-the-pythons-marshaled-code-object-or-pyc-file)
- [Python bytecode explanation on example 2](https://stackoverflow.com/questions/16064409/how-to-create-a-code-object-in-python/16123158#16123158)
- [Article on bytecode decompilation](https://medium.com/@skuznetsov/understanding-python-byte-code-and-decompilation-a-comprehensive-guide-a35a9c1329cb)

### Docs

- [Opcodes](https://github.com/python/cpython/blob/main/Include/opcode.h)
- [Opcode Ids](https://github.com/python/cpython/blob/main/Include/opcode_ids.h)
- [Code object bit flags](https://docs.python.org/3/library/inspect.html#inspect-module-co-flags)
- [Exception handling encoding](https://github.com/python/cpython/blob/main/Objects/exception_handling_notes.txt)
- [Bytecode address-to-line encoding](https://github.com/python/cpython/blob/main/Objects/locations.md)
- [Python bytecode instructions](https://docs.python.org/3/library/dis.html#python-bytecode-instructions)
- https://github.com/python/cpython/blob/main/Python/marshal.c
- https://github.com/python/cpython/blob/main/Include/cpython/code.h

### Random

- [Unicode character values lookup](https://unicodelookup.com/#c/2)
- https://github.com/ThePrimeagen/ts-rust-zig-deez/blob/master/rust/src/lexer/lexer.rs#L175
- https://github.com/ThePrimeagen/ts-rust-zig-deez/blob/master/python/deez_py/tokens.py
- https://nedbatchelder.com/blog/200804/the_structure_of_pyc_files.html
- https://nowave.it/python-bytecode-analysis-1.html
