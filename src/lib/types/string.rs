use super::number::NNumber;
use crate::{errors::BasicError, lib::object_type::IntoConv};

#[derive(PartialEq)]
pub struct NString {
    pub inner_value: String,
}

impl Into<IntoConv<NNumber>> for NString {
    fn into(self) -> IntoConv<NNumber> {
        let result = self
            .inner_value
            .parse::<f64>()
            .map_err(|_| BasicError::new("dwjdi".to_owned()))?;

        Ok(NNumber {
            inner_value: result,
        })
    }
}
