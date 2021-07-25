extern crate rustyline;

mod reader;
mod tokenizer;
mod types;

use reader::read_str;
use types::MalType;

pub fn read(input: &str) -> Option<MalType> {
    read_str(&input)
}

fn eval(input: Option<MalType>) -> Option<MalType> {
    input
}

fn print(input: Option<MalType>) {
    if let Some(mal_type) = input {
        println!("{}", mal_type);
    }
}

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        match rl.readline("user> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                print(eval(read(&line)));
            }
            Err(_) => break,
        }
    }
}
