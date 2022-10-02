use crate::{
    chain_reader::ChainReader,
    errors::{BasicError, Error},
    token::{Token, Type},
};

pub struct BaseParser {
    chain_reader: ChainReader<Token>,
}

impl BaseParser {
    pub fn expect(&mut self, r#type: Type) -> Result<Token, Box<dyn Error>> {
        let token = self
            .chain_reader
            .eat()
            .ok_or(BasicError::new(String::from("Missing")))?;

        if r#type != token.r#type {
            return Err(BasicError::new(String::from("the fuck")));
        }

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
