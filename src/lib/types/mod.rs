pub mod boolean;
pub mod number;
pub mod string;
use super::types::{boolean::NBoolean, number::NNumber, string::NString};

pub enum ObjectType {
    NBoolean(NBoolean),
    NString(NString),
    NNumber(NNumber),
}
