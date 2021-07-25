use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    At,
    Carat,
    LBrace,
    LCurly,
    LParen,
    NonSpecial(String),
    Quasiquote,
    Quote,
    RBrace,
    RCurly,
    RParen,
    SpliceUnquote,
    Str(String),
    Tilde,
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
                ' ' | '\n' | '\t' | ',' => continue,
                '~' => {
                    if self.input.next_if(|&x| x == '@').is_some() {
                        return Some(Token::SpliceUnquote);
                    } else {
                        return Some(Token::Tilde);
                    }
                }
                '[' => return Some(Token::LBrace),
                ']' => return Some(Token::RBrace),
                '{' => return Some(Token::LCurly),
                '}' => return Some(Token::RCurly),
                '(' => return Some(Token::LParen),
                ')' => return Some(Token::RParen),
                '`' => return Some(Token::Quasiquote),
                '\'' => return Some(Token::Quote),
                '^' => return Some(Token::Carat),
                '@' => return Some(Token::At),
                '"' => {
                    let mut data: Vec<char> = Vec::new();
                    data.push(c);
                    let mut maybe_val = self.input.next();
                    while maybe_val.is_some() {
                        let val = maybe_val.unwrap();
                        data.push(val);
                        // Reached end of string because previous character was not escaping the double quotes.
                        if val == '"' {
                            if data.len() > 0 && data[data.len() - 2] != '\\' {
                                return Some(Token::Str(data.into_iter().collect()));
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
                            return Some(Token::NonSpecial(data.into_iter().collect()));
                        } else {
                            self.input.next();
                            data.push(val);
                        }

                        maybe_val = self.input.peek();
                    }
                    return Some(Token::NonSpecial(data.into_iter().collect()));
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
        assert!(matches!(t.next(), Some(Token::LBrace)));
        assert!(matches!(t.next(), Some(Token::RBrace)));
        assert!(matches!(t.next(), Some(Token::LCurly)));
        assert!(matches!(t.next(), Some(Token::RCurly)));
        assert!(matches!(t.next(), Some(Token::LParen)));
        assert!(matches!(t.next(), Some(Token::RParen)));
        assert!(matches!(t.next(), None));
    }

    #[test]
    fn special_chars() {
        let mut t = Tokenizer::new("~@@~\'`");
        assert!(matches!(t.next(), Some(Token::SpliceUnquote)));
        assert!(matches!(t.next(), Some(Token::At)));
        assert!(matches!(t.next(), Some(Token::Tilde)));
        assert!(matches!(t.next(), Some(Token::Quote)));
        assert!(matches!(t.next(), Some(Token::Quasiquote)));
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

        if let Some(Token::Str(data)) = t.next() {
            assert_eq!(input, data);
        } else {
            panic!();
        }
    }

    #[test]
    fn non_special() {
        let input = "true,false,nil variable_name\ta 1234 -123";
        let mut t = Tokenizer::new(&input);

        let expected = ["true", "false", "nil", "variable_name", "a", "1234", "-123"];
        for e in expected {
            let nxt = t.next();
            if let Some(Token::NonSpecial(data)) = nxt {
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

        assert!(matches!(t.next(), Some(Token::LParen)));
        let expected = ["hello", "world", "this", "is", "a", "list"];
        for e in expected {
            let nxt = t.next();
            if let Some(Token::NonSpecial(data)) = nxt {
                assert_eq!(e, data);
            } else {
                panic!("Next token {:?} did not match expected form.", nxt);
            }
        }
        assert!(matches!(t.next(), Some(Token::RParen)));
        assert!(matches!(t.next(), None));
    }

    #[test]
    fn commas_as_whitespace() {
        let input = "(1 2, 3,,,,),,";
        let mut t = Tokenizer::new(&input);
        assert!(matches!(t.next(), Some(Token::LParen)));
        let expected = ["1", "2", "3"];
        for e in expected {
            let nxt = t.next();
            if let Some(Token::NonSpecial(data)) = nxt {
                assert_eq!(e, data);
            } else {
                panic!("Next token {:?} did not match expected form.", nxt);
            }
        }
        assert!(matches!(t.next(), Some(Token::RParen)));
        assert!(matches!(t.next(), None));
    }

    #[test]
    fn hash_maps() {
        let input = "{ \"a\" 1 }";
        let mut t = Tokenizer::new(&input);
        assert!(matches!(t.next(), Some(Token::LCurly)));
        assert!(matches!(t.next(), Some(Token::Str(_))));
        assert!(matches!(t.next(), Some(Token::NonSpecial(_))));
        assert!(matches!(t.next(), Some(Token::RCurly)));
        assert!(matches!(t.next(), None));
    }
}
