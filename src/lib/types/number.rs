use crate::{errors::BasicError, lib::object_type::IntoConv};

use super::{boolean::NBoolean, string::NString};

#[derive(PartialEq, PartialOrd)]
pub struct NNumber {
    pub inner_value: f64,
}

impl Into<IntoConv<NString>> for NNumber {
    fn into(self) -> IntoConv<NString> {
        Ok(NString {
            inner_value: self.inner_value.to_string(),
        })
    }
}

impl Into<IntoConv<NBoolean>> for NNumber {
    fn into(self) -> IntoConv<NBoolean> {
        let result = if self.inner_value == 1.0 {
            Ok(true)
        } else if self.inner_value == 0.0 {
            Ok(false)
        } else {
            Err(BasicError::new("834572bhjer".to_owned()))
        }?;

        Ok(NBoolean {
            inner_value: result,
        })
    }
}
