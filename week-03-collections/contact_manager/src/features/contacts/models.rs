pub struct Contact {
    pub name: String,
    pub phone: String, 
    pub email: String,
}

pub struct ContactManager {
    pub contacts: Vec<Contact>,
}

impl ContactManager {
    pub fn new() -> Self {
        ContactManager {
            contacts: Vec::new(),
        }
    }


    
    
    pub fn create_contact(name: &str, phone: &str, email: &str) -> Contact {
        Contact {
            name: name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
        }
    }
    
    
    
    pub fn add_contact(&mut self, new_person: Contact) {
        let already_exists = self.contacts.iter().any(|c| c.name == new_person.name);

        if already_exists {
            println!("Error: Contact '{}' already exists!", new_person.name);
        } else {
            self.contacts.push(new_person);
            println!("Success: Contact added.");
        }
    }
    
    
    pub fn list_all_contacts(&self) {
        for contact in &self.contacts {
            println!("Name: {}, Phone: {}, Email: {}", contact.name, contact.phone, contact.email);
        }
    }
    
    
    
    
    pub fn count_total_contacts(&self) -> usize {
        self.contacts.len()
    }
    
    
    pub fn find_contact_by_name(&self, name_to_find: &str) {
        let found = self.contacts.iter().any(|c| c.name == name_to_find);

        if found {
            println!("Contact found: {}", name_to_find);
        } else {
            println!("Contact '{}' not found.", name_to_find);
        }
    }


    
    pub fn remove_contact_by_name(&mut self, name_to_remove: &str) {
        self.contacts.retain(|c| c.name != name_to_remove);
    }


}
