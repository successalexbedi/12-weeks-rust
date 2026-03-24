pub mod features;

pub use features::{
    create_item,
    add_stock,
    remove_stock,
    transfer_item,
    merge_items,
    check_stock_level, 
    can_fulfill_order, 
    get_restock_priority, 
    calculate_discount, 
    is_perishable, 
    needs_refrigeration,
    show_item, 
    show_low_stock_alert, 
    show_inventory_report,
};

fn main() {
    let mut apples = create_item(String::from("Apple"), 45);
    let mut milk = create_item(String::from("Milk"), 8);

    println!("Apples status: {}", check_stock_level(45));
    println!("Milk status: {}", check_stock_level(8));

    println!("Can fulfill 50 apples? {}", can_fulfill_order(45, 50));

    println!("Milk restock priority: {}", get_restock_priority(8, 5));

    println!("Discounted price: ${}", calculate_discount(10.0, 25));

    println!("Is 'Dairy' perishable? {}", is_perishable("Dairy"));

    println!("Does milk need refrigeration? {}", needs_refrigeration(&milk));

    add_stock(&mut milk, 10);

    if remove_stock(&mut milk, 5) {
        println!("Successfully removed 5 units of milk.");
    }

    apples = transfer_item(apples, "Warehouse A", "Warehouse B");

    show_low_stock_alert(&milk, 15);

    let extra_apples = create_item(String::from("Apple"), 20);

    let merged_apples = merge_items(apples, extra_apples);

    show_item(&merged_apples);
}
