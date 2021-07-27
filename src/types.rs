use std::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum MalType {
    List(MalList),
    Map(MalMap),
    Int(isize),
    Symbol(String),
    Str(String),
    Nil,
    False,
    True,
}

impl fmt::Display for MalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MalType::List(mal_list) => mal_list.fmt(f),
            MalType::Map(mal_map) => mal_map.fmt(f),
            MalType::Int(value) => write!(f, "{}", value),
            MalType::Symbol(value) => write!(f, "{}", value),
            MalType::Str(value) => write!(f, "{}", value),
            MalType::Nil => write!(f, "nil"),
            MalType::False => write!(f, "false"),
            MalType::True => write!(f, "true"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MalMap {
    map: HashMap<String, MalType>,
}

impl MalMap {
    pub fn new(map: HashMap<String, MalType>) -> MalMap {
        MalMap { map }
    }
}

impl fmt::Display for MalMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for (k, v) in self.map.iter() {
            write!(f, "{} ", k)?;
            v.fmt(f)?;
            write!(f, " ")?;
        }
        write!(f, "}}")
    }
}

#[derive(Debug, PartialEq)]
pub struct MalList {
    pub elements: Vec<MalType>,
}

impl fmt::Display for MalList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        if self.elements.is_empty() {
            return write!(f, "()");
        }

        write!(f, "(")?;
        let mut itr = self.elements.iter();

        if let Some(elem) = itr.next() {
            elem.fmt(f)?;
        }

        for elem in itr {
            write!(f, ",")?;
            elem.fmt(f)?;
        }
        
        write!(f, ")")
    }
}

impl MalList {
    pub fn new(elements: Vec<MalType>) -> MalList {
        MalList { elements }
    }
}
