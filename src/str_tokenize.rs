// src/tokenizer.rs
// Tokenizer iterator
// Splits a string slice into tokens without allocating new strings.
// Each token is represented by the `Token<'a>` enum, which may be a keyword,
// identifier, number, operator, or other syntactic unit.
// The iterator returns references to the original string slice (&'a str) for zero-copy performance.
// Usage example:
// let mut tokenizer = Tokenizer::new("let x = 42 + y");
// while let Some(token) = tokenizer.next() {
//     println!("{:?}", token);
// }
