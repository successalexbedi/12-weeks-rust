fn main() {
    let s1 = "Hello";
    let s2 = "World";
    let text = "Rust is amazing 🦀";

    // 1. Test Concatenation
    let joined = concat_strings(s1, s2);
    println!("Concatenate: '{}'", joined); // "Hello World"

    // 2. Test Reversing
    let reversed = reverse_string(s1);
    println!("Reverse: '{}' -> '{}'", s1, reversed); // "olleH"

    // 3. Test Uppercase
    let upper = to_uppercase_custom(text);
    println!("Uppercase: '{}'", upper); // "RUST IS AMAZING 🦀"

    // 4. Test Remove Whitespace
    let no_space = remove_whitespace("a b c");
    println!("No spaces: '{}'", no_space); // "abc"

    // 5. Test First Word
    let first = first_word(text);
    println!("First word: '{}'", first); // "Rust"

    // 6. Test Character Count
    let count = count_chars(text);
    println!("Char count of '{}': {}", text, count); // 16 (The emoji is 1 char)

    // 7. Test Substring
    let has_rust = contains_substring(text, "Rust");
    let has_java = contains_substring(text, "Java");
    println!("Contains 'Rust': {}", has_rust); 
    println!("Contains 'Java': {}", has_java);
}


fn concat_strings(s1: &str, s2: &str) -> String {
    let mut result = String::from(s1);
    result.push(' ');
    result.push_str(s2);
    result
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn to_uppercase_custom(s: &str) -> String {
    s.to_uppercase() 
}

fn remove_whitespace(s: &str) -> String {
    s.replace(" ", "")
}

fn first_word(s: &str) -> &str {
    s.split_whitespace().nth(0).unwrap_or("")
}

fn count_chars(s: &str) -> usize {
    s.chars().count() 
}

fn contains_substring(s: &str, sub: &str) -> bool {
    
    s.contains(sub)
}
