#![allow(dead_code, unused)]
// src/lib.rs
// Main library file for the str_tools crate.
// This file declares all modules and re-exports the public iterators for convenient usage.
// Users can import iterators via `use str_tools::*;`

pub mod str_lines;
pub mod str_matches;
pub mod str_rsplit;
pub mod str_splitn;
pub mod str_words;

pub use str_lines::*;
pub use str_matches::*;
pub use str_rsplit::*;
pub use str_splitn::*;
pub use str_words::*;
