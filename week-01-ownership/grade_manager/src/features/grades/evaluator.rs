pub fn get_letter_grade(score: i32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'E',
        _ => 'X', 
    }
}


pub fn is_passing(score: i32) -> bool {
    score >= 60
}

pub fn get_status(name: &String, score: i32) -> String {
    if is_passing(score) {
        format!("{name} is passing with {score}")
    } else {
        format!("{name} is failing with {score}")
    }
}


     
pub fn apply_curve(score: &mut i32, curve: i32) {
    *score += curve;
    if *score > 100 {
        *score = 100;
    }
}

pub fn compare_scores(score1: i32, score2: i32) -> &'static str {
    if score1 > score2 { 
        "first is higher"
    } else if score1 < score2 {
        "second is higher"
    } else {
        "equal"
    }
}
