pub fn create_student(name: String, score: i32) -> (String, i32){
    (name, score)
}
   

pub fn update_score(student: &mut String, new_score: i32) {
    *student = format!("Student: score {new_score}");
}


pub fn transfer_student(student: String, from_class: &str, to_class: &str) -> String {
    println!("Transferring from {from_class} to {to_class}");
    student 
}


pub fn display_student(student: &String) {
    println!("Student Info: {student}");
}
