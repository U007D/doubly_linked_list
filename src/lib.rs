#![warn(clippy::all)]
#![forbid(unsafe_code)] // Do not remove!  Explicitly change to #![allow(unsafe_code)] to use `unsafe` keyword.
#![allow(clippy::match_bool,)]
#![deny(warnings)]
#![forbid(overflowing_literals)]
// vvv Safety-critical application lints (pedantic: use for safety-critical applications only) vvv
#![deny(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss,
clippy::cast_sign_loss, clippy::float_cmp_const, clippy::indexing_slicing, clippy::integer_arithmetic,
clippy::maybe_infinite_iter, clippy::option_unwrap_used, clippy::result_unwrap_used)]
// ^^^ End of safety-critical lint section ^^^
// Uncomment before ship to reconcile use of possibly redundant crates and uncover possible debug remnants
// #![warn(clippy::multiple_crate_versions, clippy::print_on_stdout, clippy::unimplemented, clippy::use_debug)]

mod error;
#[cfg(test)]
mod unit_tests;
pub use self::error::Error;
