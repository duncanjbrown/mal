use std::io;
use std::io::Write;
use std::str;

use mal::types::MalType;
use mal::env::Env;
use mal::env::repl_env;
use mal::reader::read_str;
use mal::printer::pr_str;

fn eval_ast(ast: MalType, env: &Env) -> MalType {
    match ast {
        MalType::Vector { contents } => {
            match contents.len() {
                0 => MalType::Vector { contents: vec![] },
                _ => {
                    let evaluated_contents: Vec<MalType> = contents.iter().map(|item| eval_ast(item.clone(), env)).collect();
                    MalType::Vector { contents: evaluated_contents }
                }
            }
        },
        MalType::HashMap { contents } => {
            match contents.len() {
                0 => MalType::HashMap { contents: vec![] },
                _ => {
                    let evaluated_contents: Vec<MalType> = contents.iter().map(|item| eval_ast(item.clone(), env)).collect();
                    MalType::HashMap { contents: evaluated_contents }
                }
            }
        },
        MalType::List { contents } => {
            match contents.get(0) {
                Some(f) => {
                    let args = &contents[1..];
                    let evaluated_args: Vec<MalType> = args.iter().map(|item| eval_ast(item.clone(), env)).collect();

                    match f {
                        MalType::Symbol(sym) => {
                            match env.get(sym) {
                                Some(MalType::Function(f)) => f(evaluated_args.to_vec()),
                                _ => MalType::ParseError("i don't understand!".to_string())
                            }
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
    eval_ast(expr, &repl_env())
}

fn print(expr: &MalType) {
    println!("{}", pr_str(expr));
}
