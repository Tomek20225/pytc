use super::operations::Operation;

#[derive(Default)]
#[derive(Debug)]
pub struct Reader {
    pub current_idx: usize,
    pub contents: Vec<u8> 
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
        return byte;
    }

    pub fn read_long(&mut self) -> i32 {
        // TODO: EOF case
        let long = i32::from_le_bytes([
            self.contents[self.current_idx],
            self.contents[self.current_idx + 1],
            self.contents[self.current_idx + 2],
            self.contents[self.current_idx + 3],
        ]);
        self.jump(4);
        return long;
    }

    pub fn read_instruction(&mut self) -> Option<Operation> {
        let byte = *self.get().expect(&format!("reading byte on idx {}", self.current_idx));
        self.next();
        Operation::from_byte(&byte, self)
    }
}