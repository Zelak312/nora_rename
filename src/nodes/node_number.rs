use crate::node::Node;
use std::fmt::{Debug, Formatter, Result};

pub struct NodeNumber {
    content: f64,
}

impl NodeNumber {
    pub fn new(content: String) -> Self {
        let num: f64 = content.parse().unwrap();
        Self { content: num }
    }
}

impl Node for NodeNumber {}

impl Debug for NodeNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("NodeIdentifier").finish()
    }
}
