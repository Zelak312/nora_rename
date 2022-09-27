pub fn replace(regexInput: String, output: &String) -> String {
    let mut out = String::new();
    let chars = output.chars();
    for c in chars {
        if c == '\\' {
            let next = chars.next();
        } else if c == '$' {
        }
    }

    out
}
