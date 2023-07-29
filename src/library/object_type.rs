use crate::errors::{BasicError, Error};

use super::types::{boolean::NBoolean, number::NNumber, string::NString};

pub enum ObjectType {
    NBoolean(NBoolean),
    NString(NString),
    NNumber(NNumber),
}

pub type IntoConv<T> = Result<T, Box<dyn Error>>;
impl ObjectType {
    pub fn into_string(self) -> IntoConv<NString> {
        match self {
            ObjectType::NBoolean(n) => n.try_into(),
            ObjectType::NString(n) => Ok(n),
            ObjectType::NNumber(n) => n.try_into(),
        }
    }

    pub fn into_number(self) -> IntoConv<NNumber> {
        match self {
            ObjectType::NBoolean(n) => n.try_into(),
            ObjectType::NString(n) => n.try_into(),
            ObjectType::NNumber(n) => Ok(n),
        }
    }

    pub fn into_boolean(self) -> IntoConv<NBoolean> {
        match self {
            ObjectType::NBoolean(n) => Ok(n),
            ObjectType::NNumber(n) => n.try_into(),
            _ => Err(BasicError::new(
                "conversion to boolean not supported".to_owned(),
            )),
        }
    }
}
