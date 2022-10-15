use super::{number::NNumber, string::NString, traits::To};

#[derive(PartialEq)]
pub struct NBoolean {
    inner_value: bool,
}

impl NBoolean {
    pub fn new(inner_value: bool) -> Self {
        Self { inner_value }
    }
}

impl To<NNumber> for NBoolean {
    fn to(&self) -> Result<NNumber, Box<dyn crate::errors::Error>> {
        let float = match self.inner_value {
            true => 1.0,
            false => 0.0,
        };

        Ok(NNumber::new(float))
    }
}

impl To<NString> for NBoolean {
    fn to(&self) -> Result<NString, Box<dyn crate::errors::Error>> {
        Ok(NString::new(self.inner_value.to_string()))
    }
}
