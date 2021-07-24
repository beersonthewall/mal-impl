#[derive(Debug, PartialEq)]
pub enum MalType {
    List(MalList),
    Atom(MalAtom),
}

#[derive(Debug, PartialEq)]
pub struct MalList {
    elements: Vec<MalType>,
}

impl MalList {
    pub fn new(elements: Vec<MalType>) -> MalList {
        MalList { elements }
    }
}

#[derive(Debug, PartialEq)]
pub enum MalAtom {
    Int(isize),
    Symbol(String),
}
