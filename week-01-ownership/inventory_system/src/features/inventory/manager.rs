pub fn create_item(name: String, quantity: i32) -> String {
    format!("{}: {} units", name, quantity)
}

pub fn add_stock(item: &mut String, amount: i32) {
    let parts: Vec<&str> = item.split(": ").collect();
    let name = parts[0];
    let current_qty: i32 = parts[1]
        .replace(" units", "")
        .parse()
        .unwrap_or(0);

    let new_qty = current_qty + amount;
    *item = format!("{}: {} units", name, new_qty);
}

pub fn remove_stock(item: &mut String, amount: i32) -> bool {
    let parts: Vec<&str> = item.split(": ").collect();
    let name = parts[0];
    let current_qty: i32 = parts[1]
        .replace(" units", "")
        .parse()
        .unwrap_or(0);

    if current_qty >= amount {
        let new_qty = current_qty - amount;
        *item = format!("{}: {} units", name, new_qty);
        true
    } else {
        false
    }
}

pub fn transfer_item(item: String, from_warehouse: &str, to_warehouse: &str) -> String {
    println!("Transferring from {} to {}", from_warehouse, to_warehouse);
    item
}

pub fn merge_items(item1: String, item2: String) -> String {
    let parts1: Vec<&str> = item1.split(": ").collect();
    let parts2: Vec<&str> = item2.split(": ").collect();

    let name = parts1[0];
    
    let qty1: i32 = parts1[1].replace(" units", "").parse().unwrap_or(0);
    let qty2: i32 = parts2[1].replace(" units", "").parse().unwrap_or(0);

    format!("{}: {} units", name, qty1 + qty2)
}
