

pub fn count_vowels(text: &str) -> usize {
    let case = text.to_lowercase();
    let ca: Vec<char> = case.chars().collect();
    let mut count = 0;
    
    for c in ca {
       if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
          count += 1;
       }
    }
    
    count
}


pub fn count_consonants(text: &str) -> usize{
    let case = text.to_lowercase();
    let ca: Vec<char> = case.chars().collect();
    let mut count = 0;
    
    for c in ca{
        if c.is_alphabetic() && !( c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u');
        count += 1;
    }
    count
}
   

pub fn count_digits(text: &str) -> usize {
    let mut count = 0;
    for c in text.chars() {
        if c.is_ascii_digit() {
            count += 1;
        }
    }
    
    count
}


pub fn count_unique_words(text: &str) -> usize {
    let case = text.to_lowercase();
    let words = case.split_whitespace();
    
    // This is our "tracking" list to store words we've already seen
    let mut seen_words: Vec<&str> = Vec::new();
    
    for word in words {
        // We check: "Is this word NOT already in our tracking list?"
        if !seen_words.contains(&word) {
            seen_words.push(word);
        }
    }
    
    // The number of items in our tracking list is the unique count
    seen_words.len()
}

   
   

pub fn average_word_length(text: &str) -> f64 {
    let mut total_chars = 0;
    let mut word_count = 0;

    for word in text.split_whitespace() {
        total_chars += word.len(); 
        word_count += 1;         
    }

    
    if word_count == 0 {
        return 0.0;
    }

    total_chars as f64 / word_count as f64
}
