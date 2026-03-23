

pub fn borrow_book(book: String, borrower: &str) -> String{
    println!("transferring book to {borrower}");
    book
}


pub fn return_book(book: String) -> String{
    println!("Book returned to library");
    book
}


pub fn display_book(book: &String){
    println!("Book: {book}")
}
