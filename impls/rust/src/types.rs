#[derive(Debug)]
#[derive(Clone)]
pub enum MalType {
    List { contents: Vec<MalType> },
    Vector { contents: Vec<MalType> },
    HashMap { contents: Vec<MalType> },
    Int(isize),
    Function(fn(Vec<MalType>) -> MalType),
    Symbol(String),
    String(String),
    True,
    False,
    ParseError(String),
    Null
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
