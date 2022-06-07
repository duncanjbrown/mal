use crate::reader::types::MalType;

pub fn pr_str(expr: &MalType) -> String {
    match expr {
        MalType::Int(value) => format!("{}", value),
        MalType::Symbol(s) => format!("{}", s),
        MalType::Null => "nil".to_string(),
        MalType::List { contents } => {
            let printed_contents: Vec<_> = contents
                .into_iter()
                .map(|value| pr_str(value))
                .collect();

            format!("({})", printed_contents.join(" "))
        }
    }
}
