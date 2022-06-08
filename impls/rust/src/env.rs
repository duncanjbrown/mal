use crate::types::MalType;
use std::collections::HashMap;

pub struct Env<'a> {
    outer: Option<&'a Env<'a>>,
    symbols: HashMap<String, MalType>
}

impl Env<'_> {
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

    pub fn set(mut self, symbol: String, value: MalType) -> Self {
        self.symbols.insert(symbol, value);

        self
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

fn add(args: Vec<MalType>) -> MalType {
    MalType::Int(args.iter().fold(0, |acc, next|
        match next {
            MalType::Int(a) => acc + a,
            _ => panic!("Can't add non-integer {:?}", next)
        }
    ))
}

fn mult(args: Vec<MalType>) -> MalType {
    MalType::Int(args.iter().fold(1, |acc, next|
        match next {
            MalType::Int(a) => acc * a,
            _ => panic!("Can't multiply non-integer {:?}", next)
        }
    ))
}

fn div(args: Vec<MalType>) -> MalType {
    match args.get(0) {
        Some(MalType::Int(arg1)) => {
            MalType::Int(
                args[1..].iter().fold(*arg1, |acc, next|
                match next {
                    MalType::Int(a) => acc / a,
                    _ => panic!("Can't divide non-integer {:?}", next)
                }
            ))
        },
        Some(x) => panic!("Can’t divide non-integer {:?}", x),
        None => panic!("Wrong number of args passed to \"/\"")
    }
}

fn sub(args: Vec<MalType>) -> MalType {
    match args.get(0) {
        Some(MalType::Int(arg1)) => {
            MalType::Int(
                args[1..].iter().fold(*arg1, |acc, next|
                match next {
                    MalType::Int(a) => acc - a,
                    _ => panic!("Can't subtract non-integer {:?}", next)
                }
            ))
        },
        Some(x) => panic!("Can’t subtract non-integer {:?}", x),
        None => panic!("Wrong number of args passed to \"-\"")
    }
}

pub fn repl_env() -> Env<'static> {
    let mut symbols: HashMap<String, MalType> = HashMap::new();

    symbols.insert("+".to_string(), MalType::Function(add));
    symbols.insert("-".to_string(), MalType::Function(sub));
    symbols.insert("*".to_string(), MalType::Function(mult));
    symbols.insert("/".to_string(), MalType::Function(div));

    Env { symbols: symbols, outer: None }
}
