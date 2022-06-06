#[derive(Debug)]
pub enum MalType {
    List { contents: Vec<MalType> },
    Int(isize),
    Symbol(MalSymbol),
    Null
}

#[derive(Debug)]
pub enum MalSymbol {
    Add,
    Sub,
    Mult,
    Div,
}

impl MalType {
    pub fn push(&mut self, x: MalType) {
        match self {
            MalType::List { contents } => {
                contents.push(x);
            },
            _ => ()
        }
    }
}
