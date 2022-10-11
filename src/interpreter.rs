use std::rc::Rc;

use regex::Captures;

use crate::{
    ast::{
        NodeBinaryOperator, NodeBlock, NodeCondition, NodeContent, NodeIdentifer, NodeNumber,
        NodeString, NodeTernary,
    },
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
        let left_o = self.left.execute(interpreter)?;
        match left_o {
            ExecutableNodeReturn::Number(n) => {
                let rigth = self.right.execute(interpreter)?.to_number()?;
                let out = match self.operator {
                    Type::Addition => n + rigth,
                    Type::Subtraction => n - rigth,
                    Type::Multiplication => n * rigth,
                    Type::Division => n / rigth,
                    _ => panic!("Operator not found (this shouldn't be panicing!"),
                };

                return Ok(ExecutableNodeReturn::Number(out));
            }
            ExecutableNodeReturn::String(n) => {
                let rigth = self.right.execute(interpreter)?.to_string()?;
                let out = match self.operator {
                    Type::Addition => n + &rigth,
                    _ => panic!("Operator not found (this shouldn't be panicing!)"),
                };

                return Ok(ExecutableNodeReturn::String(out));
            }
            _ => panic!("Cannot do binary operation on this type"),
        }
    }
}

impl ExecutableNode for NodeBlock {
    fn execute(
        &self,
        interpreter: &mut Interpreter,
    ) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        let mut out = self.content.execute(interpreter)?.to_string()?;
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

impl ExecutableNode for NodeString {
    fn execute(&self, _: &mut Interpreter) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        Ok(ExecutableNodeReturn::String(self.content.clone()))
    }
}

impl ExecutableNode for NodeNumber {
    fn execute(&self, _: &mut Interpreter) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        Ok(ExecutableNodeReturn::Number(self.content.clone()))
    }
}

impl ExecutableNode for NodeCondition {
    fn execute(&self, i: &mut Interpreter) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        self.left.execute(i)?.eqq(self.right.execute(i)?)
    }
}
impl ExecutableNode for NodeTernary {
    fn execute(&self, i: &mut Interpreter) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        let cond = self.condition.execute(i)?.bool()?;
        if cond {
            return self.left.execute(i);
        }

        return self.right.execute(i);
    }
}
