pub fn add_prefix(text: &str, prefix: &str) -> String {
    format!("{}{}", prefix, text)
}

pub fn add_suffix(text: &str, suffix: &str) -> String {
    format!("{}{}", text, suffix)
}



pub fn surround(text: &str, before: &str, after: &str) -> String {
    format!("{}{}{}", before, text, after)
}



pub fn insert_at(text: &str, pos: usize, insert: &str) -> String {
    // 1. Get the part before the position
    let head = &text[..pos];
    // 2. Get the part after the position
    let tail = &text[pos..];
    
    // 3. Combine them with the new insert
    format!("{}{}{}", head, insert, tail)
}
