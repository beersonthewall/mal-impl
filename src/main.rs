extern crate rustyline;

use mal_impl::*;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        match rl.readline("=> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                print(eval(read(&line)));
            }
            Err(_) => break,
        }
    }
}
