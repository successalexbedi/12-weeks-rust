#[derive(Debug, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
pub struct Task {
    pub description: String,
    pub complete_status: bool,
    pub priority: Priority,
}

pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    

    pub fn add_task(&mut self, description: &str, priority: Priority) {
        let exists = self.tasks.iter().any(|t| t.description == description);

        if exists {
            println!("Error: '{}' already exists.", description);
        } else {
            let new_task = Task {
                description: description.to_string(),
                complete_status: false,
                priority,
            };
            self.tasks.push(new_task);
        }
    }
    
    

    pub fn mark_task_as_complete(&mut self, description: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.description == description) {
            task.complete_status = true;
        }
    }
    
    

    pub fn remove_task(&mut self, description: &str) {
        if let Some(index) = self.tasks.iter().position(|t| t.description == description) {
            self.tasks.remove(index);
        } else {
            println!("Does not exist: {}", description);
        }
    }
    
    

    pub fn list_all_tasks(&self) {
        println!("{:?}", self.tasks);
    }
    

    pub fn list_incomplete_tasks(&self) {
        for task in self.tasks.iter().filter(|t| !t.complete_status) {
            println!("{:?}", task);
        }
    }
    
    

    pub fn total_task(&self) -> usize {
        self.tasks.len()
    }
    
    

    pub fn total_complete_task(&self) -> usize {
        self.tasks.iter().filter(|t| t.complete_status).count()
    }
    
    
}
