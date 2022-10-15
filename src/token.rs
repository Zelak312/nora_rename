use std::fmt::{Debug, Formatter, Result};

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Unvariable,

    Identifier,
    String,
    Number,

    BlockStart,
    BlockEnd,
    Addition,
    Subtraction,
    Division,
    Multiplication,

    ParentL,
    ParentR,

    Semicolon,
    QuestionMark,
    ExclamationMark,

    EqualSign,
    DoubleEqualSign,
    NotEqualSign,
    LessThanSign,
    LessThanEqualSign,
    GreaterThanSign,
    GreaterThanEqualSign,

    KeyNumber,
    KeyString,
}

#[derive(Clone)]
pub struct Token {
    pub raw: String,
    pub r#type: TokenType,
}

impl Token {
    pub fn new(raw: String, r#type: TokenType) -> Self {
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
