pub mod analysis;
pub mod search;
pub mod modify;

pub use analysis::{
    calculate_sum,
    calculate_product,
    count_in_range,
    find_range,
    count_matches,
};


pub use search::*;
pub use modify::*;