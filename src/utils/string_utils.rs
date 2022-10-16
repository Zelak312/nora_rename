pub fn is_identifer(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '#'
}
