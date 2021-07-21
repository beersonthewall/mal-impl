use std::iter::Peekable;
use super::tokenizer::{Tokenizer, Token};
use super::types::{MalType, MalList, MalAtom };

fn read_list(tokenizer: &mut Peekable<Tokenizer<'_>>) -> MalList {
    // Throw away the lparen
    tokenizer.next();
    let mut elements = Vec::<MalType>::new();
    loop {
        let maybe_form = read_form(tokenizer);
        
        if let Some(form) = maybe_form {
            elements.push(form);
        } else if let Some(Token::RPAREN) = tokenizer.peek() {
            // Toss right parenthesis
            tokenizer.next();
            return MalList::new(elements);
        } else {
            panic!("Error missing right parenthesis");
        }

        if let Some(Token::RPAREN) = tokenizer.peek() {
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
        Some(Token::LPAREN) => Some(MalType::List(read_list(tokenizer))),
        Some(_) => Some(MalType::Atom(read_atom(tokenizer))),
        None => None,
    }
}

fn read_atom(tokenizer: &mut Peekable<Tokenizer<'_>>) -> MalAtom {

    match tokenizer.next() {
        Some(Token::NONSPECIAL(_)) => {
            
        },
        Some(_) => {},
        None => {},
    }
    MalAtom {}
}


#[cfg(test)]
mod tests {
}
