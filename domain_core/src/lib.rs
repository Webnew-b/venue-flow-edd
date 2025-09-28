#![deny(
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::dbg_macro,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::exit,
    clippy::todo,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::integer_division,
    clippy::float_arithmetic,
    clippy::mem_forget,
    clippy::as_conversions,
    clippy::allow_attributes_without_reason
)]

pub mod domain_core_error;
pub mod rental;
pub mod user;
pub mod utils;
pub mod venue;
