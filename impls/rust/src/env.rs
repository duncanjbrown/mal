use crate::types::MalType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Env<'a> {
    pub outer: Option<&'a Env<'a>>,
    symbols: HashMap<String, MalType>
}

impl<'a> Env<'a> {
    pub fn new(outer: Option<&'a Env>) -> Self {
        let symbols: HashMap<String, MalType> = HashMap::new();

        match outer {
            Some(new_outer) => Self { outer: Some(new_outer), symbols: symbols },
            None => Self { outer: None, symbols: symbols }
        }
    }

    pub fn get(&self, symbol: &str) -> Option<&MalType> {
        match self.find(symbol) {
            Some(env) => {
                let val = env.symbols.get(symbol);
                Some(&val.unwrap())
            },
            None => None
        }
    }

    pub fn set(&mut self, symbol: String, value: MalType) {
        self.symbols.insert(symbol, value);
    }

    pub fn find(&self, symbol: &str) -> Option<&Self> {
        match &self.symbols.get(symbol) {
            Some(_) => Some(&self),
            None => {
                match &self.outer {
                    Some(env) => env.find(symbol),
                    None => None
                }
            }
        }
    }
}

