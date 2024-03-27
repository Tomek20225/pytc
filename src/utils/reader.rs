use super::{code::CodeBlock, operations::Operation, var::Var};

// TODO: Consider pushing this, code and var modules as a single, separate package
// making it a dependency of llvm and transpiler modules

#[derive(Debug)]
pub struct Reader {
    current_idx: usize,
    contents: Vec<u8>,
    last_operation: String,
    refs: Vec<Var>,
}

impl Reader {
    pub fn new(contents: Vec<u8>) -> Reader {
        Reader {
            current_idx: 0,
            contents,
            last_operation: "init".to_string(),
            refs: Vec::new(),
        }
    }

    pub fn get_refs(&self) -> &Vec<Var> {
        &self.refs
    }

    pub fn get_refs_len(&self) -> usize {
        self.refs.len()
    }

    pub fn push_ref(&mut self, var: Var) -> usize {
        self.refs.push(var);
        self.refs.len() - 1
    }

    pub fn get_current_idx(&self) -> usize {
        self.current_idx
    }

    pub fn get(&self) -> Option<&u8> {
        // TODO: EOF Case
        self.contents.get(self.current_idx)
    }

    pub fn get_by_idx(&self, idx: usize) -> Option<&u8> {
        self.contents.get(idx)
    }

    pub fn jump(&mut self, jump: usize) {
        // > instead of >= to be able to read the last byte from file
        if self.is_eof() || self.current_idx + jump > self.contents.len() {
            panic!("Attempting to move the cursor beyond the file")
        }
        self.current_idx += jump
    }

    pub fn next(&mut self) {
        self.jump(1)
    }

    pub fn is_eof(&self) -> bool {
        self.current_idx >= self.contents.len()
    }

    // TODO: Make the last_operation a Vec containing the history of operations and calls within Reader
    pub fn get_error_msg(&self) -> String {
        let byte = self.contents[self.current_idx];
        format!(
            "Attempting to {} b'{}' (char: {}, hex: {:x?}) on idx {}",
            self.last_operation, byte, byte as char, byte, self.current_idx
        )
    }

    fn set_last_operation(&mut self, last_op: &str) {
        self.last_operation = last_op.to_string();
    }

    pub fn read_bytes(&mut self, bytes_amnt: i32) -> &[u8] {
        // TODO: EOF case
        self.set_last_operation("read bytes");
        &self.contents[self.current_idx..self.current_idx + bytes_amnt as usize]
    }

    pub fn read_byte(&mut self) -> u8 {
        // TODO: EOF case
        self.set_last_operation("read byte");
        let byte = self.contents[self.current_idx];
        self.next();
        byte
    }

    pub fn read_char(&mut self) -> char {
        // TODO: EOF case
        self.set_last_operation("read char");
        let chr = self.contents[self.current_idx] as char;
        self.next();
        chr
    }

    pub fn read_long(&mut self) -> i32 {
        self.set_last_operation("read long or int");

        // 4 bytes in 32-bit C and within CPython .pyc object
        // TODO: EOF case
        let long = i32::from_le_bytes([
            self.contents[self.current_idx],
            self.contents[self.current_idx + 1],
            self.contents[self.current_idx + 2],
            self.contents[self.current_idx + 3],
        ]);
        self.jump(4);
        long
    }

    pub fn read_ulong(&mut self) -> u32 {
        self.set_last_operation("read u_long");

        // 4 bytes in 32-bit C and within CPython .pyc object
        // TODO: EOF case
        let long = u32::from_le_bytes([
            self.contents[self.current_idx],
            self.contents[self.current_idx + 1],
            self.contents[self.current_idx + 2],
            self.contents[self.current_idx + 3],
        ]);
        self.jump(4);
        long
    }

    pub fn read_int(&mut self) -> i32 {
        // 4 bytes in 32-bit C and within CPython .pyc object
        self.read_long()
    }

    pub fn read_short_string(&mut self) -> String {
        self.set_last_operation("read short string");
        let len = self.read_byte();
        let mut str_res: String = String::from("");
        for _ in 0..len {
            let char: char = self.read_char();
            str_res.push(char);
        }
        str_res
    }

    pub fn read_string(&mut self) -> String {
        self.set_last_operation("read string");
        let len = self.read_ulong();
        let mut str_res: String = String::from("");
        for _ in 0..len {
            let char: char = self.read_char();
            str_res.push(char);
        }
        str_res
    }

    pub fn read_operation(&mut self) -> Option<Operation> {
        // TODO: EOF case
        self.set_last_operation("read operation");
        let byte = self.read_byte();
        Operation::from_byte(&byte, self)
    }

    pub fn read_var(&mut self) -> Option<Var> {
        self.set_last_operation("read var");
        let byte = self.read_byte();
        Var::from_byte(&byte, self)
    }

    pub fn read_tuple(&mut self) -> Vec<Var> {
        self.set_last_operation("read tuple");
        let len = self.read_byte();
        let mut tuple: Vec<Var> = Vec::new();
        for _ in 0..len {
            let byte = self.read_byte();
            let var =
                Var::from_byte(&byte, self).unwrap_or_else(|| panic!("{}", self.get_error_msg()));
            tuple.push(var);
        }
        tuple
    }

    pub fn read_code(&mut self) -> CodeBlock {
        let mut code = CodeBlock {
            ..Default::default()
        };

        // Static params
        code.co_argcount = self.read_long();
        code.co_kwonlyargcount = self.read_long();
        code.co_nlocals = self.read_long();
        code.co_posonlyargcount = self.read_long();
        code.co_stacksize = self.read_long();
        code.co_flags = self.read_long();
        self.next(); // skip 1 byte flag representing the co_code_size, i.e. 's' (string)
        let co_code_size = self.read_long();

        // Operations (next co_code_size bytes)
        let mut co_code: Vec<Operation> = Vec::new();
        let limit = self.current_idx + co_code_size as usize;
        while self.current_idx < limit {
            let operation = self
                .read_operation()
                .unwrap_or_else(|| panic!("{}", self.get_error_msg()));
            co_code.push(operation);
        }
        code.co_code = co_code;

        // co_const - tuple of typed variables, including CodeBlocks
        let co_const = self
            .read_var()
            .unwrap_or_else(|| panic!("{}", self.get_error_msg()));
        code.co_const = Box::new(co_const);

        // co_names - tuple of strings
        let co_names = self
            .read_var()
            .unwrap_or_else(|| panic!("{}", self.get_error_msg()));
        code.co_names = Box::new(co_names);

        // co_varnames
        let co_varnames = self
            .read_var()
            .unwrap_or_else(|| panic!("{}", self.get_error_msg()));
        code.co_varnames = Box::new(co_varnames);

        // co_freevars
        let co_freevars = self
            .read_var()
            .unwrap_or_else(|| panic!("{}", self.get_error_msg()));
        code.co_freevars = Box::new(co_freevars);

        // co_cellvars
        let co_cellvars = self
            .read_var()
            .unwrap_or_else(|| panic!("{}", self.get_error_msg()));
        code.co_cellvars = Box::new(co_cellvars);

        // co_filename
        let co_filename = self
            .read_var()
            .unwrap_or_else(|| panic!("{}", self.get_error_msg()));
        code.co_filename = Box::new(co_filename);

        // co_name
        let co_name = self
            .read_var()
            .unwrap_or_else(|| panic!("{}", self.get_error_msg()));
        code.co_name = Box::new(co_name);

        // co_firstlineno
        let co_firstlineno = self.read_long();
        code.co_firstlineno = co_firstlineno;

        // co_lnotab
        // Probably something wrong got read here, but this doesn't matter - it's unused
        // This should be a mapping of bytecode offset to line locations in Python file
        let co_lnotab = self
            .read_var()
            .unwrap_or_else(|| panic!("{}", self.get_error_msg()));
        code.co_lnotab = Box::new(co_lnotab);

        code
    }

    pub fn read_file(&mut self) -> Option<CodeBlock> {
        // Read the main block of code in the .pyc file
        let code = self
            .read_var()
            .unwrap_or_else(|| panic!("{}", self.get_error_msg()));

        // Go back to the beginning of the file
        self.current_idx = 0;

        // Proper .pyc file has to start with either a code block or a reference to it
        match code {
            Var::Code(code_block) => Some(code_block),
            Var::FlagRef(boxed_var) => match *boxed_var {
                Var::Code(code_block) => Some(code_block),
                _ => None,
            },
            _ => None,
        }
    }
}
