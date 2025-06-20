#![deny(
    //防止副作用和 panic 。
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

    //禁止不安全和不确定行为
    clippy::float_arithmetic,
    clippy::mem_forget,
    clippy::as_conversions,

    // 提升代码质量(使用allow的时候必须提供理由)
    clippy::allow_attributes_without_reason,
)]


pub mod user;
pub mod venue;
pub mod domain_core_error;
pub mod rental;
pub mod utils;
