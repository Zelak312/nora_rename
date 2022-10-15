use crate::errors::{BasicError, Error};

use super::{number::NNumber, traits::To};

#[derive(PartialEq)]
pub struct NString {
    pub inner_value: String,
}

impl To<NNumber> for NString {
    fn to(&self) -> Result<NNumber, Box<dyn Error>> {
        let result = self
            .inner_value
            .parse::<f64>()
            .map_err(|_| BasicError::new("dwjdi".to_owned()))?;

        Ok(NNumber {
            inner_value: result,
        })
    }
}
