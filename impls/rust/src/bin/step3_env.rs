use std::io;
use std::io::Write;
use std::str;

use mal::types::MalType;
use mal::env::Env;
use mal::reader::read_str;
use mal::printer::pr_str;

fn eval_ast(ast: MalType, env: &mut Env) -> MalType {
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
        MalType::Symbol(sym) => {
            match env.get(&sym) {
                Some(n) => n.clone(),
                None => MalType::Symbol(sym)
            }
        },
        MalType::List { contents } => {
            match contents.get(0) {
                Some(f) => {
                    let args = &contents[1..];

                    match f {
                        MalType::Symbol(sym) => {
                            match env.get(sym) {
                                Some(MalType::BuiltIn(f)) => f(env, args.to_vec()),
                                Some(MalType::Function(f)) => {
                                    let func = *f;
                                    let evaluated_args: Vec<MalType> = args.iter().map(|item| eval_ast(item.clone(), env)).collect();
                                    func(evaluated_args.to_vec())
                                }
                                None => MalType::ParseError(format!("{} not found", sym)),
                                value => value.unwrap().clone()
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

fn add(args: Vec<MalType>) -> MalType {
    MalType::Int(args.iter().fold(0, |acc, next|
        match next {
            MalType::Int(a) => acc + a,
            _ => panic!("Can't add non-integer {:?}", next)
        }
    ))
}

fn mult(args: Vec<MalType>) -> MalType {
    MalType::Int(args.iter().fold(1, |acc, next|
        match next {
            MalType::Int(a) => acc * a,
            _ => panic!("Can't multiply non-integer {:?}", next)
        }
    ))
}

fn div(args: Vec<MalType>) -> MalType {
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
}

fn sub(args: Vec<MalType>) -> MalType {
    match args.get(0) {
        Some(MalType::Int(arg1)) => {
            MalType::Int(
                args[1..].iter().fold(*arg1, |acc, next|
                match next {
                    MalType::Int(a) => acc - a,
                    _ => panic!("Can't subtract non-integer {:?}", next)
                }
            ))
        },
        Some(x) => panic!("Can’t subtract non-integer {:?}", x),
        None => panic!("Wrong number of args passed to \"-\"")
    }
}

fn def(env: &mut Env, args: Vec<MalType>) -> MalType {
    match &args[..] {
        [MalType::Symbol(sym), ast] => {
            set_value(env, sym, ast.clone())
        },
        _ => MalType::ParseError("Could not def".to_string())
    }
}

fn set_value(env: &mut Env, symbol: &str, ast: MalType) -> MalType {
    let new_val = eval_ast(ast, env);
    match new_val {
        MalType::ParseError(_) => { new_val },
        _ => {
            env.set(symbol.to_string(), new_val.clone());
            new_val
        }
    }
}

fn mal_let(env: &mut Env, args: Vec<MalType>) -> MalType {
    match &args[..] {
        [MalType::List { contents: bindings } | MalType::Vector { contents: bindings }, ast] => {
            if bindings.len() % 2 != 0 {
                MalType::ParseError("Uneven number of forms passed to let".to_string())
            } else {
                let mut new_env = Env::new(Some(env));
                let iter = bindings.chunks(2);

                let bound_env: &mut Env = iter.fold(&mut new_env, |e, chunk|
                        match chunk {
                            [MalType::Symbol(sym), let_expr] => {
                                set_value(e, sym, let_expr.clone());
                                e
                            },
                            _ => {
                                panic!("Can't let");
                            }
                        }
                );

                eval_ast(ast.clone(), bound_env)
            }
        }
        _ => MalType::ParseError("Could not let".to_string())
    }
}

pub fn repl_env() -> Env<'static> {
    let mut env = Env::new(None);

    env.set("+".to_string(), MalType::Function(add));
    env.set("-".to_string(), MalType::Function(sub));
    env.set("*".to_string(), MalType::Function(mult));
    env.set("/".to_string(), MalType::Function(div));

    env.set("def!".to_string(), MalType::BuiltIn(def));
    env.set("let*".to_string(), MalType::BuiltIn(mal_let));

    env
}

fn main() {
    let mut repl_env = repl_env();

    loop {
        print!("user> ");
        io::stdout().flush().expect("Could not flush to stdout");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                } else {
                    rep(input.trim_end(), &mut repl_env);
                }
            }
            Err(error) => println!("Input error! {}", error),
        }
    }
}

fn rep(line: &str, env: &mut Env) {
    print(&eval(read(line), env))
}

fn read(line: &str) -> MalType {
    read_str(line)
}

fn eval(expr: MalType, env: &mut Env) -> MalType {
    eval_ast(expr, env)
}

fn print(expr: &MalType) {
    println!("{}", pr_str(expr));
}
