use crate::lib::object_type::IntoConv;

use super::{number::NNumber, string::NString};

#[derive(PartialEq)]
pub struct NBoolean {
    pub inner_value: bool,
}

impl Into<IntoConv<NNumber>> for NBoolean {
    fn into(self) -> IntoConv<NNumber> {
        let inner_value = match self.inner_value {
            true => 1.0,
            false => 0.0,
        };

        Ok(NNumber { inner_value })
    }
}

impl Into<IntoConv<NString>> for NBoolean {
    fn into(self) -> IntoConv<NString> {
        Ok(NString {
            inner_value: self.inner_value.to_string(),
        })
    }
}
