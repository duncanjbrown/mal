use crate::types::MalType;

pub fn pr_str(expr: &MalType) -> String {
    match expr {
        MalType::Int(value) => format!("{}", value),
        MalType::Symbol(s) => s.to_string(),
        MalType::String(s) => format!("\"{}\"", s),
        MalType::Function(_) => "<# Function>".to_string(),
        MalType::BuiltIn(_) => "<# BuiltIn>".to_string(),
        MalType::True => "true".to_string(),
        MalType::False => "false".to_string(),
        MalType::Nil => "nil".to_string(),
        MalType::ParseError(s) => s.to_string(),
        MalType::List { contents } => {
            let printed_contents: Vec<_> = contents
                .iter()
                .map(pr_str)
                .collect();

            format!("({})", printed_contents.join(" "))
        },
        MalType::Vector { contents } => {
            let printed_contents: Vec<_> = contents
                .iter()
                .map(pr_str)
                .collect();

            format!("[{}]", printed_contents.join(" "))
        },
        MalType::HashMap { contents } => {
            let printed_contents: Vec<_> = contents
                .iter()
                .map(pr_str)
                .collect();

            format!("{{{}}}", printed_contents.join(" "))
        }
    }
}
