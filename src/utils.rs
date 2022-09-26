pub fn is_identifer(c: char) -> bool {
    c.is_alphabetic() || c == '$'
}
