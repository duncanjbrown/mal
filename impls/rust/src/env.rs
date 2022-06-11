use crate::types::MalType;
use std::collections::HashMap;

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

    pub fn set_outer(&mut self, env: &'a Env) {
        self.outer = Some(env)
    }

    pub fn call(&self, symbol: &str, args: Vec<MalType>) -> MalType {
        match self.symbols.get(symbol) {
            Some(MalType::Function(f)) => f(args),
            _ => MalType::ParseError(format!("Cannot call {} as a function", symbol))
        }
    }

    pub fn get(&self, symbol: &str) -> Option<&MalType> {
        match self.find(symbol) {
            Some(env) => {
                let val = env.symbols.get(symbol).unwrap();
                Some(val)
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

