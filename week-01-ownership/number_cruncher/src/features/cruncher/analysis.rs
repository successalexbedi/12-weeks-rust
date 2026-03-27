



pub fn calculate_sum(numbers: &[i32; 10]) -> i32 {
    let mut sum = 0;
    for number in numbers {
        sum += number 
    }
    sum
}




pub fn calculate_product(numbers: &[i32; 5]) -> i32 {
    let mut sum = 1;
    for number in numbers {
        sum *= number
    }
    sum
}



pub fn count_in_range(numbers: &[i32; 10], min: i32, max: i32) -> i32{
    let mut count = 0;
    for number in numbers {
        if number >= &min && *number <= max{
            count += 1
        };
    }
    count
}


pub fn find_range(numbers: &[i32; 10]) -> i32{
    let mut min = numbers[0];
    for number in numbers {
        if *number < min{
            min = *number
        }
    }
    
    
    let mut max = numbers[0];
    for number in numbers {
        if *number > max{
            max = *number 
        }
    }
    
    
    max - min
}


pub fn count_matches(numbers: &[i32; 10], target: i32) -> i32{
    let target= 2;
    let mut count = 0;
    for number in numbers {
        if *number == target {
            count+=1
        };
    }
    count
}

