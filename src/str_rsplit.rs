// src/str_rsplit.rs
// StrRSplit iterator
// Splits a string slice in reverse order by a specified delimiter without allocating.
// Each item returned is a &'a str referencing the original string.
// Usage: `StrRSpli::new(some_str, delimiter)`

pub struct StrRSplit<'a, 'b> {
    remainder: Option<&'a str>,
    delimiter: &'b str,
}

impl<'a, 'b> StrRSplit<'a, 'b> {
    pub fn new(haystack: &'a str, delimiter: &'b str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a, 'b> Iterator for StrRSplit<'a, 'b> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;

        // find delimeter from the right side
        if let Some(pos) = remainder.rfind(self.delimiter) {
            // slice from the right side of the delimiter
            let after_delimiter = &remainder[pos + self.delimiter.len()..];

            // update remainder to everything after the delimeter
            *remainder = &remainder[..pos];
            Some(after_delimiter)
        } else {
            // no delimeter found
            self.remainder.take()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_rsplit() {
        let text = "a,b,c";
        let mut splitter = StrRSplit::new(text, ",");

        assert_eq!(splitter.next(), Some("c"));
        assert_eq!(splitter.next(), Some("b"));
        assert_eq!(splitter.next(), Some("a"));
        assert_eq!(splitter.next(), None);
    }

    #[test]
    fn delimiter_not_found() {
        let text = "hello";
        let mut splitter = StrRSplit::new(text, ",");

        assert_eq!(splitter.next(), Some("hello"));
        assert_eq!(splitter.next(), None);
    }

    #[test]
    fn delimiter_at_start() {
        let text = ",a,b";
        let mut splitter = StrRSplit::new(text, ",");

        assert_eq!(splitter.next(), Some("b"));
        assert_eq!(splitter.next(), Some("a"));
        assert_eq!(splitter.next(), Some(""));
        assert_eq!(splitter.next(), None);
    }

    #[test]
    fn delimiter_at_end() {
        let text = "a,b,";
        let mut splitter = StrRSplit::new(text, ",");

        assert_eq!(splitter.next(), Some(""));
        assert_eq!(splitter.next(), Some("b"));
        assert_eq!(splitter.next(), Some("a"));
        assert_eq!(splitter.next(), None);
    }

    #[test]
    fn multi_char_delimiter() {
        let text = "a--b--c";
        let mut splitter = StrRSplit::new(text, "--");

        assert_eq!(splitter.next(), Some("c"));
        assert_eq!(splitter.next(), Some("b"));
        assert_eq!(splitter.next(), Some("a"));
        assert_eq!(splitter.next(), None);
    }

    #[test]
    fn collect_iterator() {
        let text = "x:y:z";
        let parts: Vec<&str> = StrRSplit::new(text, ":").collect();

        assert_eq!(parts, vec!["z", "y", "x"]);
    }
}
