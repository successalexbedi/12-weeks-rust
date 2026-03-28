pub fn starts_with_word(text: &str, prefix: &str) -> bool {
    // 
    match text.split_whitespace().next() {
        Some(first_word) => first_word == prefix,
        None => false, // 
    }
}


pub fn ends_with_word(text: &str, suffix: &str) -> bool {
    
    match text.split_whitespace().last() {
        Some(last_word) => last_word == suffix,
        None => false,
    }
}

   
   
pub fn find_word_position(text: &str, word: &str) -> Option<usize> {
    let mut current_index = 0;

    for w in text.split_whitespace() {
        if w == word {
            return Some(current_index); 
        }
        current_index += 1; 
    }

    None 
}


pub fn extract_words_with_length(text: &str, len: usize) -> Vec<String> {
    let mut my_list: Vec<String> = Vec::new();
    
    for word in text.split_whitespace() {
        if word.len() == len {
            my_list.push(word.to_string());
        }
    }
    my_list
}
