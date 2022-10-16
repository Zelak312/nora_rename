use crate::{errors::Error, lib::object_type::IntoConv};

use super::{number::NNumber, string::NString};

#[derive(PartialEq)]
pub struct NBoolean {
    pub inner_value: bool,
}

impl TryInto<NNumber> for NBoolean {
    type Error = Box<dyn Error>;
    fn try_into(self) -> IntoConv<NNumber> {
        let inner_value = match self.inner_value {
            true => 1.0,
            false => 0.0,
        };

        Ok(NNumber { inner_value })
    }
}

impl TryInto<NString> for NBoolean {
    type Error = Box<dyn Error>;
    fn try_into(self) -> IntoConv<NString> {
        Ok(NString {
            inner_value: self.inner_value.to_string(),
        })
    }
}
