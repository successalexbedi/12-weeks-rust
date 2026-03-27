pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

pub fn count_lines(text: &str) -> usize {
    text.lines().count()
}


pub fn find_longest_word(text: &str) -> &str {
    let mut longest = ""; 
    for word in text.split_whitespace() {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    
    longest 
}

pub fn count_occurrences(text: &str, word: &str) -> usize {
    let mut count = 0;
    let target = word.to_lowercase(); 
    
    for t in text.split_whitespace() {
        if t.to_lowercase() == target {
            count += 1;
        }
    }
    count
}


pub fn get_word_at_index(text: &str, index: usize) -> Option<&str> {
    text.split_whitespace().nth(index)
}
