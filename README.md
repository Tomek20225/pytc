# PyTC

Python compiler written in Rust and Python.

**Developed and tested on Python 3.10.2 binaries**

The tool's original premise was to transpile generated Python bytecode (through .pycache files generated by CPython interpreter) to C code. The C would be then compiled to an executable through gcc with optimizations and linked to Python std library function equivalents.

However, assuming the goal is to speed up Python through compilation, there are other possible methods of achieving that:
1. Transpile the Python binary into Assembly directly. This shouldn't be as hard as it sounds, because instructions in Python binaries resemble Assembly behavior. However, as to my current knowledge, there are no decent Assembly optimizers that work as good as gcc, and developing the code for different platforms would be necessary.
2. Transpile the Python source code directly to C. For the most part it should be rather easy for basic code, but more advanced Python syntax may require plenty of code generation. Recognizing types may also turn out to be a pain.
3. Transpile the Python binary into LLVM. I'm not sure what the problems with this approach could be, except learning LLVM.
4. Create a Lexer to parse the Python source code into an AST and then reconstruct the AST in C. This approach may have similar drawbacks as direct source code manipulation, but basic cases should be easier to handle and more maintainable.


## Todo:
- [x] Create a Python script to generate the .pycache (.pyc) file
- [x] Map the basic Python binary in Rust
- [ ] Try transpilation with method 3
- [ ] Map the extended Python binary in Rust
- [ ] Create a more basic representation of the binary for further processing
- [ ] Try transpilation with the original method
- [ ] Try transpilation with method 1
- [ ] Try transpilation with method 2
- [ ] Try transpilation with method 4
- [ ] Finally compile the basic Python code
- [ ] Iterate and implement more advanced Python functionality

## References:
### Examples
- [Python bytecode explanation on example 1](https://reverseengineering.stackexchange.com/questions/21085/the-structure-of-the-pythons-marshaled-code-object-or-pyc-file)
- [Python bytecode explanation on example 2](https://stackoverflow.com/questions/16064409/how-to-create-a-code-object-in-python/16123158#16123158)

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
