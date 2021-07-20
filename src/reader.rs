use std::iter::Peekable;
use super::tokenizer::{Tokenizer, Token};
use super::types::{MalType, MalList, MalAtom, MalSymbol};

fn read_list(mut tokenizer: Peekable<Tokenizer<'_>>) -> MalList {
    // Throw away the lparen
    tokenizer.next();

    loop {
        match read_form(tokenizer) {
            MalType::Atom(mal_atom) => {
            },
            _ => {},
        }
        break;
    }
    MalList {}
}

pub fn read_str(input: &str) {
    let tokenizer = Tokenizer::new(&input);
    read_form(tokenizer.peekable());
}

pub fn read_form(mut tokenizer: Peekable<Tokenizer<'_>>) -> MalType {
    
    let maybe_next = tokenizer.peek();

    match maybe_next {
        Some(Token::LPAREN) => MalType::List(read_list(tokenizer)),
        Some(_) => MalType::Atom(read_atom(tokenizer)),
        None => MalType::Atom(MalAtom{}),
    }
}

fn read_symbol(mut _tokenizer: Peekable<Tokenizer<'_>>) -> MalSymbol {
    MalSymbol {}
}

fn read_atom(mut _tokenizer: Peekable<Tokenizer<'_>>) -> MalAtom {
    MalAtom {}
}
