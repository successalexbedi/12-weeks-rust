mod features;

use features::*;

fn main() {
    let mut test_array = [15, 23, -8, 42, 16, 4, 23, 15, 35, -8];
    
    
    let sum = calculate_sum(&test_array);
    println!("Sum: {}", sum);
    
    let small = [2, 3, 4, 5, 6];
    let product = calculate_product(&small);
    println!("Product: {}", product);
    
    let in_range = count_in_range(&test_array, 10, 25);
    println!("In range [10,25]: {}", in_range);
    
    let range = find_range(&test_array);
    println!("Range (max-min): {}", range);
    
    let matches = count_matches(&test_array, 23);
    println!("Count of 23: {}", matches);
    
    
    let first = find_first_index(&test_array, 15);
    match first {
        Some(i) => println!("First 15 at index: {}", i),
        None => println!("Not found"),
    }
    
    let last = find_last_index(&test_array, 15);
    match last {
        Some(i) => println!("Last 15 at index: {}", i),
        None => println!("Not found"),
    }
    
    let has_42 = contains_value(&test_array, 42);
    println!("Contains 42: {}", has_42);
    
    let all_pos = are_all_positive(&test_array);
    println!("All positive: {}", all_pos);
    
    
    increment_all(&mut test_array);
    println!("After increment: {:?}", test_array);
    
    negate_all(&mut test_array);
    println!("After negate: {:?}", test_array);
    
    cap_at_max(&mut test_array, 20);
    println!("After cap at 20: {:?}", test_array);
    
    swap_first_last(&mut test_array);
    println!("After swap first/last: {:?}", test_array);
}