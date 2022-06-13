use regex::Regex;
use crate::types::MalType;

pub struct Reader {
    tokens: Vec<String>,
    position: usize
}

impl Reader {
    fn next(&mut self) {
        self.position += 1;
    }

    fn peek(&self) -> Option<String> {
        self.tokens.get(self.position).map(|tok| tok.to_string())
    }
}

fn tokenize(str: &str) -> Vec<String> {
    let re = Regex::new(
        r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###
    ).unwrap();

    let matches = re
        .captures_iter(str.trim())
        .map(|c| String::from(&c[1]))
        .collect();

    matches
}

fn read_atom(reader: &Reader) -> MalType {
    match reader.peek() {
        Some(token) => {
            parse_token(token)
        }
        None => MalType::Null
    }
}

fn parse_token(token: String) -> MalType {
    if token == "nil" {
        return MalType::Null;
    }

    if token == "true" {
        return MalType::True;
    }

    if token == "false" {
        return MalType::False;
    }

    let intval = token.parse::<isize>();
    if let Ok(value) = intval {
        return MalType::Int(value);
    }

    if token.starts_with('\"') {
        if token.len() > 1 && token.ends_with('\"') {
            let string_literal = &token[1..token.len() - 1];
            MalType::String(string_literal.to_string())
        } else {
            MalType::ParseError("EOF when reading string".to_string())
        }
    } else {
        MalType::Symbol(token)
    }
}

fn read_list(reader: &mut Reader, bound: &str, mut list: MalType) -> Result<MalType, &'static str> {
    match reader.peek() {
        Some(token) => {
            if bound == token.trim()  {
                Ok(list)
            } else {
                list.push(read_form(reader));
                reader.next();

                read_list(reader, bound, list)
            }
        },
        None => {
            Err("EOF when reading list")
        }
    }
}


fn read_form(reader: &mut Reader) -> MalType {
    match reader.peek() {
        Some(token) => {
            let tok = token.trim();
            if "(" == tok {
                reader.next();
                let list = MalType::List { contents: vec!() };

                match read_list(reader, ")", list) {
                    Ok(list) => list,
                    Err(error) => MalType::ParseError(error.to_string())
                }
            } else if "[" == tok {
                reader.next();
                let list = MalType::Vector { contents: vec!() };

                match read_list(reader, "]", list) {
                    Ok(list) => list,
                    Err(error) => MalType::ParseError(error.to_string())
                }
            } else if "{" == tok {
                reader.next();
                let list = MalType::HashMap { contents: vec!() };

                match read_list(reader, "}", list) {
                    Ok(list) => list,
                    Err(error) => MalType::ParseError(error.to_string())
                }
            } else {
                read_atom(reader)
            }
        },
        None => MalType::Null
    }
}

pub fn read_str(str: &str) -> MalType {
    let mut reader = Reader {
        tokens: tokenize(str),
        position: 0
    };

    read_form(&mut reader)
}
