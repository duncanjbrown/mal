use std::io;
use std::io::Write;
use std::str;
use std::collections::HashMap;
use mal;
use mal::types::MalType;
use mal::reader::read_str;
use mal::printer::pr_str;

struct ReplEnv {
    symbols: HashMap<&'static str, fn(Vec<MalType>) -> MalType>
}

impl ReplEnv {
    fn call(self, symbol: &str, args: Vec<MalType>) -> MalType {
        match self.symbols.get(symbol) {
            Some(sym) => sym(args),
            None => MalType::ParseError(format!("Cannot call {} as a function", symbol))
        }
    }
}

fn eval_ast(ast: MalType) -> MalType {
    let mut symbols: HashMap<_, fn(Vec<MalType>) -> MalType> = HashMap::new();

    // println!("{:?}", ast);

    symbols.insert("+", |args|
        MalType::Int(args.iter().fold(0, |acc, next|
            match next {
                MalType::Int(a) => acc + a,
                _ => panic!("Can't add non-integer {:?}", next)
            }
        ))
    );

    symbols.insert("-", |args|
        match args.get(0) {
            Some(MalType::Int(arg1)) => {
                MalType::Int(
                    args[1..].iter().fold(*arg1, |acc, next|
                    match next {
                        MalType::Int(a) => acc - a,
                        _ => panic!("Can't add non-integer {:?}", next)
                    }
                ))
            },
            Some(x) => panic!("Can’t subtract non-integer {:?}", x),
            None => panic!("Wrong number of args passed to \"-\"")
        }
    );

    symbols.insert("*", |args|
        MalType::Int(args.iter().fold(1, |acc, next|
            match next {
                MalType::Int(a) => acc * a,
                _ => panic!("Can't multiply non-integer {:?}", next)
            }
        ))
    );

    symbols.insert("/", |args|
        match args.get(0) {
            Some(MalType::Int(arg1)) => {
                MalType::Int(
                    args[1..].iter().fold(*arg1, |acc, next|
                    match next {
                        MalType::Int(a) => acc / a,
                        _ => panic!("Can't divide non-integer {:?}", next)
                    }
                ))
            },
            Some(x) => panic!("Can’t divide non-integer {:?}", x),
            None => panic!("Wrong number of args passed to \"/\"")
        }
    );

    let env = ReplEnv { symbols: symbols };

    match ast {
        MalType::Vector { contents } => {
            match contents.len() {
                0 => MalType::Vector { contents: vec![] },
                _ => {
                    let evaluated_contents: Vec<MalType> = contents.iter().map(|item| eval_ast(item.clone())).collect();
                    MalType::Vector { contents: evaluated_contents }
                }
            }
        },
        MalType::HashMap { contents } => {
            match contents.len() {
                0 => MalType::HashMap { contents: vec![] },
                _ => {
                    let evaluated_contents: Vec<MalType> = contents.iter().map(|item| eval_ast(item.clone())).collect();
                    MalType::HashMap { contents: evaluated_contents }
                }
            }
        },
        MalType::List { contents } => {
            match contents.get(0) {
                Some(f) => {
                    let args = &contents[1..];
                    let evaluated_args: Vec<MalType> = args.iter().map(|item| eval_ast(item.clone())).collect();

                    match f {
                        MalType::Symbol(sym) => {
                            env.call(sym, evaluated_args.to_vec())
                        },
                        MalType::String(val) => { MalType::ParseError(format!("Cannot call {} as a function", val)) },
                        MalType::Int(val) => { MalType::ParseError(format!("Cannot call {} as a function", val)) },
                        _ => {
                            MalType::ParseError("Syntax error".to_string())
                        }
                    }
                },
                None => MalType::List { contents: vec![] }
            }
        },
        _ => ast
    }
}

fn main() {
    loop {
        print!("user> ");
        io::stdout().flush().expect("Could not flush to stdout");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                } else {
                    rep(&input.trim_end());
                }
            }
            Err(error) => println!("Input error! {}", error),
        }
    }
}

fn rep(line: &str) {
    print(&eval(read(line)))
}

fn read(line: &str) -> MalType {
    let expr = read_str(&line);

    expr
}

fn eval(expr: MalType) -> MalType {
    eval_ast(expr)
}

fn print(expr: &MalType) {
    println!("{}", pr_str(expr));
}
