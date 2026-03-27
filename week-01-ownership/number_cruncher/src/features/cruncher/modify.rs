


pub fn increment_all(numbers: &mut [i32; 10]){
    for number in numbers {
         *number = *number + 1 
    }
}




pub fn negate_all(numbers: &mut [i32; 10]){
    for number in numbers {
         *number = *number * -1
    }
}


pub fn cap_at_max(numbers: &mut [i32; 10], max: i32){
    let maximum= max;
    for num in numbers {
         if *num > maximum {
             *num = maximum 
         }
    }
}





pub fn swap_first_last(numbers: &mut [i32; 10]){
     let temp = numbers[0];
     numbers[0] = numbers[9];
     numbers[9] = temp;
}

