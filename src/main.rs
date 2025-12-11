mod ast;
mod lexer;

use std::{env, fs, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("example: nap main.nap");
        return;
    }

    let mut contents = String::new();
    if let Ok(mut file) = fs::File::open(&args[1]) {
        file.read_to_string(&mut contents).unwrap(); // edit unwrap later
    } else {
        println!("enter the correct path");
        return;
    }

    contents = contents.to_uppercase();
    lexer::tokenize();

    println!("{}", contents);
}
