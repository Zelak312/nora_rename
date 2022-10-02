use crate::node::Node;
use std::fmt::{Debug, Formatter, Result};

pub struct NodeContent {}

impl NodeContent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Node for NodeContent {}

impl Debug for NodeContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("NodeIdentifier").finish()
    }
}
