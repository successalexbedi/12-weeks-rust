fn main() {
    let n = 10;
    let sum = sum_to_n(n);
    println!("The sum of numbers from 1 to {} is: {}", n, sum);
    
    
    
   let evens = [2,5,8,11,14];
   let count = count_evens(&evens);
   println!("Even count: {}", count);
   
   
   
   
   
   
   
   
   let nums = [5,3,-2,8,1,-7];
   let first_neg = find_first_negative(&nums);
   match first_neg {
       Some(n) => println!("First negative: {}", n),
       None => println!("No negatives"),
   }
   
   
   
   
   
   
   let result = multiply_until(2, 100);
   println!("Multiply until: {}", result);
   
   
   
   
   
   print_skip_threes(15);
   reverse_countdown(5);
 
   let mut to_double = [1,2,3,4,5];
   double_all(&mut to_double);
   println!("Doubled: {:?}", to_double);

   let big_nums = [30,40,50,20,10,5,2,1,1,1];
   let sum = sum_until_hundred(&big_nums);
   println!("Sum until 100: {}", sum);
   
}


fn sum_to_n(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}


fn count_evens(numbers: &[i32; 5]) -> i32{
   
   let mut count = 0;
   
   for number in numbers {
       if number % 2 == 0 {
            println!("number: {number} is even");
          count += 1;
       }
   }
   
   count
}





fn find_first_negative(numbers: &[i32; 6]) -> Option<i32> {
    for &number in numbers {
        if number < 0 {
            return Some(number);
        }
    }
    
    None 
}




fn multiply_until(start: i32, limit: i32) -> i32 {
    let mut current_value = start; 
    loop {
        if current_value > limit {
            break; 
        }
        current_value *= 2; 
    }
    current_value 
}





fn print_skip_threes(max: i32){
    for m in 1..=max{
        if m % 3 == 0 {
            continue 
            
        }
        println!("{}", m);
    }
}




fn reverse_countdown(mut n: i32){
    while  n > 0 {
        println!("{n}");
        n -= 1; 
    }
    println!("Go!");
}





fn double_all(numbers: &mut [i32; 5]){
    for number in numbers {
       *number *= 2
    }
}


fn sum_until_hundred(numbers: &[i32; 10]) -> i32{
    let mut sum = 0;
    for number in numbers {
        if sum == 100{
            break
        }
        sum += number
        
    }
    sum 
}

