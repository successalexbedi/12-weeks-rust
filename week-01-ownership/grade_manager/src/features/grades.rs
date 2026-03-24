pub mod evaluator;
pub mod manager;

pub use evaluator::{
    get_letter_grade,
    is_passing,
    get_status,
    apply_curve,
    compare_scores,
};


pub use manager::{
    update_score,
    create_student,
    transfer_student,
    display_student,
};