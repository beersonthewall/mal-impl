use std::fmt;

#[derive(Debug, PartialEq)]
pub enum MalType {
    List(MalList),
    Atom(MalAtom),
}

impl fmt::Display for MalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MalType::List(mal_list) => mal_list.fmt(f),
            MalType::Atom(mal_atom) => mal_atom.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MalList {
    elements: Vec<MalType>,
}

impl fmt::Display for MalList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( ")?;
        let mut itr = self.elements.iter();

        if let Some(elem) = itr.next() {
            elem.fmt(f)?;
        }

        for elem in itr {
            write!(f, ", ")?;
            elem.fmt(f)?;
        }
        
        write!(f, " )")
    }
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

impl fmt::Display for MalAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MalAtom::Int(integer) => write!(f, "{}", integer),
            MalAtom::Symbol(string) => write!(f, "{}", string),
        }
    }
}
