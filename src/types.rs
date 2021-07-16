/// Make a List Types
pub enum MalType {
    List(MalList),
    Symbol(MalSymbol),
    Atom(MalAtom),
}

pub struct MalList;

pub struct MalSymbol;

pub struct MalAtom;
