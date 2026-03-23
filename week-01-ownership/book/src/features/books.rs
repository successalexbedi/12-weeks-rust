pub mod operations;
pub mod display;

pub use operations::{
    create_book,
    transfer_book,
    update_title,
};

pub use display::{
    read_book,
    show_book,
};