use std::fmt;
use crate::env::Env;
use std::fmt::Pointer;

#[derive(Clone)]
pub enum MalType {
    List { contents: Vec<MalType> },
    Vector { contents: Vec<MalType> },
    HashMap { contents: Vec<MalType> },
    Int(isize),
    Function(fn(Vec<MalType>) -> MalType),
    BuiltIn(fn(&mut Env, Vec<MalType>) -> MalType),
    Symbol(String),
    String(String),
    True,
    False,
    ParseError(String),
    Nil
}

impl fmt::Debug for MalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MalType::List { contents } => contents.fmt(f),
            MalType::Vector { contents } => contents.fmt(f),
            MalType::HashMap { contents } => contents.fmt(f),
            MalType::Int(x)  => x.fmt(f),
            MalType::Function(x)  => Pointer::fmt(&Box::new(x), f),
            MalType::BuiltIn(x)  => Pointer::fmt(&Box::new(x), f),
            MalType::Symbol(x)  => x.fmt(f),
            MalType::String(x)  => x.fmt(f),
            MalType::True  => true.fmt(f),
            MalType::False  => false.fmt(f),
            MalType::ParseError(x)  => x.fmt(f),
            MalType::Nil  => "nil".fmt(f)
        }
    }
}

impl MalType {
    pub fn push(&mut self, x: MalType) {
        match self {
            MalType::List { contents } => {
                contents.push(x);
            },
            MalType::Vector { contents } => {
                contents.push(x);
            },
            MalType::HashMap { contents } => {
                contents.push(x);
            },
            _ => ()
        }
    }
}
