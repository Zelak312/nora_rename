pub fn is_identifer(c: char, is_first: bool) -> bool {
    if is_first {
        return c == '&';
    }

    c.is_alphanumeric() || c == '_'
}
