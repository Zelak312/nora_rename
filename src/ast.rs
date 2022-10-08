use crate::{node::ExecutableNode, token::Type};
use std::fmt::Debug;

#[derive(Debug)]
pub struct NodeBinaryOperator {
    pub operator: Type,
    pub left: Box<dyn ExecutableNode>,
    pub right: Box<dyn ExecutableNode>,
}

#[derive(Debug)]
pub struct NodeBlock {
    pub content: Box<dyn ExecutableNode>,
    pub next: Option<Box<dyn ExecutableNode>>,
}

#[derive(Debug)]
pub struct NodeContent {
    pub content: String,
    pub next: Option<Box<dyn ExecutableNode>>,
}

#[derive(Debug)]
pub struct NodeIdentifer {
    pub content: String,
}

#[derive(Debug)]
pub struct NodeNumber {
    pub content: f64,
}
