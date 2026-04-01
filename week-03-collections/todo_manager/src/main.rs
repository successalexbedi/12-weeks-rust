mod features;
use features::{TaskManager, Priority};



fn main() {
    // 1. Initialize the manager
    let mut manager = TaskManager::new();

    // 2. Add some tasks
    println!("--- Adding Tasks ---");
    manager.add_task("Finish Rust project", Priority::High);
    manager.add_task("Buy groceries", Priority::Medium);
    manager.add_task("Read a book", Priority::Low);

    // 3. Test the duplicate logic (This should print your error message)
    manager.add_task("Buy groceries", Priority::Low); 

    // 4. List everything
    println!("\n--- All Tasks ---");
    manager.list_all_tasks();
    println!("Total tasks: {}", manager.total_task());

    // 5. Mark a task complete
    println!("\n--- Completing 'Buy groceries' ---");
    manager.mark_task_as_complete("Buy groceries");

    // 6. Test Filtering
    println!("\n--- Incomplete Tasks ---");
    manager.list_incomplete_tasks();

    println!("\n--- Stats ---");
    println!("Completed: {}", manager.total_complete_task());
    // Since we added a helper for incomplete total:
    println!("Incomplete: {}", manager.total_task() - manager.total_complete_task());

    // 7. Test Removal
    println!("\n--- Removing 'Read a book' ---");
    manager.remove_task("Read a book");
    
    println!("\n--- Final Task Count ---");
    println!("Total: {}", manager.total_task());
}
