use crate::{
    chain_reader::ChainReader,
    node::Node,
    nodes::{
        node_binary_operator::NodeBinaryOperator, node_identifier::NodeIdentifer,
        node_number::NodeNumber,
    },
    token::{Token, Type},
};

pub struct Parser {
    chain_reader: ChainReader<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            chain_reader: ChainReader::new(tokens),
        }
    }

    pub fn content_all(&mut self, start: &str) -> String {
        let mut content = String::from(start);
        while let Some(token) = self.chain_reader.get_current() {
            if token.r#type == Type::BlockStart {
                break;
            }

            content += &token.raw;
            self.chain_reader.advance();
        }

        content
    }

    pub fn parse(&mut self) -> Box<dyn Node> {
        let token = self.chain_reader.eat();
        if token.is_none() {
            panic!("No tokens to parse");
        }

        self.parse_content(token.unwrap())
    }

    pub fn parse_identifier(&mut self, token: Token) -> Box<dyn Node> {
        Box::new(NodeIdentifer::new(token.raw))
    }

    pub fn parse_number(&mut self, token: Token) -> Box<dyn Node> {
        Box::new(NodeNumber::new(token.raw))
    }

    pub fn parse_basic_type(&mut self, token: Token) -> Box<dyn Node> {
        match token.r#type {
            Type::Identifier => self.parse_identifier(token),
            Type::Number => self.parse_number(token),
            _ => {
                panic!("sheet")
            }
        }
    }

    pub fn parse_binary_operation(&mut self, token: Token) -> Box<dyn Node> {
        let left = self.parse_basic_type(token);
        let token = self.chain_reader.eat().expect("dwdueu");
        match token.r#type {
            Type::Plus | Type::Minus | Type::Mul | Type::Div => {
                let n_token = self.chain_reader.eat().expect("some");
                let rigth = self.parse_basic_type(n_token);
                let binary = NodeBinaryOperator::new(token.r#type, left, rigth);
                Box::new(binary)
            }
            _ => left,
        }
    }

    pub fn parse_block(&mut self, token: Token) -> Box<dyn Node> {
        // self.chain_reader.eat(); // need to check shit
        let token = self.chain_reader.eat();
        let node = self.parse_binary_operation(token.expect("msss"));
        self.chain_reader.eat();
        node
    }

    pub fn parse_content(&mut self, token: Token) -> Box<dyn Node> {
        match token.r#type {
            Type::BlockStart => self.parse_block(token),
            _ => {
                let content = self.content_all(&token.raw);
                let identifer = NodeIdentifer::new(content);
                if let Some(token) = self.chain_reader.eat() {
                    return self.parse_content(token);
                }

                Box::new(identifer)
            }
        }
    }
}
