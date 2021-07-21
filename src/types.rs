/// Make a List Types
pub enum MalType {
    List(MalList),
    Symbol(MalSymbol),
    Atom(MalAtom),
}

pub struct MalList {
    elements: Vec<MalType>,
}
impl MalList {
    pub fn new(elements: Vec<MalType>) -> MalList {
        MalList { elements }
    }
}
pub struct MalSymbol;

pub struct MalAtom;
