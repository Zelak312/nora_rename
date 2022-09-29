use std::fmt::{Debug, Formatter, Result};

pub struct ChainReader<T>
where
    T: Copy,
{
    pos: usize,
    items: Vec<T>,
}

impl<T> ChainReader<T>
where
    T: Copy,
{
    pub fn new(items: Vec<T>) -> Self {
        Self {
            pos: 0,
            items: items,
        }
    }

    pub fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn get_current(&mut self) -> Option<T> {
        if self.pos >= self.items.len() {
            return None;
        }

        Some(self.items[self.pos])
    }
}

impl<T> Debug for ChainReader<T>
where
    T: Copy + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("StringReader")
            .field("pos", &self.pos)
            .finish()
        // .field("items", &self.items)
    }
}
