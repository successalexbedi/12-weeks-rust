pub fn find_max(numbers: &[i32; 5]) -> i32

WHAT IT DOES:
- Find largest number in array
- Return the max value

USE:
- for loop
- Track max value
- if num > max

OWNERSHIP:
- Borrows array (&)












pub fn find_min(numbers: &[i32; 5]) -> i32

WHAT IT DOES:
- Find smallest number in array
- Return the min value

USE:
- for loop
- Track min value
- if num < min

OWNERSHIP:
- Borrows array (&)









pub fn calculate_average(numbers: &[i32; 5]) -> f64

WHAT IT DOES:
- Calculate average of array
- Return as f64
- Example: [10,20,30,40,50] → 30.0

USE:
- for loop to sum
- Divide by length
- Convert to f64

OWNERSHIP:
- Borrows array (&)











pub fn count_above_threshold(numbers: &[i32; 5], threshold: i32) -> i32

WHAT IT DOES:
- Count how many numbers > threshold
- Example: [10,20,30,40,50], threshold=25 → 3

USE:
- for loop
- Counter variable
- if num > threshold

OWNERSHIP:
- Borrows array (&)
- threshold is Copy