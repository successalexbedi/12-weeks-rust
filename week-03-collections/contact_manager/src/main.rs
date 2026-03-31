mod features; // This loads the folder

// Direct import from the "features" shortcut you created
use features::{Contact, ContactManager};

fn main() {
    let mut manager = ContactManager::new();

    // 1. ADD
    println!("--- Testing Add ---");
    manager.add_contact(Contact {
        name: "IDGΛF+".to_string(),
        phone: "08012345678".to_string(),
        email: "artist@bolt.dev".to_string(),
    });
    manager.add_contact(Contact {
        name: "P Ξ Λ C H Y".to_string(),
        phone: "07099988877".to_string(),
        email: "premium@peachy.io".to_string(),
    });

    // 2. LIST & COUNT
    println!("\n--- Testing List ---");
    println!("Total: {}", manager.count_total_contacts());
    manager.list_all_contacts();

    // 3. FIND
    println!("\n--- Testing Find ---");
    manager.find_contact_by_name("P Ξ Λ C H Y");

    // 4. REMOVE
    println!("\n--- Testing Remove ---");
    manager.remove_contact_by_name("IDGΛF+");
    
    // 5. FINAL CHECK
    println!("\n--- Final List ---");
    manager.list_all_contacts();
}

