use std::rc::Rc;

use regex::Captures;

use crate::{
    ast::{NodeBinaryOperator, NodeBlock, NodeContent, NodeIdentifer, NodeNumber},
    errors::Error,
    node::{ExecutableNode, ExecutableNodeReturn},
    token::Type,
};

pub struct Interpreter<'t> {
    captures: Captures<'t>,
}

impl<'t> Interpreter<'t> {
    pub fn new(captures: Captures<'t>) -> Self {
        Self { captures }
    }

    pub fn execute(
        &mut self,
        node: Rc<dyn ExecutableNode>,
    ) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        node.execute(self)
    }
}

impl ExecutableNode for NodeBinaryOperator {
    fn execute(
        &self,
        interpreter: &mut Interpreter,
    ) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        let left = self.left.execute(interpreter)?.to_number()?;
        let rigth = self.right.execute(interpreter)?.to_number()?;

        let out = match self.operator {
            Type::Addition => left + rigth,
            Type::Subtraction => left - rigth,
            Type::Multiplication => left * rigth,
            Type::Division => left / rigth,
            _ => panic!("really bad"),
        };

        Ok(ExecutableNodeReturn::Number(out))
    }
}

impl ExecutableNode for NodeBlock {
    fn execute(
        &self,
        interpreter: &mut Interpreter,
    ) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        let mut out = match self.content.execute(interpreter)? {
            ExecutableNodeReturn::String(content) => content,
            ExecutableNodeReturn::Number(content) => content.to_string(),
        };

        if let Some(node) = &self.next {
            out += &node.execute(interpreter)?.string()?;
        }

        Ok(ExecutableNodeReturn::String(out))
    }
}

impl ExecutableNode for NodeContent {
    fn execute(
        &self,
        interpreter: &mut Interpreter,
    ) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        let mut out = String::from(&self.content);
        if let Some(node) = &self.next {
            out += &node.execute(interpreter)?.string()?;
        }

        Ok(ExecutableNodeReturn::String(out))
    }
}

impl ExecutableNode for NodeIdentifer {
    fn execute(&self, i: &mut Interpreter) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        let capture = i.captures.name(&self.content).expect("not here");
        Ok(ExecutableNodeReturn::String(String::from(capture.as_str())))
    }
}

impl ExecutableNode for NodeNumber {
    fn execute(&self, _: &mut Interpreter) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        Ok(ExecutableNodeReturn::Number(self.content.clone()))
    }
}
