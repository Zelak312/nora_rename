use std::{collections::HashMap, rc::Rc};

use crate::errors::BasicError;
use crate::library::types::boolean::NBoolean;
use crate::library::types::number::NNumber;
use crate::library::types::string::NString;
use crate::utils::equal_utils;
use crate::{errors::Error, library::object_type::ObjectType, tokenizer::token::TokenType};

use super::nodes;

pub struct Interpreter {
    scope: HashMap<String, String>,
    count: i32,
    cap_count: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            scope: HashMap::new(),
            count: 0,
            cap_count: 0,
        }
    }

    fn insert_special_vars(&mut self) {
        self.scope
            .insert(String::from("#count"), self.count.to_string());
        self.scope
            .insert(String::from("#cap_count"), self.cap_count.to_string());
    }

    fn insert_captures(&mut self, captures: &HashMap<String, &str>) {
        for (key, val) in captures {
            if key.parse::<i8>().is_ok() {
                self.scope.insert(String::from("#") + key, val.to_string());
            } else {
                self.scope.insert(key.to_owned(), val.to_string());
            }
        }
    }

    pub fn mutate_scope(&mut self, key: String, val: String) -> Result<(), Box<dyn Error>> {
        if key.starts_with("#") {
            return Err(BasicError::new(format!(
                "Cannot mutate special variable: {}",
                key
            )));
        }

        self.scope.insert(key, val);
        Ok(())
    }

    pub fn execute(
        &mut self,
        captures: &HashMap<String, &str>,
        node: Rc<dyn nodes::ExecutableNode>,
    ) -> Result<ObjectType, Box<dyn Error>> {
        self.cap_count = captures.len();
        self.insert_captures(captures);
        self.insert_special_vars();
        let res = node.execute(self);
        self.count += 1;
        res
    }
}

impl nodes::ExecutableNode for nodes::NodeBinaryOperator {
    fn execute(&self, interpreter: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        let left_o = self.left.execute(interpreter)?;
        match left_o {
            ObjectType::NNumber(n) => {
                let rigth = self.right.execute(interpreter)?.into_number()?;
                let inner_value = match self.operator {
                    TokenType::Addition => n.inner_value + rigth.inner_value,
                    TokenType::Subtraction => n.inner_value - rigth.inner_value,
                    TokenType::DoubleSubtraction => n.inner_value + rigth.inner_value,
                    TokenType::Multiplication => n.inner_value * rigth.inner_value,
                    TokenType::Division => n.inner_value / rigth.inner_value,
                    TokenType::Power => n.inner_value.powf(rigth.inner_value),
                    TokenType::Log => n.inner_value.log(rigth.inner_value),
                    TokenType::Modulo => n.inner_value % rigth.inner_value,
                    _ => panic!("Operation not found for number"),
                };

                Ok(ObjectType::NNumber(NNumber { inner_value }))
            }
            ObjectType::NString(n) => {
                let rigth = self.right.execute(interpreter)?.into_string()?;
                let inner_value = match self.operator {
                    TokenType::Addition => n.inner_value + &rigth.inner_value,
                    TokenType::Subtraction => n.sub(&rigth).inner_value,
                    TokenType::DoubleSubtraction => n.sub_multiple(&rigth).inner_value,
                    _ => panic!("Operation not found for string"),
                };

                Ok(ObjectType::NString(NString { inner_value }))
            }
            _ => panic!("Cannot do binary operation on this type"), // TODO: better error
        }
    }
}

impl nodes::ExecutableNode for nodes::NodeBlock {
    fn execute(&self, interpreter: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        let mut inner_value = self
            .content
            .execute(interpreter)?
            .into_string()?
            .inner_value;
        if let Some(node) = &self.next {
            inner_value += &node.execute(interpreter)?.into_string()?.inner_value;
        }

        Ok(ObjectType::NString(NString { inner_value }))
    }
}

impl nodes::ExecutableNode for nodes::NodeFor {
    fn execute(&self, interpreter: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        let identifier = self.identifer.execute(interpreter)?.into_string()?;
        let from = self.from.execute(interpreter)?.into_number()?;
        let to = self.to.execute(interpreter)?.into_number()?;
        let mut inner_value = String::new();

        for i in (from.inner_value as i32)..(to.inner_value as i32) {
            interpreter.mutate_scope(identifier.inner_value.clone(), i.to_string())?;
            inner_value += &self
                .content
                .execute(interpreter)?
                .into_string()?
                .inner_value;
        }

        Ok(ObjectType::NString(NString { inner_value }))
    }
}

impl nodes::ExecutableNode for nodes::NodeContent {
    fn execute(&self, interpreter: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        let mut inner_value = String::from(&self.content);
        if let Some(node) = &self.next {
            inner_value += &node.execute(interpreter)?.into_string()?.inner_value;
        }

        Ok(ObjectType::NString(NString { inner_value }))
    }
}

impl nodes::ExecutableNode for nodes::NodeIdentifer {
    fn execute(&self, i: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        // TODO: should make this a linePointingError
        // Need to implement nodes start and length for this to happen
        if self.use_for_name {
            return Ok(ObjectType::NString(NString {
                inner_value: self.content.clone(),
            }));
        }

        let mut capture = i.scope.get(&self.content);
        if capture.is_none() {
            let removed_hashtag = self.content.trim_start_matches("#");
            if self.content.starts_with("#") && i.scope.contains_key(removed_hashtag) {
                capture = i
                    .scope
                    .get(&("#".to_owned() + i.scope.get(removed_hashtag).unwrap()));
            }
        }

        Ok(ObjectType::NString(NString {
            inner_value: capture
                .ok_or_else(|| {
                    BasicError::new(format!("Couldn't find variable: {}", &self.content))
                })?
                .to_owned(),
        }))
    }
}

impl nodes::ExecutableNode for nodes::NodeString {
    fn execute(&self, _: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        Ok(ObjectType::NString(NString {
            inner_value: self.content.clone(),
        }))
    }
}

impl nodes::ExecutableNode for nodes::NodeNumber {
    fn execute(&self, _: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        Ok(ObjectType::NNumber(NNumber {
            inner_value: self.content,
        }))
    }
}

impl nodes::ExecutableNode for nodes::NodeCondition {
    fn execute(&self, i: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        let inner_value = match self.left.execute(i)? {
            ObjectType::NBoolean(n) => {
                equal_utils::partial_eq(&self.operator, n, self.right.execute(i)?.into_boolean()?)?
            }
            ObjectType::NString(n) => {
                equal_utils::partial_eq(&self.operator, n, self.right.execute(i)?.into_string()?)?
            }
            ObjectType::NNumber(n) => {
                equal_utils::partial_ord(&self.operator, n, self.right.execute(i)?.into_number()?)?
            }
        };

        Ok(ObjectType::NBoolean(NBoolean { inner_value }))
    }
}
impl nodes::ExecutableNode for nodes::NodeTernary {
    fn execute(&self, i: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        let cond = self.condition.execute(i)?.into_boolean()?;
        if cond.inner_value {
            return self.left.execute(i);
        }

        self.right.execute(i)
    }
}

impl nodes::ExecutableNode for nodes::NodeKeyword {
    fn execute(&self, i: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        Ok(match self.keyword {
            TokenType::KeyNumber => {
                let mut num = self.content.execute(i)?.into_number()?;
                if !self.options.is_empty() {
                    let pow_val =
                        10f64.powf(self.options[0].execute(i)?.into_number()?.inner_value);
                    num.inner_value = (num.inner_value * pow_val).round() / pow_val;
                }

                ObjectType::NNumber(num)
            }
            TokenType::KeyString => ObjectType::NString(self.content.execute(i)?.into_string()?),
            _ => panic!("djijdiw"),
        })
    }
}
