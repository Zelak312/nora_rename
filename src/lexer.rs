use super::string_reader::StringReader;
use super::token::{Token, Type};
use super::utils;
use std::fmt::{Debug, Formatter, Result};

pub struct Lexer {
    string_reader: StringReader,
}

impl Lexer {
    pub fn handle_number(&mut self, c: char) -> Token {
        let mut raw = c.to_string();
        self.string_reader.advance();
        while !self.string_reader.reached_end() {
            let current = self.string_reader.get_current();
            if !current.is_numeric() {
                break;
            }

            raw += &current.to_string();
            self.string_reader.advance();
        }

        Token::new(raw, Type::Number)
    }

    pub fn handle_identifer(&mut self, c: char) -> Token {
        let mut raw = c.to_string();
        self.string_reader.advance();
        while !self.string_reader.reached_end() {
            let current = self.string_reader.get_current();
            if !utils::is_identifer(current) {
                break;
            }

            raw += &current.to_string();
            self.string_reader.advance();
        }

        Token::new(raw, Type::Identifier)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while !self.string_reader.reached_end() {
            let mut token = None;
            let current = self.string_reader.get_current();
            if current.is_numeric() {
                token = Some(self.handle_number(current));
            } else if utils::is_identifer(current) {
                token = Some(self.handle_identifer(current));
            }

            if let Some(found_token) = token {
                tokens.push(found_token);
            }
        }

        tokens
    }
}

impl Debug for Lexer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Lexer")
            .field("string_reader", &self.string_reader)
            .finish()
    }
}
