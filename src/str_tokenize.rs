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

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Keyword(&'a str),
    Identifier(&'a str),
    Number(&'a str),
    Operator(&'a str),
    Punctuation(&'a str),
    StringLiteral(&'a str),
    Whitespace(&'a str),
    Unknown(&'a str),
}

pub struct Tokenizer<'a> {
    text: &'a str,
    cursor: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Tokenizer { text, cursor: 0 }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.text.len() {
            return None;
        }
        let rest = &self.text[self.cursor..];
        let mut chars = rest.char_indices();
        let (i, c) = chars.next().unwrap();
        if c.is_whitespace() {
            let mut end = i + c.len_utf8();
            for (j, ch) in chars {
                if !ch.is_whitespace() {
                    break;
                }
                end = j + ch.len_utf8();
            }
            let token_str = &rest[i..end];
            self.cursor += end;
            return Some(Token::Whitespace(token_str));
        }
        if c.is_alphabetic() || c == '_' {
            let mut end = i + c.len_utf8();
            for (j, ch) in chars {
                if !(ch.is_alphanumeric() || ch == '_') {
                    break;
                }
                end = j + ch.len_utf8();
            }
            let token_str = &rest[i..end];
            self.cursor += end;
            let is_keyword = matches!(
                token_str,
                "let"
                    | "fn"
                    | "if"
                    | "else"
                    | "return"
                    | "while"
                    | "for"
                    | "loop"
                    | "match"
                    | "mut"
                    | "struct"
                    | "enum"
                    | "pub"
                    | "use"
            );
            if is_keyword {
                return Some(Token::Keyword(token_str));
            } else {
                return Some(Token::Identifier(token_str));
            }
        }
        if c.is_ascii_digit() {
            let mut end = i + c.len_utf8();
            for (j, ch) in chars {
                if !ch.is_ascii_digit() {
                    break;
                }
                end = j + ch.len_utf8();
            }
            let token_str = &rest[i..end];
            self.cursor += end;
            return Some(Token::Number(token_str));
        }
        if c == '"' {
            let mut end = i + c.len_utf8();
            let mut closed = false;
            for (j, ch) in chars {
                end = j + ch.len_utf8();
                if ch == '"' {
                    closed = true;
                    break;
                }
            }
            let token_str = &rest[i..end];
            self.cursor += end;
            if closed {
                return Some(Token::StringLiteral(token_str));
            } else {
                return Some(Token::Unknown(token_str));
            }
        }
        if matches!(
            c,
            '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '!' | '&' | '|'
        ) {
            let mut end = i + c.len_utf8();
            if let Some((j, ch)) = chars.next() {
                if (c == '=' && ch == '=')
                    || (c == '<' && ch == '=')
                    || (c == '>' && ch == '=')
                    || (c == '!' && ch == '=')
                    || (c == '&' && ch == '&')
                    || (c == '|' && ch == '|')
                {
                    end = j + ch.len_utf8();
                }
            }
            let token_str = &rest[i..end];
            self.cursor += end;
            return Some(Token::Operator(token_str));
        }
        if matches!(c, '(' | ')' | '{' | '}' | '[' | ']' | ',' | ';' | ':' | '.') {
            let end = i + c.len_utf8();
            let token_str = &rest[i..end];
            self.cursor += end;
            return Some(Token::Punctuation(token_str));
        }
        let end = i + c.len_utf8();
        let token_str = &rest[i..end];
        self.cursor += end;
        Some(Token::Unknown(token_str))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let input = "let x = 42 + y;";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Keyword("let"),
                Token::Whitespace(" "),
                Token::Identifier("x"),
                Token::Whitespace(" "),
                Token::Operator("="),
                Token::Whitespace(" "),
                Token::Number("42"),
                Token::Whitespace(" "),
                Token::Operator("+"),
                Token::Whitespace(" "),
                Token::Identifier("y"),
                Token::Punctuation(";"),
            ]
        );
    }
    #[test]
    fn test_empty_string() {
        let tokens: Vec<Token> = Tokenizer::new("").collect();
        assert_eq!(tokens, Vec::<Token>::new());
    }
    #[test]
    fn test_single_word() {
        let tokens: Vec<Token> = Tokenizer::new("rust").collect();
        assert_eq!(tokens, vec![Token::Identifier("rust")]);
    }

    #[test]
    fn test_string_literal_and_operators() {
        let input = r#"x == "hello" || y"#;
        let tokens: Vec<Token> = Tokenizer::new(input).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("x"),
                Token::Whitespace(" "),
                Token::Operator("=="),
                Token::Whitespace(" "),
                Token::StringLiteral("\"hello\""),
                Token::Whitespace(" "),
                Token::Operator("||"),
                Token::Whitespace(" "),
                Token::Identifier("y"),
            ]
        );
    }

    #[test]
    fn test_keywords_and_punctuation() {
        let input = "fn main() { return 0; }";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Keyword("fn"),
                Token::Whitespace(" "),
                Token::Identifier("main"),
                Token::Punctuation("("),
                Token::Punctuation(")"),
                Token::Whitespace(" "),
                Token::Punctuation("{"),
                Token::Whitespace(" "),
                Token::Keyword("return"),
                Token::Whitespace(" "),
                Token::Number("0"),
                Token::Punctuation(";"),
                Token::Whitespace(" "),
                Token::Punctuation("}"),
            ]
        );
    }
}
