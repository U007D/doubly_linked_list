#![warn(clippy::all)]
#![allow(unsafe_code)] // Do not remove!  Explicitly change to #![allow(unsafe_code)] to use `unsafe` keyword.
//#![forbid(warnings)]
#![forbid(overflowing_literals)]
// Uncomment before ship to reconcile use of possibly redundant crates and uncover possible debug remnants
//#![warn(clippy::multiple_crate_versions, clippy::print_stdout, clippy::unimplemented, clippy::use_debug)]
// vvv Safety-critical application lints (pedantic: use for safety-critical applications only) vvv
#![deny(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss,
        clippy::cast_sign_loss, clippy::float_cmp_const, clippy::indexing_slicing, clippy::integer_arithmetic,
        clippy::maybe_infinite_iter, clippy::option_unwrap_used, clippy::result_unwrap_used)]
// ^^^ End of safety-critical lint section ^^^
#![allow(clippy::match_bool,)]
mod error;
mod node;
mod doubly_linked_list;
pub use self::{
    doubly_linked_list::DoublyLinkedList,
    error::Error,
    node::{
        Node,
        StrongLink,
        WeakLink,
    },
};
