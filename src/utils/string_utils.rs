use std::fmt::Debug;

pub fn is_identifer(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '#'
}

pub fn join_vec<T>(v: Vec<T>, sep: &str) -> String
where
    T: Debug,
{
    if v.is_empty() {
        return String::new();
    }

    let mut s = format!("{:?}", v[0]);
    for i in 1..v.len() {
        s += &format!("{}{:?}", sep, v[i]);
    }

    s
}
