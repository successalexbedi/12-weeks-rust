mod features;

use features::{
    count_words,
    count_lines,
    find_longest_word,
    count_occurrences,
    get_word_at_index,
    replace_word,
    trim_text,
    to_title_case,
    remove_punctuation,
    truncate,
};

fn main() {
    let sample = "  learning rust is fun!  ";
    let target_word = "rust";

    println!("Original: '{}'", sample);

    // 1. Basic Counts
    println!("Words: {}", count_words(sample));
    println!("Lines: {}", count_lines(sample));

    // 2. Finding things
    println!("Longest word: {}", find_longest_word(sample));
    println!("Occurrences of '{}': {}", target_word, count_occurrences(sample, target_word));
    
    println!("Word at index 1: {}", get_word_at_index(sample, 1).unwrap_or("None"));

    // 3. Changing things
    println!("Replaced: {}", replace_word(sample, "fun", "awesome"));
    println!("Title Case: {}", to_title_case(sample));
    println!("No Punctuation: {}", remove_punctuation(sample));

    // 4. Cutting things
    let clean = trim_text(sample);
    println!("Trimmed: '{}'", clean);
    println!("Truncated (8): {}", truncate(clean, 8));
}
