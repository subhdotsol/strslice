// src/str_matches.rs
// StrMatches iterator
// Iterates over all occurrences of a substring pattern in a string slice without allocating.
// Each item returned is a &'a str referencing the matched part of the original string.
// Usage: `StrMatches::new(some_str, pattern)`

pub struct StrMatches<'a, 'b> {
    text: &'a str,
    pattern: &'b str,
    cursor: usize,
}

impl<'a, 'b> StrMatches<'a, 'b> {
    pub fn new(text: &'a str, pattern: &'b str) -> Self {
        Self {
            text,
            pattern,
            cursor: 0,
        }
    }
}

impl<'a, 'b> Iterator for StrMatches<'a, 'b> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.text.len() {
            return None;
        }
        //finding the pattern
        match self.text[self.cursor..].find(self.pattern) {
            Some(val) => {
                let match_start = self.cursor + val;
                let match_end = match_start + self.pattern.len();

                self.cursor = match_end;
                return Some(&self.text[match_start..match_end]);
            }
            None => {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let words: Vec<&str> = StrMatches::new("bri baby", "baby").collect();
        assert_eq!(words, vec!["baby"]);
    }

    #[test]
    fn test_empty_string() {
        let words: Vec<&str> = StrMatches::new("", "").collect();
        assert_eq!(words, Vec::<&str>::new());
    }

    #[test]
    fn test_extra_spaces() {
        let words: Vec<&str> = StrMatches::new("  bri   love  ", "love").collect();
        assert_eq!(words, vec!["love"]);
    }

    #[test]
    fn test_single_word() {
        let words: Vec<&str> = StrMatches::new("bri", "i").collect();
        assert_eq!(words, vec!["i"]);
    }

    #[test]
    fn test_multiple_occurrences() {
        let words: Vec<&str> = StrMatches::new("hello hello hello", "hello").collect();
        assert_eq!(words, vec!["hello", "hello", "hello"]);
    }

    #[test]
    fn test_pattern_not_found() {
        let words: Vec<&str> = StrMatches::new("hello world", "xyz").collect();
        assert_eq!(words, Vec::<&str>::new());
    }

    #[test]
    fn test_pattern_at_start() {
        let words: Vec<&str> = StrMatches::new("rustisgreat", "rust").collect();
        assert_eq!(words, vec!["rust"]);
    }

    #[test]
    fn test_pattern_at_end() {
        let words: Vec<&str> = StrMatches::new("ilovebri", "bri").collect();
        assert_eq!(words, vec!["bri"]);
    }

    #[test]
    fn test_pattern_longer_than_text() {
        let words: Vec<&str> = StrMatches::new("hi", "hello").collect();
        assert_eq!(words, Vec::<&str>::new());
    }

    #[test]
    fn test_pattern_equals_text() {
        let words: Vec<&str> = StrMatches::new("hello", "hello").collect();
        assert_eq!(words, vec!["hello"]);
    }

    #[test]
    fn test_adjacent_matches() {
        let words: Vec<&str> = StrMatches::new("abababab", "ab").collect();
        assert_eq!(words, vec!["ab", "ab", "ab", "ab"]);
    }
}
