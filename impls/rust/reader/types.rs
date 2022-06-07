#[derive(Debug)]
pub enum MalType {
    List { contents: Vec<MalType> },
    Int(isize),
    Symbol(String),
    Null
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
