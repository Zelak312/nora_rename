use crate::{
    errors::{BasicError, Error},
    library::object_type::IntoConv,
};

use super::{boolean::NBoolean, string::NString};

#[derive(PartialEq, PartialOrd)]
pub struct NNumber {
    pub inner_value: f64,
}

impl TryInto<NString> for NNumber {
    type Error = Box<dyn Error>;
    fn try_into(self) -> IntoConv<NString> {
        Ok(NString {
            inner_value: self.inner_value.to_string(),
        })
    }
}

impl TryInto<NBoolean> for NNumber {
    type Error = Box<dyn Error>;
    fn try_into(self) -> IntoConv<NBoolean> {
        let result = if self.inner_value == 1.0 {
            Ok(true)
        } else if self.inner_value == 0.0 {
            Ok(false)
        } else {
            Err(BasicError::new(
                "Couldn't convert number to boolean".to_owned(),
            ))
        }?;

        Ok(NBoolean {
            inner_value: result,
        })
    }
}
