use super::tokenizer::{Token, Tokenizer};
use super::types::{MalAtom, MalList, MalType};
use std::iter::Peekable;

fn read_list(tokenizer: &mut Peekable<Tokenizer<'_>>) -> MalList {
    match tokenizer.next() {
        Some(token) if token != Token::LParen => {
            panic!("Error read_list: Called without beginning left parenthesis, found {:?} instead", token)
        }
        _ => {}
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

pub fn read_str(input: &str) {
    let tokenizer = Tokenizer::new(&input);
    read_form(&mut tokenizer.peekable());
}

pub fn read_form(tokenizer: &mut Peekable<Tokenizer<'_>>) -> Option<MalType> {
    let maybe_next = tokenizer.peek();

    match maybe_next {
        Some(Token::LParen) => Some(MalType::List(read_list(tokenizer))),
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
        },
        Some(_) => {
            MalAtom::Symbol(String::from(""))
        },
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
