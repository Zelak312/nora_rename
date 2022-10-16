use std::fmt::{Debug, Formatter, Result};

use crate::utils::{chain_reader::ChainReader, string_utils};

use super::token::{Token, TokenType};

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
            '[' => Some(TokenType::BlockStart),
            ']' => Some(TokenType::BlockEnd),
            '+' => Some(TokenType::Addition),
            '-' => Some(TokenType::Subtraction),
            '/' => Some(TokenType::Division),
            '*' => Some(TokenType::Multiplication),
            '(' => Some(TokenType::ParentL),
            ')' => Some(TokenType::ParentR),
            ':' => Some(TokenType::Semicolon),
            '?' => Some(TokenType::QuestionMark),
            '=' => Some(TokenType::EqualSign),
            '<' => Some(TokenType::LessThanSign),
            '>' => Some(TokenType::GreaterThanSign),
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

    pub fn handle_double_special(&mut self, c: char, _type: TokenType) -> Option<Token> {
        let next_c_o = self.chain_reader.get_current();
        if next_c_o.is_none() {
            return None;
        }

        let next_c = next_c_o.unwrap();
        let type_o = match _type {
            TokenType::EqualSign => match next_c {
                '=' => Some(TokenType::DoubleEqualSign),
                _ => None,
            },
            TokenType::LessThanSign => match next_c {
                '=' => Some(TokenType::LessThanEqualSign),
                _ => None,
            },
            TokenType::GreaterThanSign => match next_c {
                '=' => Some(TokenType::GreaterThanEqualSign),
                _ => None,
            },
            TokenType::ExclamationMark => match next_c {
                '=' => Some(TokenType::NotEqualSign),
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

        Token::new(raw, TokenType::Number)
    }

    pub fn handle_identifer(&mut self, c: char) -> Token {
        let mut raw = c.to_string();
        self.chain_reader.advance();
        while let Some(current) = self.chain_reader.get_current() {
            if !string_utils::is_identifer(current) {
                break;
            }

            raw += &current.to_string();
            self.chain_reader.advance();
        }

        Token::new(raw, TokenType::Identifier)
    }

    pub fn handle_string(&mut self) -> Token {
        let mut raw = String::new();
        self.chain_reader.advance();
        while let Some(current) = self.chain_reader.get_current() {
            if current == '"' {
                self.chain_reader.advance();
                break;
            }

            raw += &current.to_string();
            self.chain_reader.advance();
        }

        Token::new(raw, TokenType::String)
    }

    pub fn handle_keyword(&mut self, s: &str) -> Option<Token> {
        let _type = match s {
            "number" => Some(TokenType::KeyNumber),
            "string" => Some(TokenType::KeyString),
            _ => None,
        };

        if _type.is_some() {
            return Some(Token::new(s.to_owned(), _type.unwrap()));
        }

        None
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        let mut raw = String::new();
        while let Some(current) = self.chain_reader.get_current() {
            let mut token_o = None;
            let mut unvariable = false;
            if let Some(found_token) = self.handle_special(current) {
                if found_token.r#type == TokenType::BlockStart {
                    self.in_block = true;
                } else if found_token.r#type == TokenType::BlockEnd {
                    self.in_block = false;
                }

                token_o = Some(found_token);
            } else if current.is_numeric() {
                token_o = Some(self.handle_number(current));
            } else if current == '"' {
                token_o = Some(self.handle_string());
            } else if string_utils::is_identifer(current) {
                token_o = Some(self.handle_identifer(current));
                if let Some(keyword) = self.handle_keyword(&token_o.as_ref().unwrap().raw) {
                    token_o = Some(keyword)
                }
            } else if !self.in_block {
                unvariable = true;
                raw += &current.to_string();
                self.chain_reader.advance();
            } else {
                // skip
                self.chain_reader.advance();
            }

            if !unvariable && raw != "" {
                let tmp_token = Token::new(raw.to_string(), TokenType::Unvariable);
                tokens.push(tmp_token);
                raw = String::new();
            }

            if let Some(token) = token_o {
                tokens.push(token);
            }
        }

        if raw != "" {
            let tmp_token = Token::new(raw.to_string(), TokenType::Unvariable);
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
