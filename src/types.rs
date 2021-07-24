#[derive(Debug)]
pub enum MalType {
    List(MalList),
    Symbol(MalSymbol),
    Atom(MalAtom),
}

#[derive(Debug)]
pub struct MalList {
    elements: Vec<MalType>,
}

impl MalList {
    pub fn new(elements: Vec<MalType>) -> MalList {
        MalList { elements }
    }
}

#[derive(Debug)]
pub struct MalSymbol;

#[derive(Debug)]
pub struct MalAtom;
