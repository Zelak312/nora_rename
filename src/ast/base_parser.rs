use owo_colors::OwoColorize;

use crate::{
    errors::{BasicError, Error, LinePointingError},
    tokenizer::token::{Token, TokenType},
    utils::{chain_reader::ChainReader, string_utils},
};

pub struct BaseParser {
    pub chain_reader: ChainReader<Token>,
    code: String,
}

impl BaseParser {
    pub fn new(tokens: Vec<Token>, code: String) -> Self {
        Self {
            chain_reader: ChainReader::new(tokens),
            code,
        }
    }

    pub fn get_code(&self) -> String {
        self.code.clone()
    }

    pub fn any(&mut self) -> Result<Token, Box<dyn Error>> {
        self.chain_reader
            .eat()
            .ok_or_else(|| BasicError::new("Unexpected end of input".to_owned()).into())
    }

    pub fn expect(&mut self, r#type: TokenType) -> Result<Token, Box<dyn Error>> {
        let token = self
            .chain_reader
            .get_current()
            .ok_or_else(|| BasicError::new("Unexpected end of input".to_owned()))?;

        if r#type != token.r#type {
            return Err(LinePointingError::new(
                &format!(
                    "Unexpected ({:?}), expected token ({:?})",
                    token.r#type.blue(),
                    r#type.blue()
                ),
                &self.code,
                token.start,
                token.length,
            ));
        }

        self.chain_reader.advance();
        Ok(token)
    }

    pub fn expect_m(&mut self, types: Vec<TokenType>) -> Result<Token, Box<dyn Error>> {
        for r#type in &types {
            let token = self.expect(r#type.clone());
            if token.is_ok() {
                return token;
            }
        }

        let token = self
            .chain_reader
            .get_current()
            .ok_or_else(|| BasicError::new("Unexpected end of input".to_owned()))?;

        Err(LinePointingError::new(
            &format!(
                "Unexpected ({:?}), expected ({})",
                token.r#type.blue(),
                string_utils::join_vec(types, ", ").blue()
            ),
            &self.code,
            token.start,
            token.length,
        ))
    }
}
