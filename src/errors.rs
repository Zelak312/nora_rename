use std::fmt::{Debug, Display};

use crate::token::TokenType;

pub trait Error: Debug {
    fn message(&self) -> String;
}

impl Display for dyn Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl From<Box<BasicError>> for Box<dyn Error> {
    fn from(b: Box<BasicError>) -> Self {
        b
    }
}

impl From<Box<UnexpectedError>> for Box<dyn Error> {
    fn from(b: Box<UnexpectedError>) -> Self {
        b
    }
}

impl From<Box<UnexpectedEndOfFile>> for Box<dyn Error> {
    fn from(b: Box<UnexpectedEndOfFile>) -> Self {
        b
    }
}

#[derive(Debug)]
pub struct BasicError {
    msg: String,
}

impl BasicError {
    pub fn new(msg: String) -> Box<Self> {
        Box::new(Self { msg })
    }
}

impl Error for BasicError {
    fn message(&self) -> String {
        self.msg.clone()
    }
}

#[derive(Debug)]
pub struct UnexpectedError {
    msg: String,
}

impl UnexpectedError {
    pub fn new(found: TokenType, expected: TokenType) -> Box<Self> {
        Box::new(Self {
            msg: format!("Unexpected: {:?}\nExpected: {:?}", found, expected),
        })
    }

    pub fn new_m(found: TokenType, expected: Vec<TokenType>) -> Box<Self> {
        let mut list = format!("{:?}", expected[0]);
        for i in 1..expected.len() {
            list += &format!(", {:?}", expected[i]);
        }

        Box::new(Self {
            msg: format!("Unexpected: {:?}\nExpected: {:?}", found, list),
        })
    }
}

impl Error for UnexpectedError {
    fn message(&self) -> String {
        self.msg.clone()
    }
}

#[derive(Debug)]
pub struct UnexpectedEndOfFile {
    msg: String,
}

impl UnexpectedEndOfFile {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            msg: format!("Unexpected end of file"),
        })
    }
}

impl Error for UnexpectedEndOfFile {
    fn message(&self) -> String {
        self.msg.clone()
    }
}
