


pub fn increment_all(numbers: &mut [i32; 10])

WHAT IT DOES:
- Add 1 to each number
- Modify in place

USE:
- for loop with &mut
- *num = *num + 1

OWNERSHIP:
- Mutably borrows (&mut)








pub fn negate_all(numbers: &mut [i32; 10])

WHAT IT DOES:
- Flip sign of each number
- Example: [5,-3,2] → [-5,3,-2]

USE:
- for loop with &mut
- *num = *num * -1

OWNERSHIP:
- Mutably borrows (&mut)










pub fn cap_at_max(numbers: &mut [i32; 10], max: i32)

WHAT IT DOES:
- If any number > max, set it to max
- Example: [15,25,35,45], max=30 → [15,25,30,30]

USE:
- for loop with &mut
- if *num > max, *num = max

OWNERSHIP:
- Mutably borrows (&mut)









pub fn swap_first_last(numbers: &mut [i32; 10])

WHAT IT DOES:
- Swap first and last element
- Example: [1,2,3,4,5] → [5,2,3,4,1]

USE:
- let temp = numbers[0]
- numbers[0] = numbers[9]
- numbers[9] = temp

OWNERSHIP:
- Mutably borrows (&mut)






