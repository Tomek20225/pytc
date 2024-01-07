# PyTC

Python to C compiler written in Rust and Python.

It uses generated Python bytecode (.pyc files) to transpile the instructions to C code. The C code is then compiled to an executable through gcc.

## References:
- https://github.com/ThePrimeagen/ts-rust-zig-deez/blob/master/rust/src/lexer/lexer.rs#L175
- https://github.com/ThePrimeagen/ts-rust-zig-deez/blob/master/python/deez_py/tokens.py
- https://nedbatchelder.com/blog/200804/the_structure_of_pyc_files.html
- https://nowave.it/python-bytecode-analysis-1.html
- https://reverseengineering.stackexchange.com/questions/21085/the-structure-of-the-pythons-marshaled-code-object-or-pyc-file
- https://github.com/python/cpython/blob/main/Python/marshal.c
- https://github.com/python/cpython/blob/main/Include/cpython/code.h
- https://stackoverflow.com/questions/16064409/how-to-create-a-code-object-in-python/16123158#16123158
- [Opcodes](https://github.com/python/cpython/blob/main/Include/opcode.h)
- [Opcode Ids](https://github.com/python/cpython/blob/main/Include/opcode_ids.h)

## Todo:
- [x] Create a Python script to generate the .pyc file
- [x] Map the basic Python binary in Rust
- [ ] Transpile the basic results to C
- [ ] Map the extended Python binary in Rust
- [ ] Transpile the extended results to C