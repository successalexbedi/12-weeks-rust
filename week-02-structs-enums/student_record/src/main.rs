pub struct Student {
    name: String,
    id: i32,
    grade: i32,
    passed: bool,
}

pub fn create_student(name: String, id: i32, grade: i32) -> Student {
    // You can set 'passed' directly during instantiation
    Student {
        name,
        id,
        grade,
        passed: grade >= 60, 
    }
}

pub fn display_student(student: &Student) {
    // Rust allows shorthand for if/else assignments
    let yes = if student.passed { "yes" } else { "no" };
    
    println!("ID: {}, Name: {}, Grade: {}, Passed: {}", 
        student.id, student.name, student.grade, yes)
}

pub fn update_grade(student: &mut Student, new_grade: i32) {
    student.grade = new_grade;
    student.passed = new_grade >= 60;
}

pub fn is_honor_student(student: &Student) -> bool {
    student.grade >= 90
}

pub fn compare_students(s1: &Student, s2: &Student) -> String {
    // Fixed lowercase 'if' and added format! macro
    if s1.grade > s2.grade {
        format!("{} scored higher", s1.name)
    } else if s2.grade > s1.grade {
        format!("{} scored higher", s2.name)
    } else {
        String::from("Both scored equally")
    }
}

fn main() {
    // 1. Create 3 students with different grades
    let mut s1 = create_student(String::from("Alice"), 101, 88);
    let s2 = create_student(String::from("Bob"), 102, 95);
    let s3 = create_student(String::from("Charlie"), 103, 55);

    // 2. Display all 3
    println!("--- Initial Students ---");
    display_student(&s1);
    display_student(&s2);
    display_student(&s3);

    // 3. Update one student's grade
    // We update Alice from 88 to 92 (making her an honor student)
    update_grade(&mut s1, 92);

    // 4. Display updated student
    println!("\n--- After Grade Update ---");
    display_student(&s1);

    // 5. Check if each is honor student
    println!("\n--- Honor Roll Status ---");
    println!("{}: {}", s1.name, if is_honor_student(&s1) { "Honor Student" } else { "Regular" });
    println!("{}: {}", s2.name, if is_honor_student(&s2) { "Honor Student" } else { "Regular" });
    println!("{}: {}", s3.name, if is_honor_student(&s3) { "Honor Student" } else { "Regular" });

    // 6. Compare students pairwise
    let cmp1 = compare_students(&s1, &s2);
    let cmp2 = compare_students(&s2, &s3);
    let cmp3 = compare_students(&s1, &s3);

    // 7. Print all results
    println!("\n--- Pairwise Comparisons ---");
    println!("Alice vs Bob: {}", cmp1);
    println!("Bob vs Charlie: {}", cmp2);
    println!("Alice vs Charlie: {}", cmp3);
}
