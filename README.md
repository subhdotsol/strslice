# strslice

`strslice` is a Rust library that provides zero copy string iterators for working with string slices. The library offers iterators similar to standard Rust string methods such as `lines`, `split_whitespace`, and `split`, but returns references to the original string without allocating new memory. This makes it efficient for large strings or performance critical applications.  

## Features

* StrLines – iterate over lines in a string  
* StrWords – iterate over words separated by whitespace  
* StrSplit – split a string by a delimiter  
* StrRSplit – split a string in reverse order  
* StrMatches – iterate over all occurrences of a substring  
* Tokenizer – iterate over tokens in a string slice  

All iterators are zero copy and use lifetimes to reference the original string safely.  

## Installation

To use `strslice` locally, add the path to your `Cargo.toml` dependencies:

```toml
[dependencies]
strslice = { path = "../str_tools" }
```

## Usage

```rust
use strslice::*;

fn main() {
    let text = "hello world\nRust is efficient\nzero copy iterators";
    for line in StrLines::new(text) {
        println!("{}", line);
    }

    let sentence = "Rust is fast and safe";
    for word in StrWords::new(sentence) {
        println!("{}", word);
    }

    let s = "a,b,c,d,e";
    let mut iter = StrSplit::new(s, ",");
    for part in iter {
        println!("{}", part);
    }

    let path = "a/b/c/d";
    for part in StrRSplit::new(path, "/") {
        println!("{}", part);
    }

    let text = "banana";
    for m in StrMatches::new(text, "an") {
        println!("{}", m);
    }

    let code = "let x = 42 + y;";
    for token in Tokenizer::new(code) {
        println!("{:?}", token);
    }
}
```

## Testing

```bash
cargo test 
```
