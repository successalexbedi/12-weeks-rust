mod features;

use features::{
    get_letter_grade,
    is_passing,
    get_status,
    apply_curve,
    compare_scores,
    update_score,
    create_student,
    transfer_student,
    display_student,
};

fn main() {
    // 1. Create student "Alex" with score 85
    let name = String::from("Alex");
    let mut score = 85;

    // 2. Get letter grade
    let grade = get_letter_grade(score);
    println!("Grade: {grade}");

    // 3. Check if passing
    let passing = is_passing(score);
    println!("Is passing: {passing}");

    // 4. Get status (borrow name)
    let status = get_status(&name, score);
    println!("Status: {status}");

    // 5. Apply curve of 10 points (mutable borrow score)
    apply_curve(&mut score, 10);

    // 6. Compare with score of 92
    let comparison = compare_scores(score, 92);
    println!("Comparison to 92: {comparison}");

    // 7. Create student record (returns a tuple)
    let (student_name, student_score) = create_student(name, score);

    // 8. Display it
    // Note: We need to format them into a string since display_student expects &String
    let mut student_record = format!("{} - {}", student_name, student_score);
    display_student(&student_record);

    // 9. Update score to 98 (mutable borrow)
    update_score(&mut student_record, 98);

    // 10. Transfer student from "Math 101" to "Math 201" (move and return)
    student_record = transfer_student(student_record, "Math 101", "Math 201");

    // 11. Display final
    display_student(&student_record);
}
