use crate::errors::Error;

pub trait To<T>: Sized {
    fn to(&self) -> Result<T, Box<dyn Error>>;
}
