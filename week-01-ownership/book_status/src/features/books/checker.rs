

pub fn check_availability(title: &String, owner: &str) -> &'static str{
    match owner{
        "library" => "available",
        "alex" => "checkout",
        "Bob" => "reserved",
        _ => "unknown",
    }
    
}



pub fn can_borrow(_book: &String, current_owner: &str, requester_age: i32) -> bool {
    current_owner.to_lowercase() == "library" && requester_age >= 13
}



pub fn calculate_late_fee(days_late: i32) -> f64 {
    let days = days_late as f64;
    match days_late {
        0 => 0.00,
        1..=7 => days * 0.50,
        8..=14 => days * 1.00,
        15.. => days * 2.00,
        _ => 0.00,           
    }
}


pub fn get_category(title: &str) -> &'static str {
    if title.contains("Rust") || title.contains("Python") {
        "Programmig"
    } else if title.contains("History") {
        "Non-Fiction"
    } else if title.contains("Novel") {
        "Fiction"
    } else {
        "General"
    }
}
