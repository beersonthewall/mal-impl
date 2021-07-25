use super::tokenizer::{Token, Tokenizer};
use super::types::{MalAtom, MalHashMap, MalList, MalType};
use std::iter::Peekable;
use std::collections::HashMap;

fn read_list(tokenizer: &mut Peekable<Tokenizer<'_>>) -> MalList {
    let token = tokenizer.next();
    if !matches!(Some(Token::LParen), token) {
        panic!(
            "Error read_list: Called without beginning left parenthesis, found {:?} instead",
            token
        )
    }

    let mut elements = Vec::<MalType>::new();
    loop {
        let maybe_form = read_form(tokenizer);

        if let Some(form) = maybe_form {
            elements.push(form);
        } else if let Some(Token::RParen) = tokenizer.peek() {
            tokenizer.next();
            return MalList::new(elements);
        } else {
            panic!(
                "Error read_list: missing end parenthesis for list {:?}",
                elements
            );
        }

        if let Some(Token::RParen) = tokenizer.peek() {
            tokenizer.next();
            return MalList::new(elements);
        }
    }
}

fn read_map(tokenizer: &mut Peekable<Tokenizer<'_>>) -> MalHashMap {
    let next = tokenizer.next();
    if !matches!(Some(Token::LCurly), next) {
        panic!(
            "Error read_map: Called without beginning left curly brace, found {:?} instead.",
            next
        );
    }

    let mut map = HashMap::<String, MalType>::new();
    loop {
        let maybe_key = read_form(tokenizer);
        let mut key = None;

        if let Some(MalType::Atom(MalAtom::Symbol(value))) = maybe_key {
            key = Some(value);
        } else if let Some(MalType::Atom(MalAtom::Str(value))) = maybe_key {
            key = Some(value);
        } else if let None = maybe_key {
            // Possibly empty map.
        } else {
            panic!("Error read_map: Missing or invalid key value. {:?}", maybe_key);
        }

        let maybe_value = read_form(tokenizer);
        if key.is_some() && maybe_value.is_some() {
            map.insert(key.unwrap(), maybe_value.unwrap());
        } else if key.is_none() && maybe_value.is_none() {
            let next = tokenizer.next();
            if !matches!(Some(Token::RCurly), next) {
                panic!("Error read_map: missing ending right curly brace, found {:?} instead.", next);
            }

            return MalHashMap::new(HashMap::new());
        } else {
            panic!("Error read_map: no valid key, value pair found.");
        }


        if let Some(Token::RCurly) = tokenizer.peek() {
            // End of the line
            return MalHashMap::new(map);
        }
    }
}

pub fn read_str(input: &str) -> Option<MalType> {
    let tokenizer = Tokenizer::new(&input);
    read_form(&mut tokenizer.peekable())
}

pub fn read_form(tokenizer: &mut Peekable<Tokenizer<'_>>) -> Option<MalType> {
    let maybe_next = tokenizer.peek();

    match maybe_next {
        Some(Token::LParen) => Some(MalType::List(read_list(tokenizer))),
        Some(Token::RParen) => None,
        Some(Token::LCurly) => Some(MalType::HashMap(read_map(tokenizer))),
        Some(_) => Some(MalType::Atom(read_atom(tokenizer))),
        None => None,
    }
}

fn read_atom(tokenizer: &mut Peekable<Tokenizer<'_>>) -> MalAtom {
    match tokenizer.next() {
        Some(Token::NonSpecial(value)) => {
            if let Ok(number) = value.parse::<isize>() {
                MalAtom::Int(number)
            } else {
                MalAtom::Symbol(value)
            }
        }
        Some(Token::Str(value)) => {
            if value == "nil" {
                MalAtom::Nil
            } else if value == "true" {
                MalAtom::True
            } else if value == "false" {
                MalAtom::False
            } else {
                MalAtom::Str(value)
            }
        }
        Some(token) => {
            panic!("read_atom called with unsupported token {:?}", token);
        }
        None => panic!("read_atom called with next token == none"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Error read_list: missing end parenthesis for list")]
    fn read_list_no_end_paren() {
        let input = String::from("(1, 2, 3, 4");
        read_str(&input);
    }

    #[test]
    #[should_panic(expected = "Error read_list: Called without beginning left parenthesis, found")]
    fn read_list_no_begin_paren() {
        let input = String::from("1, 2, 3, 4)");
        let tokenizer = Tokenizer::new(&input);
        read_list(&mut tokenizer.peekable());
    }
}
