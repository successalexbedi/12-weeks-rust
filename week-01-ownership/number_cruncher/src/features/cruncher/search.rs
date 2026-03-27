/// Finds the index of the first occurrence of the target.
pub fn find_first_index(numbers: &[i32; 10], target: i32) -> Option<usize> {
    for i in 0..numbers.len() {
        if numbers[i] == target {
            return Some(i);
        }
    }
    None
}

/// Finds the index of the last occurrence of the target by searching in reverse.
pub fn find_last_index(numbers: &[i32; 10], target: i32) -> Option<usize> {
    // .rev() allows us to start from the end of the range
    for i in (0..numbers.len()).rev() {
        if numbers[i] == target {
            return Some(i);
        }
    }
    None
}

/// Checks if the array contains a specific value.
pub fn contains_value(numbers: &[i32; 10], target: i32) -> bool {
    for &num in numbers {
        if num == target {
            return true;
        }
    }
    false
}

/// Returns true only if every element in the array is greater than zero.
pub fn are_all_positive(numbers: &[i32; 10]) -> bool {
    for &num in numbers {
        if num <= 0 {
            // Short-circuit: return false immediately if a non-positive is found
            return false;
        }
    }
    true
}
