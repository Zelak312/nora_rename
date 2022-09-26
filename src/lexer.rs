use super::token::{Token, Type};
use super::utils;
use std::fmt::{Debug, Formatter, Result};

pub struct Lexer {
    pos: usize,
    chars: Vec<char>,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        let chars = code.chars().collect::<Vec<char>>();

        Self { pos: 0, chars }
    }

    pub fn reached_end(&mut self) -> bool {
        self.pos >= self.chars.len()
    }

    pub fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn get_current(&mut self) -> char {
        self.chars[self.pos]
    }

    pub fn handle_number(&mut self, c: char) -> Token {
        let mut raw = c.to_string();
        self.advance();
        while !self.reached_end() {
            let current = self.get_current();
            if !current.is_numeric() {
                break;
            }

            raw += &current.to_string();
            self.advance();
        }

        Token::new(raw, Type::Number)
    }

    pub fn handle_identifer(&mut self, c: char) -> Token {
        let mut raw = c.to_string();
        self.advance();
        while !self.reached_end() {
            let current = self.get_current();
            if !utils::is_identifer(current) {
                break;
            }

            raw += &current.to_string();
            self.advance();
        }

        Token::new(raw, Type::Identifier)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while !self.reached_end() {
            let mut token = None;
            let current = self.get_current();
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
            .field("pos", &self.pos)
            .field("chars", &self.chars)
            .finish()
    }
}
