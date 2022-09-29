use crate::{chain_reader::ChainReader, node::Node, token::Token};

pub struct Parser {
    chain_reader: ChainReader<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            chain_reader: ChainReader::new(tokens),
        }
    }

    pub fn parse(&mut self) -> Box<Node> {}
}
