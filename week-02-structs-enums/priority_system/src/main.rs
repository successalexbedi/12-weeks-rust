// 1. We need the Enum defined
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

pub struct Task {
    name: String,
    priority: Priority,
}


pub fn create_task(name: String, priority: Priority) -> Task{
    Task{
        name,
        priority,
    }
}


pub fn display_task(task: &Task) {
    
    let priority_name = match task.priority {
        Priority::Low => "Low",
        Priority::Medium => "Medium",
        Priority::High => "High",
        Priority::Critical => "Critical",
    };

    println!("Task: {}, Priority: {}", task.name, priority_name);
}





pub fn is_urgent(priority: &Priority) -> bool {
    match priority {
        Priority::High | Priority::Critical => true,
        _ => false, 
    }
}



pub fn priority_level(priority: &Priority) -> i32{
    match priority{
        Priority::Low => 1,
        Priority::Medium => 2,
        Priority::High => 3,
        Priority::Critical => 4,
    }
}




pub fn compare_priority(p1: &Priority, p2: &Priority) -> String {
    let py1 = priority_level(p1);
    let py2 = priority_level(p2);
    
    if py1 > py2 {
        format!("First is higher")
    } else if py2 > py1 {
        format!("Second is higher")
    } else {
        format!("Same priority")
    }
}
















fn main() {
    // 1. Create 4 tasks with different priorities
    let t1 = create_task(String::from("Fix login bug"), Priority::Critical);
    let t2 = create_task(String::from("Update docs"), Priority::Low);
    let t3 = create_task(String::from("Add feature X"), Priority::High);
    let t4 = create_task(String::from("Style buttons"), Priority::Medium);

    // Put them in a Vector to make displaying/checking easier
    let all_tasks = [&t1, &t2, &t3, &t4];

    // 2. Display all tasks
    println!("\n🚀 --- ALL TASKS ---");
    println!("{:-<40}", ""); // Quick divider line
    for task in all_tasks.iter() {
        display_task(task);
    }

    // 3. Check which tasks are urgent & 4. Print priority levels
    println!("\n🔍 --- STATUS CHECK ---");
    println!("{:<20} | {:<10} | {:<5}", "TASK", "STATUS", "LEVEL");
    println!("{:-<40}", "");
    for task in all_tasks.iter() {
        let urgent_status = if is_urgent(&task.priority) { "🔥 URGENT" } else { "✅ Normal" };
        let level = priority_level(&task.priority);
        // Using {:<20} keeps the columns perfectly aligned
        println!("{:<20} | {:<10} | Lvl: {}", task.name, urgent_status, level);
    }

    // 5. Compare priorities pairwise & 6. Print all results
    println!("\n⚖️  --- PAIRWISE COMPARISONS ---");
    
    // Compare Task 1 vs Task 2
    let cmp1 = compare_priority(&t1.priority, &t2.priority);
    println!("├─ '{}' vs '{}'", t1.name, t2.name);
    println!("│  ↳ Result: {}", cmp1);

    // Compare Task 3 vs Task 4
    let cmp2 = compare_priority(&t3.priority, &t4.priority);
    println!("├─ '{}' vs '{}'", t3.name, t4.name);
    println!("│  ↳ Result: {}", cmp2);

    // Compare Task 1 vs Task 3
    let cmp3 = compare_priority(&t1.priority, &t3.priority);
    println!("└─ '{}' vs '{}'", t1.name, t3.name);
    println!("   ↳ Result: {}", cmp3);
    
    println!("\n✨ Done.");
}
