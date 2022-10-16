use std::fmt::{Debug, Display};

use owo_colors::OwoColorize;
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
pub struct LinePointingError {
    msg: String,
    code: String,
    point_start: usize,
    point_length: usize,
}

impl LinePointingError {
    pub fn new(msg: &str, code: &str, point_start: usize, point_length: usize) -> Box<Self> {
        Box::new(Self {
            msg: msg.to_owned(),
            code: code.to_owned(),
            point_start,
            point_length,
        })
    }
}

impl Error for LinePointingError {
    fn message(&self) -> String {
        format!(
            "{}: {}\n\t{}\n\t{}{}",
            "error".red(),
            self.msg,
            self.code,
            " ".repeat(self.point_start),
            "^".repeat(self.point_length).red()
        )
    }
}
