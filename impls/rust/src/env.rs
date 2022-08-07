use crate::types::MalType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Env<'a> {
    pub outer: Option<&'a Env<'a>>,
    symbols: HashMap<&'a str, MalType>
}

impl<'a> Env<'a> {
    pub fn new(outer: Option<&'a Env>, binds: Vec<&'a str>, exprs: Vec<MalType>) -> Self {
        let symbols: HashMap<&str, MalType> = HashMap::new();
        let bindings_kv = binds.iter().zip(exprs.into_iter());

        let mut new_env = match outer {
            Some(new_outer) => Self { outer: Some(new_outer), symbols },
            None => Self { outer: None, symbols }
        };

        bindings_kv.for_each(|(bind, expr)| new_env.set(bind, expr));

        new_env
    }

    pub fn get(&self, symbol: &str) -> Option<&MalType> {
        match self.find(symbol) {
            Some(env) => {
                let val = env.symbols.get(symbol);
                Some(val.unwrap())
            },
            None => None
        }
    }

    pub fn set(&mut self, symbol: &'a str, value: MalType) {
        self.symbols.insert(symbol, value);
    }

    pub fn find(&self, symbol: &str) -> Option<&Self> {
        match &self.symbols.get(symbol) {
            Some(_) => Some(self),
            None => {
                match &self.outer {
                    Some(env) => env.find(symbol),
                    None => None
                }
            }
        }
    }
}

