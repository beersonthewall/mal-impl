extern crate rustyline;

mod reader;
mod tokenizer;
mod types;
mod mal_error;

use reader::read_str;

pub fn read(input: &str) -> &str {
    read_str(&input);
    input
}

fn eval(input: &str) -> &str {
    return input;
}

fn print(input: &str) -> &str {
    return input;
}

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        match rl.readline("user> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("{}", print(eval(read(&line))));
            }
            Err(_) => break,
        }
    }
}

