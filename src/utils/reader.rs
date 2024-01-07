use super::{operations::Operation, var::Var};

#[derive(Default, Debug)]
pub struct Reader {
    pub current_idx: usize,
    pub contents: Vec<u8>,
    pub last_operation: String // TODO: Make this private
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
            "Attempting to {} {} on idx {}",
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

    pub fn read_string(&mut self) -> String {
        self.set_last_operation("read string");
        let len = self.read_byte();
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
}
