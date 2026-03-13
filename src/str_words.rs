// src/str_words.rs
// StrWords iterator
// Iterates over words separated by whitespace in a string slice without allocating.
// Each item returned is a &'a str referencing the original string.
// Usage: `StrWords::new(some_str)`

struct StrWords<'a> {
    text: &'a str,
    cursor: usize,
}

impl<'a> StrWords<'a> {
    pub fn new(text: &'a str) -> Self {
        StrWords { text, cursor: 0 }
    }
}

impl<'a> Iterator for StrWords<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let rest = &self.text[self.cursor..];
        // remove initial whitespaces
        let trimmed = rest.trim_start();
        if trimmed.is_empty() {
            return None;
        }

        let space_skipped = rest.len() - trimmed.len();
        self.cursor += space_skipped;
        let word_end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());

        let word = &trimmed[..word_end];
        self.cursor += word_end;
        Some(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let words: Vec<&str> = StrWords::new("bri baby").collect();
        assert_eq!(words, vec!["bri", "baby"]);
    }

    #[test]
    fn test_empty_string() {
        let words: Vec<&str> = StrWords::new("").collect();
        assert_eq!(words, Vec::<&str>::new());
    }

    #[test]
    fn test_extra_spaces() {
        let words: Vec<&str> = StrWords::new("  bri   love  ").collect();
        assert_eq!(words, vec!["bri", "love"]);
    }

    #[test]
    fn test_single_word() {
        let words: Vec<&str> = StrWords::new("bri").collect();
        assert_eq!(words, vec!["bri"]);
    }
}
// saitama
