use super::tokenizer::{Token, Tokenizer};
use super::types::{MalList, MalType, MalMap};
use std::iter::Peekable;
use std::collections::HashMap;

fn read_list(tokenizer: &mut Peekable<Tokenizer<'_>>) -> MalList {
    let token = tokenizer.next();
    if !matches!(token, Some(Token::LParen)) {
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
        } else if matches!(tokenizer.peek(), None) {
            // read_form returning 'None' is an overloaded value. It means both no more forms
            // and we hit some token that it cannot handle (e.g. closing paren) but doesn't
            // want to panic!() on. For example consider the list ( {"a" 1} 2 3 4)
            // We only want to panic in the case where we're out of forms and out of tokens.
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

fn read_map(tokenizer: &mut Peekable<Tokenizer<'_>>) -> MalMap {
    let next = tokenizer.next();
    if !matches!(next, Some(Token::LCurly)) {
        panic!(
            "Error read_map: Called without beginning left curly brace, found {:?} instead.",
            next
        );
    }

    let mut map = HashMap::<String, MalType>::new();
    loop {
        let maybe_key = read_form(tokenizer);
        let mut key = None;

        if let Some(MalType::Symbol(value)) = maybe_key {
            key = Some(value);
        } else if let Some(MalType::Str(value)) = maybe_key {
            key = Some(value);
        } else if let None = maybe_key {
            // Possibly empty map.
        } else if matches!(tokenizer.peek(), None){
            panic!("Error read_map: Missing or invalid key value. {:?}", maybe_key);
        }

        let maybe_value = read_form(tokenizer);
        if key.is_some() && maybe_value.is_some() {
            map.insert(key.unwrap(), maybe_value.unwrap());
        } else if key.is_none() && maybe_value.is_none() {
            let next = tokenizer.next();
            if !matches!(next, Some(Token::RCurly)) {
                panic!("Error read_map: missing ending right curly brace, found {:?} instead.", next);
            }

            return MalMap::new(HashMap::new());
        } else {
            panic!("Error read_map: no valid key, value pair found.");
        }


        if let Some(Token::RCurly) = tokenizer.peek() {
            // End of the line
            return MalMap::new(map);
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
        Some(Token::RParen) => consume_and_forget(tokenizer),
        Some(Token::LCurly) => Some(MalType::Map(read_map(tokenizer))),
        Some(Token::RCurly) => consume_and_forget(tokenizer),
        Some(Token::NonSpecial(_)) => read_non_special(tokenizer),
        Some(Token::Str(_)) => read_string(tokenizer),
        Some(tkn) => panic!("Error read_form: unsupported token {:?}", tkn),
        None => None,
    }
}

fn consume_and_forget(tokenizer: &mut Peekable<Tokenizer<'_>>) -> Option<MalType> {
    tokenizer.next();
    None
}

fn read_non_special(tokenizer: &mut Peekable<Tokenizer<'_>>) -> Option<MalType> {
    if let Some(Token::NonSpecial(value)) = tokenizer.next() {
        if let Ok(number) = value.parse::<isize>() {
            return Some(MalType::Int(number));
        } else if value == "nil" {
            return Some(MalType::Nil);
        } else if value == "true" {
            return Some(MalType::True);
        } else if value == "false" {
            return Some(MalType::False);
        } else {
            return Some(MalType::Symbol(value.to_string()));
        }
    }
    None
}

fn read_string(tokenizer: &mut Peekable<Tokenizer<'_>>) -> Option<MalType> {
    if let Some(Token::Str(value)) = tokenizer.next() {
        return Some(MalType::Str(value));
    }
    None
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
