use super::number::NNumber;
use crate::{
    errors::{BasicError, Error},
    lib::object_type::IntoConv,
};

#[derive(PartialEq, Eq)]
pub struct NString {
    pub inner_value: String,
}

impl NString {
    pub fn sub(&self, other: &NString) -> NString {
        NString {
            inner_value: self.inner_value.replacen(&other.inner_value, "", 1),
        }
    }

    pub fn sub_multiple(&self, other: &NString) -> NString {
        NString {
            inner_value: self.inner_value.replace(&other.inner_value, ""),
        }
    }
}

impl TryInto<NNumber> for NString {
    type Error = Box<dyn Error>;
    fn try_into(self) -> IntoConv<NNumber> {
        let result = self
            .inner_value
            .parse::<f64>()
            .map_err(|_| BasicError::new("couldn't convert string to number".to_owned()))?;

        Ok(NNumber {
            inner_value: result,
        })
    }
}
