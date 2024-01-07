use super::{operations::Operation, var::Var};

#[derive(Default, Debug)]
pub struct CodeBlock {
    pub co_argcount: i32,       // Number of positional arguments
    pub co_kwonlyargcount: i32, // Number of keyworded arguments
    pub co_nlocals: i32,        // Number of local variables, including args and kwargs
    pub co_posonlyargcount: i32,
    pub co_stacksize: i32,
    pub co_flags: i32,
    pub co_code: Vec<Operation>,
    pub co_const: Box<Var>,
    pub co_names: Box<Var>,
    pub co_varnames: Box<Var>,
    pub co_freevars: Box<Var>,
    pub co_cellvars: Box<Var>,
    pub co_filename: Box<Var>,
    pub co_name: Box<Var>,
    pub co_firstlineno: i32,
    pub co_lnotab: Box<Var>,
}

#[derive(Debug)]
pub struct Reader {
    pub current_idx: usize,
    pub contents: Vec<u8>,
    pub last_operation: String, // TODO: Make this private
}

impl Reader {
    pub fn get_current_idx(&self) -> usize {
        self.current_idx
    }

    pub fn get(&self) -> Option<&u8> {
        self.contents.get(self.current_idx)
    }

    pub fn get_by_idx(&self, idx: usize) -> Option<&u8> {
        self.contents.get(idx)
    }

    pub fn jump(&mut self, jump: usize) {
        // > instead of >= to be able to read the last byte from file
        if self.current_idx + jump > self.contents.len() {
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

    pub fn get_error_msg(&self) -> String {
        format!(
            "Attempting to {} b'{}' on idx {}",
            self.last_operation, self.contents[self.current_idx], self.current_idx
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
        self.set_last_operation("read byte or char");
        let byte = self.contents[self.current_idx];
        self.next();
        byte
    }

    pub fn read_char(&mut self) -> char {
        self.read_byte() as char
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
        let code = self
            .read_var()
            .unwrap_or_else(|| panic!("{}", self.get_error_msg()));

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

impl Default for Reader {
    fn default() -> Self {
        Reader {
            current_idx: 0,
            contents: Vec::new(),
            last_operation: "init".to_string()
        }
    }
}