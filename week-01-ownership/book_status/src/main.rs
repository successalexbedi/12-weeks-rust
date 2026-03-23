mod features;
pub use features::{
    borrow_book,
    return_book,
    display_book,
    check_availability,
    can_borrow,
    calculate_late_fee,
    get_category,
};


fn main() {
    // 1. Create a book
    let mut book = String::from("Rust Programming by Alex");

    // 2. Get category (borrow)
    let category = get_category(&book);
    println!("Category: {category}");

    // 3. Display it (borrow)
    display_book(&book);

    // 4. Check availability with "library" (borrow)
    let status = check_availability(&book, "library");
    println!("Status: {status}");

    // 5. Check if 15-year-old can borrow (borrow)
    if can_borrow(&book, "library", 15) {
        println!("Access granted for 15-year-old.");
    }

    // 6. Borrow book to "Alex" (move and return)
    // We move 'book' in, and catch the returned 'book' back into the variable
    book = borrow_book(book, "Alex");

    // 7. Update owner from "Alex" to "Bob" (mutable borrow)
    // Note: We use .replace() here to update the string content
    book = book.replace("Alex", "Bob");
    println!("Owner updated to Bob");

    // 8. Calculate late fee for 10 days
    let fee = calculate_late_fee(10);
    println!("Late fee for 10 days: ${fee:.2}");

    // 9. Return book (move and return)
    book = return_book(book);

    // 10. Display final book (borrow)
    display_book(&book);
}
