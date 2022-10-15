use super::{number::NNumber, string::NString, traits::To};

#[derive(PartialEq)]
pub struct NBoolean {
    pub inner_value: bool,
}

impl To<NNumber> for NBoolean {
    fn to(&self) -> Result<NNumber, Box<dyn crate::errors::Error>> {
        let inner_value = match self.inner_value {
            true => 1.0,
            false => 0.0,
        };

        Ok(NNumber { inner_value })
    }
}

impl To<NString> for NBoolean {
    fn to(&self) -> Result<NString, Box<dyn crate::errors::Error>> {
        Ok(NString {
            inner_value: self.inner_value.to_string(),
        })
    }
}
