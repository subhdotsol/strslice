#![allow(dead_code, unused)]
// src/lib.rs
// Main library file for the strslice crate.
// This file declares all modules and re-exports the public iterators for convenient usage.
// Users can import iterators via `use strslice::*;`

pub mod str_lines;
pub mod str_matches;
pub mod str_rsplit;
pub mod str_splitn;
pub mod str_tokenize;
pub mod str_words;

pub use str_lines::*;
pub use str_matches::*;
pub use str_rsplit::*;
pub use str_splitn::*;
pub use str_tokenize::*;
pub use str_words::*;
