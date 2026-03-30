pub struct Book {
    title: String,
    author: String,
    pages: i32,
    available: bool,
}

impl Book {
    pub fn new(title: String, author: String, pages: i32) -> Book {
        Book {
            title,
            author,
            pages,
            available: true,
        }
    }

    pub fn display(&self) {
        
        let avail = if self.available { "Yes" } else { "No" };
        println!("Title: {}, Author: {}, Pages: {}, Available: {}", 
                 self.title, self.author, self.pages, avail);
    }

    pub fn is_long(&self) -> bool {
        self.pages > 300
    }

    pub fn checkout(&mut self) {
        self.available = false;
        println!("Book checked out: {}", self.title);
    }

    pub fn return_book(&mut self) {
        self.available = true;
        println!("Book returned: {}", self.title);
    }

    pub fn summary(&self) -> String {
        format!("{} by {} ({} pages)", self.title, self.author, self.pages)
    }
}




fn main() {
    // 1. Create 3 books using Book::new()
    // We make them 'mut' because we'll be changing their 'available' status
    let mut book1 = Book::new(String::from("The Rust Programming Language"), String::from("Steve Klabnik"), 500);
    let mut book2 = Book::new(String::from("Programming Rust"), String::from("Jim Blandy"), 700);
    let book3 = Book::new(String::from("Zero to Production"), String::from("Luca Palmieri"), 250);

    // 2. Display all books
    println!("--- Initial Inventory ---");
    book1.display();
    book2.display();
    book3.display();

    // 3. Check if each is long
    println!("\n--- Length Check ---");
    for b in [&book1, &book2, &book3] {
        if b.is_long() {
            println!("'{}' is a long book!", b.title);
        }
    }

    // 4. Checkout 2 books
    println!("\n--- Checking Out ---");
    book1.checkout();
    book2.checkout();

    
    println!("\n--- Status After Checkout ---");
    book1.display();
    book2.display();
    book3.display();

    // 6. Return 1 book
    println!("\n--- Returning ---");
    book1.return_book();

    // 7. Display all books again
    println!("\n--- Final Status ---");
    book1.display();
    book2.display();
    book3.display();

    
    println!("\n--- Summaries ---");
    println!("{}", book1.summary());
    println!("{}", book2.summary());
    println!("{}", book3.summary());
}







