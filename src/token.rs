use std::fmt::{Debug, Formatter, Result};

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Unvariable,

    Identifier,
    Number,

    BlockStart,
    BlockEnd,
    Addition,
    Subtraction,
    Division,
    Multiplication,

    ParentL,
    ParentR,
}

#[derive(Clone)]
pub struct Token {
    pub raw: String,
    pub r#type: Type,
}

impl Token {
    pub fn new(raw: String, r#type: Type) -> Self {
        Self { raw, r#type }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Token")
            .field("raw", &self.raw)
            .field("type", &self.r#type)
            .finish()
    }
}
