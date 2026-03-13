// src/str_splitn.rs
// StrSplitN iterator
// Splits a string slice by a specified delimiter up to a maximum number of splits.
// Each item returned is a &'a str referencing the original string.
// Usage: `StrSplitN::new(some_str, delimiter, max_splits)`

// Defining the struct StrSplit that holds the state of out iterator
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

// Implementation block for strsplit
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack), // part of the string which is not processed yet
            delimiter,                 // substring we will split on
        }
    }
}

// Implement the iterator trait for the strsplit
impl<'a> Iterator for StrSplit<'a> {
    // each iteration returns a string slice borrowed from the original string
    type Item = &'a str;

    // next() returns the split part
    fn next(&mut self) -> Option<Self::Item> {
        // get mutable access to the remainder value if None , return None immediately
        let remainder = self.remainder.as_mut()?;

        // Trying to find the delimiter in the remainder string
        if let Some(pos) = remainder.find(self.delimiter) {
            // slice the string from the start to the delimiter
            let until_delimiter = &remainder[..pos];
            // calculate where the next substring should start
            let next_index = pos + self.delimiter.len();

            // update remainder to everything after the delimeter
            *remainder = &remainder[next_index..];
            Some(until_delimiter)
        } else {
            // if no delimeter is found return the entire remainder as the final element
            self.remainder.take()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_basic() {
        let text = "a,b,c";
        let mut splitter = StrSplit::new(text, ",");

        assert_eq!(splitter.next(), Some("a"));
        assert_eq!(splitter.next(), Some("b"));
        assert_eq!(splitter.next(), Some("c"));
        assert_eq!(splitter.next(), None);
    }

    #[test]
    fn delimiter_not_found() {
        let text = "hello";
        let mut splitter = StrSplit::new(text, ",");

        assert_eq!(splitter.next(), Some("hello"));
        assert_eq!(splitter.next(), None);
    }

    #[test]
    fn delimiter_at_start() {
        let text = ",a,b";
        let mut splitter = StrSplit::new(text, ",");

        assert_eq!(splitter.next(), Some(""));
        assert_eq!(splitter.next(), Some("a"));
        assert_eq!(splitter.next(), Some("b"));
        assert_eq!(splitter.next(), None);
    }

    #[test]
    fn delimiter_at_end() {
        let text = "a,b,";
        let mut splitter = StrSplit::new(text, ",");

        assert_eq!(splitter.next(), Some("a"));
        assert_eq!(splitter.next(), Some("b"));
        assert_eq!(splitter.next(), Some(""));
        assert_eq!(splitter.next(), None);
    }

    #[test]
    fn multi_char_delimiter() {
        let text = "a--b--c";
        let mut splitter = StrSplit::new(text, "--");

        assert_eq!(splitter.next(), Some("a"));
        assert_eq!(splitter.next(), Some("b"));
        assert_eq!(splitter.next(), Some("c"));
        assert_eq!(splitter.next(), None);
    }

    #[test]
    fn collect_iterator() {
        let text = "x:y:z";
        let parts: Vec<&str> = StrSplit::new(text, ":").collect();

        assert_eq!(parts, vec!["x", "y", "z"]);
    }
}
