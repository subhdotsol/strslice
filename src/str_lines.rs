// src/str_lines.rs
// StrLines iterator
// Iterates over lines in a string slice without allocating.
// Each item returned is a &'a str referencing the original string.
// Usage: `StrLines::new(some_str)`

pub struct StrLines<'a> {
    remainder: Option<&'a str>,
}

impl<'a> StrLines<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { remainder: Some(s) }
    }
}

impl<'a> Iterator for StrLines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // checking if we have remaining text
        let remainder = self.remainder.as_mut()?;

        // find the position of the next newline character
        if let Some(pos) = remainder.find('\n') {
            // Determine end index of the line (handle \r\n)
            let end = if pos > 0 && remainder.as_bytes()[pos - 1] == b'\r' {
                pos - 1
            } else {
                pos
            };

            // split the line and update the remainder
            let line = &remainder[..end];
            *remainder = &remainder[pos + 1..];
            Some(line)
        } else {
            // No newline found, return the entire remainder
            self.remainder.take()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines_basic() {
        let text = "i\nlove\nbri";
        let mut lines = StrLines::new(text);

        assert_eq!(lines.next(), Some("i"));
        assert_eq!(lines.next(), Some("love"));
        assert_eq!(lines.next(), Some("bri"));
        assert_eq!(lines.next(), None);
    }

    fn test_lines_crlf() {
        let text = "i\nlove\nbri";
        let mut lines = StrLines::new(text);

        assert_eq!(lines.next(), Some("i"));
        assert_eq!(lines.next(), Some("love"));
        assert_eq!(lines.next(), Some("bri"));
        assert_eq!(lines.next(), None);
    }

    fn test_empty_string() {
        let text = "";
        let mut lines = StrLines::new(text);

        assert_eq!(lines.next(), Some(""));
        assert_eq!(lines.next(), None);
    }

    fn test_single_line_no_newline() {
        let text = "i love bri";
        let mut lines = StrLines::new(text);

        assert_eq!(lines.next(), Some("i love bri"));
        assert_eq!(lines.next(), None);
    }
}
