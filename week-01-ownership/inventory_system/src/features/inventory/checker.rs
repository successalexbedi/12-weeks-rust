pub fn check_stock_level(quantity: i32) -> &'static str {
    match quantity {
        0 => "Out of stock",
        1..=10 => "Low stock",
        11..=50 => "Normal stock",
        51.. => "High stock",
        _ => "Invalid"       
    }
}




pub fn can_fulfill_order(available: i32, requested: i32) -> bool{
    available>= requested
}
   

pub fn get_restock_priority(quantity: i32, sales_per_day: i32) -> &'static str {
    if quantity == 0 {
        "URGENT"
    } else if quantity / sales_per_day < 3 {
        "HIGH"
    } else if quantity / sales_per_day < 7 {
        "MEDIUM"
    } else {
        "LOW"
    }
}




pub fn calculate_discount(price: f64, quantity: i32) -> f64 {
    let discount_percent = match quantity {
        1..=5 => 0.0,
        6..=20 => 10.0,
        21..=50 => 15.0, 
        51.. => 20.0,
        _ => 0.0,        
    };

    price * (1.0 - (discount_percent / 100.0))
}



pub fn is_perishable(category: &str) -> bool {
    match category {
        "Food" | "Dairy" | "Produce" => true,
        _ => false,
    }
}


pub fn needs_refrigeration(item: &String) -> bool {
    let item_lower = item.to_lowercase();
    
    if item_lower.contains("milk") || 
       item_lower.contains("cheese") || 
       item_lower.contains("yogurt") {
        true
    } else {
        false
    }
}
