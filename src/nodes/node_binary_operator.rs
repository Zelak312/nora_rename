use crate::{node::Node, token::Type};
use std::fmt::{Debug, Formatter, Result};

pub struct NodeBinaryOperator {
    operator: Type,
    left: Box<dyn Node>,
    rigth: Box<dyn Node>,
}

impl NodeBinaryOperator {
    pub fn new(operator: Type, left: Box<dyn Node>, rigth: Box<dyn Node>) -> Self {
        Self {
            operator,
            left,
            rigth,
        }
    }
}

impl Node for NodeBinaryOperator {}

impl Debug for NodeBinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("NodeIdentifier")
            .field("operator", &self.operator)
            .finish()
    }
}
