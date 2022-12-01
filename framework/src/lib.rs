#![feature(auto_traits)]
#![feature(decl_macro)]
#![feature(entry_insert)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(negative_impls)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

pub mod astr;
pub mod error;
pub mod graph;
pub mod inputs;
pub mod iter;
pub mod offsets;
pub mod outputs;
pub mod parsers;
pub mod prelude;
pub mod result;
pub mod runner;
pub mod util;
pub mod vecs;

pub use paste::paste;
