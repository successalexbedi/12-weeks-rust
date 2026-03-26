

pub fn find_first_index(numbers: &[i32; 10], target: i32) -> Option<usize>

WHAT IT DOES:
- Find index of first occurrence of target
- Return Some(index) if found
- Return None if not found

USE:
- for loop with index (0..numbers.len())
- if numbers[i] == target
- break and return Some(i)

OWNERSHIP:
- Borrows array (&)










pub fn find_last_index(numbers: &[i32; 10], target: i32) -> Option<usize>

WHAT IT DOES:
- Find index of LAST occurrence
- Return Some(index) or None

USE:
- for loop backwards: (0..numbers.len()).rev()
- OR: loop forward and track last found

OWNERSHIP:
- Borrows array (&)










pub fn contains_value(numbers: &[i32; 10], target: i32) -> bool

WHAT IT DOES:
- Check if array contains target
- Return true if found, false otherwise

USE:
- for loop
- if num == target, return true
- break when found

OWNERSHIP:
- Borrows array (&)











pub fn are_all_positive(numbers: &[i32; 10]) -> bool

WHAT IT DOES:
- Check if ALL numbers > 0
- Return true only if all positive
- Return false if any negative or zero

USE:
- for loop
- if num <= 0, return false immediately
- If loop completes, return true

OWNERSHIP:
- Borrows array (&)






