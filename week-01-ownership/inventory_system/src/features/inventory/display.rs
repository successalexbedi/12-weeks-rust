pub fn show_item(item: &String) {
    println!("item: {item}");
}



pub fn show_low_stock_alert(item: &String, threshold: i32) {
    let quantity: i32 = match item.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if quantity < threshold {
        println!("LOW STOCK ALERT: {item}");
    }
}




pub fn show_inventory_report(items: &[String]) {
    for item in items {
        println!("{item}");
    }
}
