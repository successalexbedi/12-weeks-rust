



pub fn calculate_sum(numbers: &[i32; 10]) -> i32{
    
}

WHAT IT DOES:
- Sum all numbers in array

USE:
- for loop
- Accumulator variable

OWNERSHIP:
- Borrows array (&)







pub fn calculate_product(numbers: &[i32; 5]) -> i32

WHAT IT DOES:
- Multiply all numbers together
- Example: [2,3,4] → 2*3*4 = 24

USE:
- for loop
- Start product at 1
- product = product * num

OWNERSHIP:
- Borrows array (&)









pub fn count_in_range(numbers: &[i32; 10], min: i32, max: i32) -> i32

WHAT IT DOES:
- Count numbers between min and max (inclusive)
- Example: array=[5,15,25,35], min=10, max=30 → 2

USE:
- for loop
- if num >= min && num <= max

OWNERSHIP:
- Borrows array (&)









pub fn find_range(numbers: &[i32; 10]) -> i32

WHAT IT DOES:
- Return max - min
- Example: [5,10,3,20,7] → 20-3 = 17

USE:
- for loop to find max
- for loop to find min
- Return difference

OWNERSHIP:
- Borrows array (&)











pub fn count_matches(numbers: &[i32; 10], target: i32) -> i32

WHAT IT DOES:
- Count how many times target appears
- Example: [1,2,3,2,5,2], target=2 → 3

USE:
- for loop
- Counter
- if num == target

OWNERSHIP:
- Borrows array (&)



