use crate::{node::Node, token::Type};
use std::fmt::Debug;

#[derive(Debug)]
pub struct NodeBinaryOperator {
    pub operator: Type,
    pub left: Box<Node>,
    pub right: Box<Node>,
}

#[derive(Debug)]
pub struct NodeBlock {
    pub content: Box<Node>,
    pub next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct NodeContent {
    pub content: String,
    pub next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct NodeIdentifer {
    pub content: String,
}

#[derive(Debug)]
pub struct NodeNumber {
    pub content: f64,
}
