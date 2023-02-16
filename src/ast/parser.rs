use std::rc::Rc;

use owo_colors::OwoColorize;

use crate::{
    errors::{BasicError, Error, LinePointingError},
    tokenizer::token::{Token, TokenType},
    utils::string_utils,
};

use super::{
    base_parser::BaseParser,
    nodes::{self, NodeString},
};

pub struct Parser {
    base_parser: BaseParser,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, code: String) -> Self {
        Self {
            base_parser: BaseParser::new(tokens, code),
        }
    }

    pub fn content_all(&mut self, start: &str) -> String {
        let mut content = String::from(start);
        while let Some(token) = self.base_parser.chain_reader.get_current() {
            if token.r#type == TokenType::BlockStart {
                break;
            }

            content += &token.content;
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
                let content = self.content_all(&token.content);
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
        Ok(Rc::new(nodes::NodeIdentifer {
            content: token.content,
        }))
    }

    pub fn parse_string(&mut self) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        let token = self.base_parser.expect(TokenType::String)?;
        Ok(Rc::new(nodes::NodeString {
            content: token.content,
        }))
    }

    pub fn parse_number(&mut self) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        // check if it's an unary
        let unary = self
            .base_parser
            .expect_m(vec![TokenType::Addition, TokenType::Subtraction]);
        let token = self.base_parser.expect(TokenType::Number)?;
        let content = (unary.map_or("".to_owned(), |t| t.content) + &token.content)
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

        let number = self.parse_number();
        if number.is_ok() {
            return number;
        }

        let token = self
            .base_parser
            .chain_reader
            .get_current()
            .ok_or_else(|| BasicError::new("Unexpected end of input".to_owned()))?;
        Err(LinePointingError::new(
            &format!(
                "Unexpected ({:?}), expected ({})",
                token.r#type.blue(),
                string_utils::join_vec(
                    vec![
                        TokenType::KeyNumber,
                        TokenType::KeyString,
                        TokenType::Identifier,
                        TokenType::String,
                        TokenType::Number
                    ],
                    ", "
                )
                .blue()
            ),
            &self.base_parser.get_code(),
            token.start,
            token.length,
        ))
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
        let mut left = self.parse_binary_mul_div()?;
        while let Ok(operator) = self.base_parser.expect_m(vec![
            TokenType::Addition,
            TokenType::Subtraction,
            TokenType::DoubleSubtraction,
            TokenType::Modulo,
        ]) {
            let right = self.parse_binary_mul_div()?;
            left = Rc::new(nodes::NodeBinaryOperator {
                operator: operator.r#type,
                left,
                right,
            });
        }

        Ok(left)
    }

    pub fn parse_binary_mul_div(
        &mut self,
    ) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        let mut left = self.parse_binary_pow_log()?;
        while let Ok(operator) = self
            .base_parser
            .expect_m(vec![TokenType::Multiplication, TokenType::Division])
        {
            let right = self.parse_binary_pow_log()?;
            left = Rc::new(nodes::NodeBinaryOperator {
                operator: operator.r#type,
                left,
                right,
            });
        }

        Ok(left)
    }

    pub fn parse_binary_pow_log(
        &mut self,
    ) -> Result<Rc<dyn nodes::ExecutableNode>, Box<dyn Error>> {
        let mut left = self.parse_binary_parenthese()?;
        while let Ok(operator) = self
            .base_parser
            .expect_m(vec![TokenType::Power, TokenType::Log])
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
        if self.base_parser.expect(TokenType::ParentL).is_ok() {
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
        let token = self.base_parser.expect_m(vec![
            TokenType::QuestionMark,
            TokenType::QuestionMarkGreaterThan,
        ]);

        if let Ok(token) = token {
            let left = self.parse_ternary()?;
            let right = if token.r#type == TokenType::QuestionMark {
                self.base_parser.expect(TokenType::Semicolon)?;
                self.parse_ternary()?
            } else {
                // Skip and place ""
                Rc::new(NodeString {
                    content: String::new(),
                })
            };

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
