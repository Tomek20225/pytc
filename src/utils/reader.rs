#[derive(Default)]
#[derive(Debug)]
pub struct Reader {
    pub current_idx: usize,
    pub contents: Vec<u8> 
}

impl Reader {
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
}