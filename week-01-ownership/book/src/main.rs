mod features;

use features::{
    create_book,
    transfer_book,
    update_title,
    read_book,
    show_book,
};

fn main() {
    // Create book
    let book = create_book(String::from("Rust Programming"));
    
    // Show it
    show_book(&book);
    
    // Read it (borrow)
    read_book(&book);
    
    // Still works!
    show_book(&book);
    
    // Transfer ownership
    let mut book = transfer_book(book, "Library", "Alex");
    
    // Update title (mutable borrow)
    update_title(&mut book, "Advanced Rust");
    
    // Show final result
    show_book(&book);
    
    // Read again
    read_book(&book);
}

