use super::tokenizer::Tokenizer;
use super::tokenizer::Token;

pub fn read(input: &str) -> &str {
    let mut tokenizer = Tokenizer::new(&input);
    let _tokens = tokenizer.get_all();
    input
}
