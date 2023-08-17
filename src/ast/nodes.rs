use crate::{
    ast::interpreter::Interpreter, errors::Error, library::object_type::ObjectType,
    tokenizer::token::TokenType,
};
use std::{fmt::Debug, rc::Rc};

pub trait ExecutableNode: Debug {
    fn execute(&self, _: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>>;
}

#[derive(Debug)]
pub struct NodeBinaryOperator {
    pub operator: TokenType,
    pub left: Rc<dyn ExecutableNode>,
    pub right: Rc<dyn ExecutableNode>,
}

#[derive(Debug)]
pub struct NodeBlock {
    pub content: Rc<dyn ExecutableNode>,
    pub next: Option<Rc<dyn ExecutableNode>>,
}

#[derive(Debug)]
pub struct NodeFor {
    pub identifer: Rc<dyn ExecutableNode>,
    pub from: Rc<dyn ExecutableNode>,
    pub to: Rc<dyn ExecutableNode>,
    pub content: Rc<dyn ExecutableNode>,
}

#[derive(Debug)]
pub struct NodeCondition {
    pub operator: TokenType,
    pub left: Rc<dyn ExecutableNode>,
    pub right: Rc<dyn ExecutableNode>,
}

#[derive(Debug)]
pub struct NodeContent {
    pub content: String,
    pub next: Option<Rc<dyn ExecutableNode>>,
}

#[derive(Debug)]
pub struct NodeIdentifer {
    pub content: String,
    pub use_for_name: bool,
}

#[derive(Debug)]
pub struct NodeIdentiferIndexer {
    pub index: String,
    pub optional: bool,
}

#[derive(Debug)]
pub struct NodeKeyword {
    pub keyword: TokenType,
    pub content: Rc<dyn ExecutableNode>,
    pub options: Vec<Rc<dyn ExecutableNode>>,
}

#[derive(Debug)]
pub struct NodeNumber {
    pub content: f64,
}

#[derive(Debug)]
pub struct NodeString {
    pub content: String,
}

#[derive(Debug)]
pub struct NodeTernary {
    pub condition: Rc<dyn ExecutableNode>,
    pub left: Rc<dyn ExecutableNode>,
    pub right: Rc<dyn ExecutableNode>,
}
