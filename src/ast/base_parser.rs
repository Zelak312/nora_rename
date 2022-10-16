use crate::{
    errors::{Error, UnexpectedEndOfFile, UnexpectedError},
    tokenizer::token::{Token, TokenType},
    utils::chain_reader::ChainReader,
};

pub struct BaseParser {
    pub chain_reader: ChainReader<Token>,
}

impl BaseParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            chain_reader: ChainReader::new(tokens),
        }
    }

    pub fn any(&mut self) -> Result<Token, Box<dyn Error>> {
        self.chain_reader.eat().ok_or(UnexpectedEndOfFile::new())
    }

    pub fn expect(&mut self, r#type: TokenType) -> Result<Token, Box<dyn Error>> {
        let token = self
            .chain_reader
            .get_current()
            .ok_or(UnexpectedEndOfFile::new())?;

        if r#type != token.r#type {
            return Err(UnexpectedError::new(token.r#type, r#type.clone()));
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

        let token = self.chain_reader.get_current().expect("diend");
        Err(UnexpectedError::new_m(token.r#type, types))
    }
}
