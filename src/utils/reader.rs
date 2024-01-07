use super::{operations::Operation, types::TypeIdentifier};

#[derive(Default, Debug)]
pub struct Reader {
    pub current_idx: usize,
    pub contents: Vec<u8>,
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

    pub fn next(&mut self) {
        // TODO: EOF case
        self.current_idx += 1
    }

    pub fn jump(&mut self, jump: usize) {
        // TODO: EOF case
        self.current_idx += jump
    }

    pub fn is_eof(&self) -> bool {
        self.current_idx >= self.contents.len()
    }

    pub fn read(&mut self, bytes_amnt: i32) -> &[u8] {
        // TODO: EOF case
        &self.contents[self.current_idx..self.current_idx + bytes_amnt as usize]
    }

    pub fn read_byte(&mut self) -> u8 {
        // TODO: EOF case
        let byte = self.contents[self.current_idx];
        self.next();
        byte
    }

    pub fn read_long(&mut self) -> i32 {
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

    pub fn read_instruction(&mut self) -> Option<Operation> {
        // TODO: EOF case
        let byte = *self
            .get()
            .unwrap_or_else(|| panic!("reading byte on idx {}", self.current_idx));
        self.next();
        Operation::from_byte(&byte, self)
    }

    pub fn read_var(&mut self) -> Option<TypeIdentifier> {
        let byte = *self
            .get()
            .unwrap_or_else(|| panic!("reading byte on idx {}", self.current_idx));
        println!("{}", byte);
        self.next();
        TypeIdentifier::from_byte(&byte, self)
    }
}
