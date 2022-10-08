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
            '<' => Some(Type::BlockStart),
            '>' => Some(Type::BlockEnd),
            '+' => Some(Type::Plus),
            '-' => Some(Type::Minus),
            '/' => Some(Type::Div),
            '*' => Some(Type::Mul),
            _ => None,
        };

        if type_o.is_some() {
            self.chain_reader.advance();
            return Some(Token::new(c.to_string(), type_o.unwrap()));
        }

        None
    }

    pub fn handle_number(&mut self, c: char) -> Token {
        let mut raw = c.to_string();
        self.chain_reader.advance();
        while let Some(current) = self.chain_reader.get_current() {
            if !current.is_numeric() {
                break;
            }

            raw += &current.to_string();
            self.chain_reader.advance();
        }

        Token::new(raw, Type::Number)
    }

    pub fn handle_identifer(&mut self) -> Token {
        let mut raw = String::new();
        self.chain_reader.advance();
        while let Some(current) = self.chain_reader.get_current() {
            if !utils::is_identifer(current, false) {
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
            } else if utils::is_identifer(current, true) {
                token_o = Some(self.handle_identifer());
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
