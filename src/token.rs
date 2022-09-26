pub enum Type {
    Identifier,
    Number,
}

pub struct Token {
    pub raw: String,
    pub r#type: Type,
}

impl Token {
    pub fn new(raw: String, r#type: Type) -> Self {
        Self { raw, r#type }
    }
}
