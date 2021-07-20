use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum Token {
    AT,
    CARAT,
    LBRACE,
    LCURLY,
    LPAREN,
    NONSPECIAL(String),
    QUASIQUOTE,
    QUOTE,
    RBRACE,
    RCURLY,
    RPAREN,
    SPLICEUNQUOTE,
    STR(String),
    TILDE,
}

pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
}
impl Tokenizer<'_> {
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer {
            input: input.chars().peekable(),
        }
    }

    pub fn get_all(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let maybe_tkn = self.next();
            if let Some(token) = maybe_tkn {
                tokens.push(token);
            } else {
                return tokens;
            }
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            let c = if let Some(c) = self.input.next() {
                c
            } else {
                return None;
            };
            match c {
                ' ' => continue,
                '\n' => continue,
                '\t' => continue,
                ',' => continue,
                '~' => {
                    if self.input.next_if(|&x| x == '@').is_some() {
                        return Some(Token::SPLICEUNQUOTE);
                    } else {
                        return Some(Token::TILDE);
                    }
                }
                '[' => return Some(Token::LBRACE),
                ']' => return Some(Token::RBRACE),
                '{' => return Some(Token::LCURLY),
                '}' => return Some(Token::RCURLY),
                '(' => return Some(Token::LPAREN),
                ')' => return Some(Token::RPAREN),
                '`' => return Some(Token::QUASIQUOTE),
                '\'' => return Some(Token::QUOTE),
                '^' => return Some(Token::CARAT),
                '@' => return Some(Token::AT),
                '"' => {
                    let mut data: Vec<char> = Vec::new();
                    data.push(c);
                    let mut maybe_val = self.input.next();
                    while maybe_val.is_some() {
                        let val = maybe_val.unwrap();
                        data.push(val);
                        // Reached end of string because previous character was not escaping the double quotes.
                        if val == '"' {
                            if data.len() > 0 && *data.last().unwrap() != '\\' {
                                return Some(Token::STR(data.into_iter().collect()));
                            }
                        }
                        maybe_val = self.input.next()
                    }
                    // TODO possibly implement Result return type instead of a panic?
                    panic!(
                        "Unbalanced String: {}",
                        data.into_iter().collect::<String>()
                    );
                }
                ';' => while self.input.next_if(|&x| x != '\n').is_some() {},
                _ => {
                    let mut data: Vec<char> = Vec::new();
                    data.push(c);
                    // We peek() instead of next() so that if the caracter is in the condition
                    // below we don't consume that character and let it be tokenized on the
                    // next loop.
                    let mut maybe_val = self.input.peek();
                    while maybe_val.is_some() {
                        let val = *maybe_val.unwrap();
                        if val == ' '
                            || val == '\n'
                            || val == '\t'
                            || val == '['
                            || val == ']'
                            || val == '{'
                            || val == '}'
                            || val == '('
                            || val == ')'
                            || val == '\''
                            || val == '"'
                            || val == '`'
                            || val == ','
                            || val == ';'
                        {
                            return Some(Token::NONSPECIAL(data.into_iter().collect()));
                        } else {
                            self.input.next();
                            data.push(val);
                        }

                        maybe_val = self.input.peek();
                    }
                    return Some(Token::NONSPECIAL(data.into_iter().collect()));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_whitespace() {
        let mut t = Tokenizer::new(" \t\n");
        assert!(matches!(t.next(), None));
        assert!(matches!(t.next(), None));
        assert!(matches!(t.next(), None));
    }

    #[test]
    fn ignores_comma() {
        let mut t = Tokenizer::new(",");
        assert!(matches!(t.next(), None));
        assert!(matches!(t.next(), None));
    }

    #[test]
    fn brace_like_things() {
        let mut t = Tokenizer::new("[]{}()");
        assert!(matches!(t.next(), Some(Token::LBRACE)));
        assert!(matches!(t.next(), Some(Token::RBRACE)));
        assert!(matches!(t.next(), Some(Token::LCURLY)));
        assert!(matches!(t.next(), Some(Token::RCURLY)));
        assert!(matches!(t.next(), Some(Token::LPAREN)));
        assert!(matches!(t.next(), Some(Token::RPAREN)));
        assert!(matches!(t.next(), None));
    }

    #[test]
    fn special_chars() {
        let mut t = Tokenizer::new("~@@~\'`");
        assert!(matches!(t.next(), Some(Token::SPLICEUNQUOTE)));
        assert!(matches!(t.next(), Some(Token::AT)));
        assert!(matches!(t.next(), Some(Token::TILDE)));
        assert!(matches!(t.next(), Some(Token::QUOTE)));
        assert!(matches!(t.next(), Some(Token::QUASIQUOTE)));
        assert!(matches!(t.next(), None));
    }

    #[test]
    fn ignores_comments() {
        let mut t = Tokenizer::new("; This is a comment");
        assert!(matches!(t.next(), None));
    }

    #[test]
    #[should_panic]
    fn unbalanced_string() {
        let input = "\"This string is lopsided.";
        let mut t = Tokenizer::new(&input);
        for _ in 0..input.len() {
            t.next();
        }
    }

    #[test]
    fn balanced_string() {
        let input = "\"VEEERY BALANCED DOUBLE QUOTES\"";
        let mut t = Tokenizer::new(&input);

        if let Some(Token::STR(data)) = t.next() {
            assert_eq!(input, data);
        } else {
            panic!();
        }
    }

    #[test]
    fn non_special() {
        let input = "true,false,nil variable_name\ta";
        let mut t = Tokenizer::new(&input);

        let expected = ["true", "false", "nil", "variable_name", "a"];
        for e in expected {
            let nxt = t.next();
            if let Some(Token::NONSPECIAL(data)) = nxt {
                assert_eq!(e, data);
            } else {
                panic!("Next token {:?} did not match expected form.", nxt);
            }
        }
    }

    #[test]
    fn list() {
        let input = "(hello world this is a list)";
        let mut t = Tokenizer::new(&input);

        assert!(matches!(t.next(), Some(Token::LPAREN)));
        let expected = ["hello", "world", "this", "is", "a", "list"];
        for e in expected {
            let nxt = t.next();
            if let Some(Token::NONSPECIAL(data)) = nxt {
                assert_eq!(e, data);
            } else {
                panic!("Next token {:?} did not match expected form.", nxt);
            }
        }
        assert!(matches!(t.next(), Some(Token::RPAREN)));
        assert!(matches!(t.next(), None));
    }
}
