use super::chain_reader::ChainReader;
use super::token::{Token, Type};
use super::utils;
use std::fmt::{Debug, Formatter, Result};

pub struct Lexer {
    chain_reader: ChainReader<char>,
    in_block: bool,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        let chars = code.chars().collect::<Vec<char>>();
        Self {
            chain_reader: ChainReader::new(chars),
            in_block: false,
        }
    }

    pub fn handle_special(&mut self, c: char) -> Option<Token> {
        let type_o = match c {
            '[' => Some(Type::BlockStart),
            ']' => Some(Type::BlockEnd),
            '+' => Some(Type::Addition),
            '-' => Some(Type::Subtraction),
            '/' => Some(Type::Division),
            '*' => Some(Type::Multiplication),
            '(' => Some(Type::ParentL),
            ')' => Some(Type::ParentR),
            ':' => Some(Type::Semicolon),
            '?' => Some(Type::QuestionMark),
            '=' => Some(Type::EqualSign),
            '<' => Some(Type::LessThanSign),
            '>' => Some(Type::GreaterThanSign),
            _ => None,
        };

        if let Some(_type) = type_o {
            self.chain_reader.advance();
            if let Some(token) = self.handle_double_special(c, _type.clone()) {
                return Some(token);
            }

            return Some(Token::new(c.to_string(), _type));
        }

        None
    }

    pub fn handle_double_special(&mut self, c: char, _type: Type) -> Option<Token> {
        let next_c_o = self.chain_reader.get_current();
        if next_c_o.is_none() {
            return None;
        }

        let next_c = next_c_o.unwrap();
        let type_o = match _type {
            Type::EqualSign => match next_c {
                '=' => Some(Type::DoubleEqualSign),
                _ => None,
            },
            Type::LessThanSign => match next_c {
                '=' => Some(Type::LessThanEqualSign),
                _ => None,
            },
            Type::GreaterThanSign => match next_c {
                '=' => Some(Type::GreaterThanEqualSign),
                _ => None,
            },
            _ => None,
        };

        if let Some(_type_d) = type_o {
            self.chain_reader.advance();
            return Some(Token::new(c.to_string() + &next_c.to_string(), _type_d));
        }

        None
    }

    pub fn handle_number(&mut self, c: char) -> Token {
        let mut raw = c.to_string();
        self.chain_reader.advance();
        while let Some(current) = self.chain_reader.get_current() {
            if current == '_' {
                self.chain_reader.advance();
                continue;
            }

            if !current.is_numeric() && current != '.' {
                break;
            }

            raw += &current.to_string();
            self.chain_reader.advance();
        }

        Token::new(raw, Type::Number)
    }

    pub fn handle_identifer(&mut self, c: char) -> Token {
        let mut raw = c.to_string();
        self.chain_reader.advance();
        while let Some(current) = self.chain_reader.get_current() {
            if !utils::is_identifer(current) {
                break;
            }

            raw += &current.to_string();
            self.chain_reader.advance();
        }

        Token::new(raw, Type::Identifier)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        let mut raw = String::new();
        while let Some(current) = self.chain_reader.get_current() {
            let mut token_o = None;
            let mut unvariable = false;
            if let Some(found_token) = self.handle_special(current) {
                if found_token.r#type == Type::BlockStart {
                    self.in_block = true;
                } else if found_token.r#type == Type::BlockEnd {
                    self.in_block = false;
                }

                token_o = Some(found_token);
            } else if current.is_numeric() {
                token_o = Some(self.handle_number(current));
            } else if utils::is_identifer(current) {
                token_o = Some(self.handle_identifer(current));
            } else if !self.in_block {
                unvariable = true;
                raw += &current.to_string();
                self.chain_reader.advance();
            } else {
                // skip
                self.chain_reader.advance();
            }

            if !unvariable && raw != "" {
                let tmp_token = Token::new(raw.to_string(), Type::Unvariable);
                tokens.push(tmp_token);
                raw = String::new();
            }

            if let Some(token) = token_o {
                tokens.push(token);
            }
        }

        if raw != "" {
            let tmp_token = Token::new(raw.to_string(), Type::Unvariable);
            tokens.push(tmp_token);
        }

        tokens
    }
}

impl Debug for Lexer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Lexer")
            .field("string_reader", &self.chain_reader)
            .finish()
    }
}
