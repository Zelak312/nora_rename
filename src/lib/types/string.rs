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
        let mut new_string = self.inner_value.clone();
        new_string.retain(|c| !other.inner_value.contains(c));
        NString {
            inner_value: new_string,
        }
    }
}

impl TryInto<NNumber> for NString {
    type Error = Box<dyn Error>;
    fn try_into(self) -> IntoConv<NNumber> {
        let result = self
            .inner_value
            .parse::<f64>()
            .map_err(|_| BasicError::new("dwjdi".to_owned()))?;

        Ok(NNumber {
            inner_value: result,
        })
    }
}
