use regex::Regex;
pub mod types;
use types::MalType;

pub struct Reader {
    tokens: Vec<String>,
    position: usize
}

impl Reader {
    fn next(&mut self) {
        self.position += 1;
    }

    fn peek(&self) -> Option<String> {
        match self.tokens.get(self.position) {
            Some(tok) => Some(tok.to_string()),
            None => None
        }
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
            let intval = token.trim().parse::<isize>();
            match intval {
                Ok(int) => MalType::Int(int),
                Err(e) => {
                    match &token.trim()[..] {
                        "nil" => MalType::Null,
                        _ => {
                            MalType::Symbol(token)
                        }
                    }
                }
            }
        }
        None => MalType::Null
    }
}

fn read_list(reader: &mut Reader, mut list: MalType) -> MalType {
    match reader.peek() {
        Some(n) => {
            if ")" == n.trim()  {
                list
            } else {
                list.push(read_form(reader));
                reader.next();

                read_list(reader, list)
            }
        },
        None => {
            println!("ERRRORRRR");
            list
        }
    }
}


fn read_form(reader: &mut Reader) -> MalType {
    match reader.peek() {
        Some(n) => {
            if "(" == n.trim()  {
                reader.next();
                let list = MalType::List { contents: vec!() };

                read_list(reader, list)
            } else {
                read_atom(&reader)
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
