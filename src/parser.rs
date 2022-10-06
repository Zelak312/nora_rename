use crate::{
    ast::{NodeBinaryOperator, NodeBlock, NodeContent, NodeIdentifer, NodeNumber},
    base_parser::BaseParser,
    errors::Error,
    node::Node,
    token::{Token, Type},
};

pub struct Parser {
    base_parser: BaseParser,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            base_parser: BaseParser::new(tokens),
        }
    }

    pub fn content_all(&mut self, start: &str) -> String {
        let mut content = String::from(start);
        while let Some(token) = self.base_parser.chain_reader.get_current() {
            if token.r#type == Type::BlockStart {
                break;
            }

            content += &token.raw;
            self.base_parser.chain_reader.advance();
        }

        content
    }

    pub fn parse(&mut self) -> Result<Box<Node>, Box<dyn Error>> {
        self.parse_content()
    }

    pub fn parse_identifier(&mut self) -> Result<Box<Node>, Box<dyn Error>> {
        let token = self.base_parser.expect(Type::Identifier)?;
        Ok(Box::new(Node::NodeIdentifer(NodeIdentifer::new(token.raw))))
    }

    pub fn parse_number(&mut self) -> Result<Box<Node>, Box<dyn Error>> {
        let token = self.base_parser.expect(Type::Number)?;
        Ok(Box::new(Node::NodeNumber(NodeNumber::new(token.raw))))
    }

    pub fn parse_basic_type(&mut self) -> Result<Box<Node>, Box<dyn Error>> {
        let identifer = self.parse_identifier();
        if identifer.is_ok() {
            return identifer;
        }

        self.parse_number()
    }

    pub fn parse_binary_operation(&mut self) -> Result<Box<Node>, Box<dyn Error>> {
        let left = self.parse_basic_type()?;
        let operator =
            self.base_parser
                .expect_m(vec![Type::Plus, Type::Minus, Type::Mul, Type::Div])?;
        let rigth = self.parse_basic_type()?;
        let binary = NodeBinaryOperator::new(operator.r#type, left, rigth);
        Ok(Box::new(Node::NodeBinaryOperator(binary)))
    }

    pub fn parse_inner_block(&mut self) -> Result<Box<Node>, Box<dyn Error>> {
        let node = self.parse_binary_operation()?;
        self.base_parser.expect(Type::BlockEnd)?;
        Ok(node)
    }

    pub fn parse_content(&mut self) -> Result<Box<Node>, Box<dyn Error>> {
        let token = self.base_parser.any()?;

        match token.r#type {
            Type::BlockStart => {
                let block = self.parse_inner_block()?;
                let mut block_node = NodeBlock::new(block);
                if let Ok(node) = self.parse_content() {
                    block_node.set_right(node);
                }

                Ok(Box::new(Node::NodeBlock(block_node)))
            }
            _ => {
                let content = self.content_all(&token.raw);
                let mut content_node = NodeContent::new(content);
                if let Ok(node) = self.parse_content() {
                    content_node.set_next(node);
                }

                Ok(Box::new(Node::NodeContent(content_node)))
            }
        }
    }
}
