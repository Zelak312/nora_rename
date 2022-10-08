use crate::{
    ast::{NodeBinaryOperator, NodeBlock, NodeContent, NodeIdentifer, NodeNumber},
    errors::Error,
    node::{ExecutableNode, ExecutableNodeReturn},
    token::Type,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(
        &mut self,
        node: Box<dyn ExecutableNode>,
    ) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        node.execute(self)
    }
}

impl ExecutableNode for NodeBinaryOperator {
    fn execute(
        &self,
        interpreter: &mut Interpreter,
    ) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        let left = self.left.execute(interpreter)?.number()?;
        let rigth = self.right.execute(interpreter)?.number()?;

        let out = match self.operator {
            Type::Plus => left + rigth,
            Type::Minus => left - rigth,
            Type::Mul => left * rigth,
            Type::Div => left / rigth,
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
    fn execute(&self, _: &mut Interpreter) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        Ok(ExecutableNodeReturn::String(String::from(&self.content)))
    }
}

impl ExecutableNode for NodeNumber {
    fn execute(&self, _: &mut Interpreter) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        Ok(ExecutableNodeReturn::Number(self.content.clone()))
    }
}
