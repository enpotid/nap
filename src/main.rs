mod ast;
mod lexer;
mod parser;

use std::{env, fs, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        todo!(); // invalid arguments
    }

    let mut contents = String::new();
    match fs::File::open(&args[1]) {
        Ok(mut file) => {
            file.read_to_string(&mut contents).unwrap(); // edit unwrap later
        }
        Err(_) => {
            todo!() // file open failed
        }
    }

    let tokens = lexer::tokenize(&contents);
    let computer = parser::parse(&tokens);

    println!("{:?}", computer);
}
