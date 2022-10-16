use crate::{
    errors::{BasicError, Error},
    tokenizer::token::TokenType,
};

pub fn partial_eq<T>(token: &TokenType, l: T, r: T) -> Result<bool, Box<dyn Error>>
where
    T: PartialEq,
{
    match token {
        TokenType::DoubleEqualSign => Ok(l == r),
        TokenType::NotEqualSign => Ok(l != r),
        _ => Err(BasicError::new("not a good equation".to_owned())),
    }
}

pub fn partial_ord<T>(token: &TokenType, l: T, r: T) -> Result<bool, Box<dyn Error>>
where
    T: PartialOrd,
{
    match token {
        TokenType::DoubleEqualSign => Ok(l == r),
        TokenType::NotEqualSign => Ok(l != r),
        TokenType::GreaterThanSign => Ok(l > r),
        TokenType::GreaterThanEqualSign => Ok(l >= r),
        TokenType::LessThanSign => Ok(l < r),
        TokenType::LessThanEqualSign => Ok(l <= r),
        _ => Err(BasicError::new("not a good equation".to_owned())),
    }
}
