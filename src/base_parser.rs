use crate::{
    chain_reader::ChainReader,
    errors::{BasicError, Error},
    token::{Token, Type},
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
        self.chain_reader
            .eat()
            .ok_or(BasicError::new(String::from("Missing")))
    }

    pub fn expect(&mut self, r#type: Type) -> Result<Token, Box<dyn Error>> {
        let token = self
            .chain_reader
            .get_current()
            .ok_or(BasicError::new(String::from("Missing")))?;

        if r#type != token.r#type {
            return Err(BasicError::new(String::from("the fuck")));
        }

        self.chain_reader.advance();
        Ok(token)
    }

    pub fn expect_m(&mut self, types: Vec<Type>) -> Result<Token, Box<dyn Error>> {
        for r#type in types {
            let token = self.expect(r#type);
            if token.is_ok() {
                return token;
            }
        }

        Err(BasicError::new(String::from("the fuck")))
    }
}
