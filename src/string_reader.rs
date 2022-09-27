use std::fmt::{Debug, Formatter, Result};

pub struct StringReader {
    pos: usize,
    chars: Vec<char>,
}

impl StringReader {
    pub fn new(code: String) -> Self {
        let chars = code.chars().collect::<Vec<char>>();

        Self { pos: 0, chars }
    }

    pub fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn get_current(&mut self) -> Option<char> {
        if self.pos >= self.chars.len() {
            return None;
        }

        Some(self.chars[self.pos])
    }
}

impl Debug for StringReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("StringReader")
            .field("pos", &self.pos)
            .field("chars", &self.chars)
            .finish()
    }
}
