#[doc = include_str!("../README.md")]

mod check;

pub use check::{CheckType, check_result, check_struct, check_value, check_option};