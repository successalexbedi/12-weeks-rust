pub fn replace_word(text: &str, old: &str, new: &str) -> String {
    text.replace(old, new)
}


pub fn trim_text(text: &str) -> &str{
    text.trim()
}
   

pub fn to_title_case(text: &str) -> String {
    let mut result = String::new();

    for word in text.split_whitespace() {
        // Grab the first char, or a blank space if the word is empty
        let first = word.chars().next().unwrap_or(' ').to_uppercase();
        let rest = &word[1..];

        // Push the uppercase letter, the rest, and a space
        result.push_str(&first.to_string());
        result.push_str(rest);
        result.push(' ');
    }

    result.trim().to_string()
}



pub fn remove_punctuation(text: &str) -> String {
    let mut result = String::new();

    for c in text.chars() {
        if c.is_alphabetic() || c.is_whitespace() {
            result.push(c);
        }
    }

    result
}


pub fn truncate(text: &str, max_len: usize) -> &str {
    text.get(0..max_len).unwrap_or(text)
}
