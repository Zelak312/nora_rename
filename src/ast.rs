use crate::{node::ExecutableNode, token::Type};
use std::{fmt::Debug, rc::Rc};

#[derive(Debug)]
pub struct NodeBinaryOperator {
    pub operator: Type,
    pub left: Rc<dyn ExecutableNode>,
    pub right: Rc<dyn ExecutableNode>,
}

#[derive(Debug)]
pub struct NodeBlock {
    pub content: Rc<dyn ExecutableNode>,
    pub next: Option<Rc<dyn ExecutableNode>>,
}

#[derive(Debug)]
pub struct NodeContent {
    pub content: String,
    pub next: Option<Rc<dyn ExecutableNode>>,
}

#[derive(Debug)]
pub struct NodeIdentifer {
    pub content: String,
}

#[derive(Debug)]
pub struct NodeNumber {
    pub content: f64,
}
