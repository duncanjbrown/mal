use crate::types::MalType;

pub fn pr_str(expr: &MalType) -> String {
    match expr {
        MalType::Int(value) => format!("{}", value),
        MalType::Symbol(s) => format!("{}", s),
        MalType::String(s) => format!("\"{}\"", s),
        MalType::True => format!("true"),
        MalType::False => format!("false"),
        MalType::Null => "nil".to_string(),
        MalType::ParseError(s) => format!("{}", s),
        MalType::List { contents } => {
            let printed_contents: Vec<_> = contents
                .into_iter()
                .map(|value| pr_str(value))
                .collect();

            format!("({})", printed_contents.join(" "))
        },
        MalType::Vector { contents } => {
            let printed_contents: Vec<_> = contents
                .into_iter()
                .map(|value| pr_str(value))
                .collect();

            format!("[{}]", printed_contents.join(" "))
        },
        MalType::HashMap { contents } => {
            let printed_contents: Vec<_> = contents
                .into_iter()
                .map(|value| pr_str(value))
                .collect();

            format!("{{{}}}", printed_contents.join(" "))
        }
    }
}
