use std::rc::Rc;

use crate::{
    ast::{NodeBinaryOperator, NodeBlock, NodeContent, NodeIdentifer, NodeNumber},
    base_parser::BaseParser,
    errors::{BasicError, Error},
    node::ExecutableNode,
    token::{Token, Type},
};

pub struct Parser {
    base_parser: BaseParser,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            base_parser: BaseParser::new(tokens),
        }
    }

    pub fn content_all(&mut self, start: &str) -> String {
        let mut content = String::from(start);
        while let Some(token) = self.base_parser.chain_reader.get_current() {
            if token.r#type == Type::BlockStart {
                break;
            }

            content += &token.raw;
            self.base_parser.chain_reader.advance();
        }

        content
    }

    pub fn parse(&mut self) -> Result<Rc<dyn ExecutableNode>, Box<dyn Error>> {
        let token = self.base_parser.any()?;
        match token.r#type {
            Type::BlockStart => {
                let content = self.parse_inner_block()?;
                let mut block_node = NodeBlock {
                    content,
                    next: None,
                };
                if let Ok(node) = self.parse() {
                    block_node.next = Some(node);
                }

                Ok(Rc::new(block_node))
            }
            _ => {
                let content = self.content_all(&token.raw);
                let mut content_node = NodeContent {
                    content,
                    next: None,
                };
                if let Ok(node) = self.parse() {
                    content_node.next = Some(node);
                }

                Ok(Rc::new(content_node))
            }
        }
    }

    pub fn parse_identifier(&mut self) -> Result<Rc<dyn ExecutableNode>, Box<dyn Error>> {
        let token = self.base_parser.expect(Type::Identifier)?;
        Ok(Rc::new(NodeIdentifer { content: token.raw }))
    }

    pub fn parse_number(&mut self) -> Result<Rc<dyn ExecutableNode>, Box<dyn Error>> {
        let token = self.base_parser.expect(Type::Number)?;
        let content = token
            .raw
            .parse::<f64>()
            .map_err(|_| BasicError::new("ss".to_owned()))?;
        Ok(Rc::new(NodeNumber { content }))
    }

    pub fn parse_basic_type(&mut self) -> Result<Rc<dyn ExecutableNode>, Box<dyn Error>> {
        let identifer = self.parse_identifier();
        if identifer.is_ok() {
            return identifer;
        }

        self.parse_number()
    }

    pub fn parse_binary_operation(&mut self) -> Result<Rc<dyn ExecutableNode>, Box<dyn Error>> {
        let left = self.parse_binary_pow_div()?;
        let operator = self
            .base_parser
            .expect_m(vec![Type::Addition, Type::Subtraction]);

        if operator.is_err() {
            return Ok(left);
        }

        let right = self.parse_binary_operation()?;
        let binary = NodeBinaryOperator {
            operator: operator.unwrap().r#type,
            left,
            right,
        };
        Ok(Rc::new(binary))
    }

    pub fn parse_binary_pow_div(&mut self) -> Result<Rc<dyn ExecutableNode>, Box<dyn Error>> {
        let left = self.parse_binary_parenthese()?;
        let operator = self
            .base_parser
            .expect_m(vec![Type::Multiplication, Type::Division]);

        if operator.is_err() {
            return Ok(left);
        }

        let right = self.parse_binary_pow_div()?;
        let binary = NodeBinaryOperator {
            operator: operator.unwrap().r#type,
            left,
            right,
        };

        Ok(Rc::new(binary))
    }

    pub fn parse_binary_parenthese(&mut self) -> Result<Rc<dyn ExecutableNode>, Box<dyn Error>> {
        if let Ok(_) = self.base_parser.expect(Type::ParentL) {
            let math = self.parse_binary_operation()?;
            self.base_parser.expect(Type::ParentR)?;
            return Ok(math);
        }

        self.parse_basic_type()
    }

    pub fn parse_inner_block(&mut self) -> Result<Rc<dyn ExecutableNode>, Box<dyn Error>> {
        let node = self.parse_binary_operation()?;
        self.base_parser.expect(Type::BlockEnd)?;
        Ok(node)
    }
}
