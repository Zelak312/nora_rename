use crate::errors::BasicError;

use super::{boolean::NBoolean, string::NString, traits::To};

#[derive(PartialEq, PartialOrd)]
pub struct NNumber {
    inner_value: f64,
}

impl NNumber {
    pub fn new(inner_value: f64) -> Self {
        Self { inner_value }
    }
}

impl To<NString> for NNumber {
    fn to(&self) -> Result<NString, Box<dyn crate::errors::Error>> {
        Ok(NString::new(self.inner_value.to_string()))
    }
}

impl To<NBoolean> for NNumber {
    fn to(&self) -> Result<NBoolean, Box<dyn crate::errors::Error>> {
        let result = if self.inner_value == 1.0 {
            Ok(true)
        } else if self.inner_value == 0.0 {
            Ok(false)
        } else {
            Err(BasicError::new("834572bhjer".to_owned()))
        }?;

        Ok(NBoolean::new(result))
    }
}
