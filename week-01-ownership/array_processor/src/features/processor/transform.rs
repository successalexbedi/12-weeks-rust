pub fn add_to_all(numbers: &mut [i32; 5], amount: i32) {
    for number in numbers { 
        *number = *number + amount;
    }
}

pub fn multiply_all(numbers: &mut [i32; 5], factor: i32) {
    for number in numbers {
        *number = *number * factor; 
    }
}

pub fn set_negatives_to_zero(numbers: &mut [i32; 5]) {
    for number in numbers {
        
        if *number < 0 {
            *number = 0; 
        }
    }
}

pub fn clamp_values(numbers: &mut [i32; 5], min: i32, max: i32) {
    for number in numbers {
        if *number < min {
            *number = min;
        }
        
        if *number > max {
            *number = max;
        }
    }
}
