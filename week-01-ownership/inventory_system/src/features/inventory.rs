pub mod checker;
pub mod display;
pub mod manager;





pub use manager::{
    create_item,
    add_stock,
    remove_stock,
    transfer_item,
    merge_items,
};


pub use checker::{
    check_stock_level, 
    can_fulfill_order, 
    get_restock_priority, 
    calculate_discount, 
    is_perishable, 
    needs_refrigeration,
};


pub use display::{
    show_item, 
    show_low_stock_alert, 
    show_inventory_report,
};
