use std::io;
use std::io::Write;
use std::str;

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
    print(eval(read(line)))
}

fn read(line: &str) -> &str {
    line
}

fn eval(line: &str) -> &str {
    line
}

fn print(line: &str) {
    println!("{}", line);
}
