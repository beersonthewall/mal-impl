use std::iter::Peekable;
use super::tokenizer::{ Tokenizer, Token};
use super::types::{ MalType, MalList, MalAtom };
use super::mal_error::MalError;

fn read_list(tokenizer: &mut Peekable<Tokenizer<'_>>) -> Result<MalList, MalError> {
    // Throw away the lparen
    tokenizer.next();
    let mut elements = Vec::<MalType>::new();
    loop {
        let maybe_form = read_form(tokenizer)?;
        
        if let Some(form) = maybe_form {
            elements.push(form);
        } else if let Some(Token::RPAREN) = tokenizer.peek() {
            // Toss right parenthesis
            tokenizer.next();
            return Ok(MalList::new(elements));
        } else {
            return Err(MalError::new(1, String::from("Unmatched parenthesis")));
        }

        if let Some(Token::RPAREN) = tokenizer.peek() {
            tokenizer.next();
            return Ok(MalList::new(elements));
        }
    }
}

pub fn read_str(input: &str) -> Result<MalType, MalError> {
    let tokenizer = Tokenizer::new(&input);
    let maybe_form = read_form(&mut tokenizer.peekable())?;

    if let Some(form) = maybe_form {
        return Ok(form);
    }

    Err(MalError::new(2, String::from("Empty")))
}

pub fn read_form(tokenizer: &mut Peekable<Tokenizer<'_>>) -> Result<Option<MalType>, MalError> {
    
    let maybe_next = tokenizer.peek();

    match maybe_next {
        Some(Token::LPAREN) => {
            let lst = read_list(tokenizer)?;
            Ok(Some(MalType::List(lst)))
        },
        Some(_) => {
            let atom = read_atom(tokenizer)?;
            Ok(Some(MalType::Atom(atom)))
        },
        None => Ok(None),
    }
}

fn read_atom(tokenizer: &mut Peekable<Tokenizer<'_>>) -> Result<MalAtom, MalError> {

    match tokenizer.next() {
        Some(Token::NONSPECIAL(_)) => {
            
        },
        Some(_) => {},
        None => {},
    }
    Ok(MalAtom {})
}


#[cfg(test)]
mod tests {
}
