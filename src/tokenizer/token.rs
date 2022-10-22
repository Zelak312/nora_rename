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
    QuestionMarkGreaterThan,

    KeyNumber,
    KeyString,
}

#[derive(Clone)]
pub struct Token {
    pub content: String,
    pub r#type: TokenType,
    pub start: usize,
    pub length: usize,
}

impl Token {
    pub fn new(raw: &str, r#type: TokenType, start: usize, length: usize) -> Self {
        Self {
            content: raw.to_owned(),
            r#type,
            start,
            length,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Token")
            .field("raw", &self.content)
            .field("type", &self.r#type)
            .finish()
    }
}
