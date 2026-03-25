pub fn add_to_all(numbers: &mut [i32; 5], amount: i32)

WHAT IT DOES:
- Add amount to every number
- Modify in place
- Example: [1,2,3,4,5] + 10 → [11,12,13,14,15]

USE:
- for loop with &mut
- *num = *num + amount

OWNERSHIP:
- Mutably borrows (&mut)









pub fn multiply_all(numbers: &mut [i32; 5], factor: i32)

WHAT IT DOES:
- Multiply each number by factor
- Modify in place
- Example: [2,4,6,8,10] * 3 → [6,12,18,24,30]

USE:
- for loop with &mut
- *num = *num * factor

OWNERSHIP:
- Mutably borrows (&mut)






pub fn set_negatives_to_zero(numbers: &mut [i32; 5])

WHAT IT DOES:
- Change all negative numbers to 0
- Modify in place
- Example: [5,-3,8,-1,2] → [5,0,8,0,2]

USE:
- for loop with &mut
- if *num < 0
- *num = 0

OWNERSHIP:
- Mutably borrows (&mut)






pub fn clamp_values(numbers: &mut [i32; 5], min: i32, max: i32)

WHAT IT DOES:
- If number < min, set to min
- If number > max, set to max
- Otherwise leave it
- Example: array=[5,15,25,35,45], min=10, max=30
  → [10,15,25,30,30]

USE:
- for loop with &mut
- if/else to check bounds
- *num = min or *num = max

OWNERSHIP:
- Mutably borrows (&mut)