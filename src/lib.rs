mod reader;
mod tokenizer;
mod types;
mod eval;
mod env;

use env::Env;
use eval::eval_ast;
use reader::read_str;
use types::MalType;

pub fn read(input: &str) -> Option<MalType> {
    read_str(&input)
}

pub fn eval(input: Option<MalType>) -> Option<MalType> {
    if let Some(ast) = input {
        return Some(eval_ast(ast, &mut Env::new()));
    }
    None
}

pub fn print(input: Option<MalType>) {
    if let Some(mal_type) = input {
        println!("{}", mal_type);
    }
}
