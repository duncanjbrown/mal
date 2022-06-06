use crate::reader::types::MalType;
use crate::reader::types::MalSymbol;

pub fn pr_str(expr: &MalType) -> String {
    match expr {
        MalType::Int(value) => format!("{}", value),
        MalType::Symbol(MalSymbol::Add) => "+".to_string(),
        MalType::Symbol(MalSymbol::Sub) => "-".to_string(),
        MalType::Symbol(MalSymbol::Mult) => "*".to_string(),
        MalType::Symbol(MalSymbol::Div) => "/".to_string(),
        MalType::Null => "null".to_string(),
        MalType::List { contents } => {
            let printed_contents: Vec<_> = contents
                .into_iter()
                .map(|value| pr_str(value))
                .collect();

            format!("({})", printed_contents.join(" "))
        }
    }
}
