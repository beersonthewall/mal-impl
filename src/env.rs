use std::collections::HashMap;

pub struct Env {
    symbol_table: HashMap<String, Box<dyn Fn()>>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            symbol_table: HashMap::new(),
        }
    }
}
