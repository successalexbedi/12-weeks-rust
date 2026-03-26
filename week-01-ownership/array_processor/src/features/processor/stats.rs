pub fn find_max(numbers: &[i32; 5]) -> i32 {
    let mut max = numbers[0];
    for &number in numbers {
        if number > max {
            max = number; 
        }
    }
    max 
}

pub fn find_min(numbers: &[i32; 5]) -> i32 {
    let mut small = numbers[0]; 
    for &number in numbers {
        if number < small {
            small = number; 
        }
    }
    small 
}

pub fn calculate_average(numbers: &[i32; 5]) -> f64 {
    let count = numbers.len();
    let mut sum = 0; 
    
    for &number in numbers {
        sum += number; 
    }
    
    let sum_f = sum as f64;
    let count_f = count as f64;
    sum_f / count_f
}

pub fn count_above_threshold(numbers: &[i32; 5], threshold: i32) -> i32 {
    let mut counter = 0; 
    for &num in numbers {
        if num > threshold {
            counter += 1; 
        }
    }
    counter
}
