use crate::{node::Node, token::Type};
use std::fmt::Debug;

#[derive(Debug)]
pub struct NodeBinaryOperator {
    operator: Type,
    left: Box<Node>,
    right: Box<Node>,
}

impl NodeBinaryOperator {
    pub fn new(operator: Type, left: Box<Node>, rigth: Box<Node>) -> Self {
        Self {
            operator,
            left,
            right: rigth,
        }
    }
}

#[derive(Debug)]
pub struct NodeBlock {
    left: Box<Node>,
    right: Option<Box<Node>>,
}

impl NodeBlock {
    pub fn new(left: Box<Node>) -> Self {
        Self { left, right: None }
    }

    pub fn set_right(&mut self, right: Box<Node>) {
        self.right = Some(right)
    }
}

#[derive(Debug)]
pub struct NodeContent {
    content: String,
    next: Option<Box<Node>>,
}

impl NodeContent {
    pub fn new(content: String) -> Self {
        Self {
            content,
            next: None,
        }
    }

    pub fn set_next(&mut self, node: Box<Node>) {
        self.next = Some(node);
    }
}

#[derive(Debug)]
pub struct NodeIdentifer {
    content: String,
}

impl NodeIdentifer {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

#[derive(Debug)]
pub struct NodeNumber {
    content: f64,
}

impl NodeNumber {
    pub fn new(content: String) -> Self {
        let num: f64 = content.parse().unwrap();
        Self { content: num }
    }
}
