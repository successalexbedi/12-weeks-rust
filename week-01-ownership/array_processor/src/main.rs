mod features;

use features::{
    find_max,
    find_min,
    calculate_average,
    count_above_threshold,
    add_to_all,
    multiply_all,
    set_negatives_to_zero,
    clamp_values,
};

fn main() {
    let numbers = [15, 42, 8, 23, 16];
    
    // Stats tests
    let max = find_max(&numbers);
    println!("Max: {}", max);
    
    let min = find_min(&numbers);
    println!("Min: {}", min);
    
    let avg = calculate_average(&numbers);
    println!("Average: {:.2}", avg);
    
    let count = count_above_threshold(&numbers, 20);
    println!("Above 20: {}", count);
    
    // Transform tests
    let mut nums = [1, 2, 3, 4, 5];
    
    add_to_all(&mut nums, 10);
    println!("After add 10: {:?}", nums);
    
    multiply_all(&mut nums, 2);
    println!("After multiply 2: {:?}", nums);
    
    let mut with_negs = [5, -3, 8, -1, 2];
    set_negatives_to_zero(&mut with_negs);
    println!("Negatives to zero: {:?}", with_negs);
    
    let mut to_clamp = [5, 15, 25, 35, 45];
    clamp_values(&mut to_clamp, 10, 30);
    println!("Clamped [10,30]: {:?}", to_clamp);
}