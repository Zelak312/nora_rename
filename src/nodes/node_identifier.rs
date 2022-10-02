use crate::node::Node;
use std::fmt::{Debug, Formatter, Result};

pub struct NodeIdentifer {
    content: String,
}

impl NodeIdentifer {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl Node for NodeIdentifer {}

impl Debug for NodeIdentifer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("NodeIdentifier").finish()
    }
}
