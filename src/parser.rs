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

    pub fn parse(&mut self) -> Result<Box<dyn ExecutableNode>, Box<dyn Error>> {
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

                Ok(Box::new(block_node))
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

                Ok(Box::new(content_node))
            }
        }
    }

    pub fn parse_identifier(&mut self) -> Result<Box<dyn ExecutableNode>, Box<dyn Error>> {
        let token = self.base_parser.expect(Type::Identifier)?;
        Ok(Box::new(NodeIdentifer { content: token.raw }))
    }

    pub fn parse_number(&mut self) -> Result<Box<dyn ExecutableNode>, Box<dyn Error>> {
        let token = self.base_parser.expect(Type::Number)?;
        let content = token
            .raw
            .parse::<f64>()
            .map_err(|_| BasicError::new("ss".to_owned()))?;
        Ok(Box::new(NodeNumber { content }))
    }

    pub fn parse_basic_type(&mut self) -> Result<Box<dyn ExecutableNode>, Box<dyn Error>> {
        let identifer = self.parse_identifier();
        if identifer.is_ok() {
            return identifer;
        }

        self.parse_number()
    }

    pub fn parse_binary_operation(&mut self) -> Result<Box<dyn ExecutableNode>, Box<dyn Error>> {
        let left = self.parse_basic_type()?;
        let operator =
            self.base_parser
                .expect_m(vec![Type::Plus, Type::Minus, Type::Mul, Type::Div]);

        if operator.is_err() {
            return Ok(left);
        }

        let right = self.parse_binary_operation()?;
        let binary = NodeBinaryOperator {
            operator: operator.unwrap().r#type,
            left,
            right,
        };
        Ok(Box::new(binary))
    }

    pub fn parse_inner_block(&mut self) -> Result<Box<dyn ExecutableNode>, Box<dyn Error>> {
        let node = self.parse_binary_operation()?;
        self.base_parser.expect(Type::BlockEnd)?;
        Ok(node)
    }
}
