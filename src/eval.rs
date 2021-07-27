use std::collections::HashMap;
use super::types::{ MalType, MalList, MalMap };
use super::env::Env;

pub fn eval_ast(ast: MalType, env: &mut Env) -> MalType {
    match ast {
        MalType::List(list_val) => eval_list(list_val, env),
        MalType::Map(map_val) => eval_map(map_val, env),
        MalType::Int(_) | MalType::Str(_) => ast,
        _ => panic!("Error eval_ast: unspported type {:?}", ast),
    }
}

fn eval_list(list: MalList, env: &mut Env) -> MalType {
    let mut elements = Vec::new();
    for elem in list.elements.into_iter() {
        elements.push(eval_ast(elem, env));
    }

    return MalType::List(MalList::new(elements));
}

fn eval_map(map: MalMap, env: &mut Env) -> MalType {
    let mut mappings = HashMap::<String, MalType>::new();
    for (key, value) in map.mappings.into_iter() {
        let new_value = eval_ast(value, env);
        mappings.insert(key.to_string(), new_value);
    }

    return MalType::Map(MalMap::new(mappings));
}
