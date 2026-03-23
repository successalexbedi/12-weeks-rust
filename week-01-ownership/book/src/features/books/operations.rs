



pub fn create_book(title: String)-> String{
    title 
}


pub fn transfer_book(book: String, from: &str, to: &str) -> String{
    println!("{book} is transfered from {from} to {to}");
    book
}

pub fn update_title(book: &mut String, new_title: &str){
   let stuff = book;
   *stuff = new_title.to_string();
   
}