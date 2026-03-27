pub fn starts_with_word(text: &str, prefix: &str) -> bool{
    
}
   - Check if text starts with prefix

2. pub fn ends_with_word(text: &str, suffix: &str) -> bool
   - Check if text ends with suffix

   
   
pub fn find_word_position(text: &str, word: &str) -> Option<usize>{
    
}
   - Find first occurrence
   - Return index or None

4. pub fn extract_words_with_length(text: &str, len: usize) -> Vec<String>
   - Find all words with exact length
   - Return Vec of Strings
   - NOTE: First time using Vec!

5. pub fn is_palindrome(text: &str) -> bool
   - Check if text reads same forwards/backwards
   - Ignore spaces and case
