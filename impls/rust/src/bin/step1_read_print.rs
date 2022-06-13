use std::io;
use std::io::Write;
use std::str;

use mal::reader;
use mal::printer;
use mal::types::MalType;

fn main() {
    // reader::read_str
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
    print(eval(&read(line)))
}

fn read(line: &str) -> MalType {
    let expr = reader::read_str(&line);

    expr
}

fn eval(expr: &MalType) -> &MalType {
    expr
}

fn print(expr: &MalType) {
    println!("{}", printer::pr_str(expr));
}
