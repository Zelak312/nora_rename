use std::rc::Rc;

use crate::{
    errors::{BasicError, Error},
    tokenizer::token::{Token, TokenType},
};

use super::{base_parser::BaseParser, nodes};

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
            if token.r#type == TokenType::BlockStart {
                break;
            }

            content += &token.raw;
            self.base_parser.chain_reader.advance();
        }

        content
    }

    pub fn parse(&mut self) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        let token = self.base_parser.any()?;
        match token.r#type {
            TokenType::BlockStart => {
                let content = self.parse_inner_block()?;
                let mut block_node = nodes::NodeBlock {
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
                let mut content_node = nodes::NodeContent {
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

    pub fn parse_identifier(&mut self) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        let token = self.base_parser.expect(TokenType::Identifier)?;
        Ok(Rc::new(nodes::NodeIdentifer { content: token.raw }))
    }

    pub fn parse_string(&mut self) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        let token = self.base_parser.expect(TokenType::String)?;
        Ok(Rc::new(nodes::NodeString { content: token.raw }))
    }

    pub fn parse_number(&mut self) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        // check if it's an unary
        let unary = self
            .base_parser
            .expect_m(vec![TokenType::Addition, TokenType::Subtraction]);
        let token = self.base_parser.expect(TokenType::Number)?;
        let content = (unary.map_or("".to_owned(), |t| t.raw).to_owned() + &token.raw)
            .parse::<f64>()
            .map_err(|_| BasicError::new("ss".to_owned()))?;
        Ok(Rc::new(nodes::NodeNumber { content }))
    }

    pub fn parse_basic_type(&mut self) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        let keyword = self.parse_keyword();
        if keyword.is_ok() {
            return keyword;
        }

        let identifer = self.parse_identifier();
        if identifer.is_ok() {
            return identifer;
        }

        let string = self.parse_string();
        if string.is_ok() {
            return string;
        }

        self.parse_number()
    }

    pub fn parse_keyword(&mut self) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        let keyword = self
            .base_parser
            .expect_m(vec![TokenType::KeyNumber, TokenType::KeyString])?;
        self.base_parser.expect(TokenType::ParentL)?;
        let content = self.parse_ternary()?;
        self.base_parser.expect(TokenType::ParentR)?;

        Ok(Rc::new(nodes::NodeKeyword {
            keyword: keyword.r#type,
            content,
        }))
    }

    pub fn parse_binary_operation(
        &mut self,
    ) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        let mut left = self.parse_binary_pow_div()?;
        while let Ok(operator) = self
            .base_parser
            .expect_m(vec![TokenType::Addition, TokenType::Subtraction])
        {
            let right = self.parse_binary_pow_div()?;
            left = Rc::new(nodes::NodeBinaryOperator {
                operator: operator.r#type,
                left,
                right,
            });
        }

        Ok(left)
    }

    pub fn parse_binary_pow_div(
        &mut self,
    ) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        let mut left = self.parse_binary_parenthese()?;
        while let Ok(operator) = self
            .base_parser
            .expect_m(vec![TokenType::Multiplication, TokenType::Division])
        {
            let right = self.parse_binary_parenthese()?;
            left = Rc::new(nodes::NodeBinaryOperator {
                operator: operator.r#type,
                left,
                right,
            });
        }

        Ok(left)
    }

    pub fn parse_binary_parenthese(
        &mut self,
    ) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        if let Ok(_) = self.base_parser.expect(TokenType::ParentL) {
            let math = self.parse_ternary()?;
            self.base_parser.expect(TokenType::ParentR)?;
            return Ok(math);
        }

        self.parse_basic_type()
    }

    pub fn parse_inner_block(&mut self) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        let node = self.parse_ternary()?;
        self.base_parser.expect(TokenType::BlockEnd)?;
        Ok(node)
    }

    pub fn parse_ternary(&mut self) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        let condition = self.parse_condition()?;
        if self.base_parser.expect(TokenType::QuestionMark).is_ok() {
            let left = self.parse_ternary()?;
            self.base_parser.expect(TokenType::Semicolon)?;
            let right = self.parse_ternary()?;

            return Ok(Rc::new(nodes::NodeTernary {
                condition,
                left,
                right,
            }));
        }

        Ok(condition)
    }

    pub fn parse_condition(&mut self) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        let mut left = self.parse_binary_operation()?;
        while let Ok(operator) = self.base_parser.expect_m(vec![
            TokenType::LessThanSign,
            TokenType::LessThanEqualSign,
            TokenType::GreaterThanSign,
            TokenType::GreaterThanEqualSign,
            TokenType::DoubleEqualSign,
        ]) {
            let right = self.parse_binary_operation()?;
            left = Rc::new(nodes::NodeCondition {
                operator: operator.r#type,
                left,
                right,
            });
        }

        Ok(left)
    }
}
