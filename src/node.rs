use crate::ast::{NodeBinaryOperator, NodeBlock, NodeContent, NodeIdentifer, NodeNumber};

#[derive(Debug)]
pub enum Node {
    NodeBinaryOperator(NodeBinaryOperator),
    NodeBlock(NodeBlock),
    NodeContent(NodeContent),
    NodeIdentifer(NodeIdentifer),
    NodeNumber(NodeNumber),
}
