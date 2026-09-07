mod token;
mod scanner;

use std::env;

use token::Token;
use scanner::Scanner;

fn main() {
    let args: Vec<String> =  env::args().collect();
    let Some(filename) = args.get(1) else {
        println!("ERROR: filename to parse must be provided as first argument");
        return;
    };

    println!("Using filename {}", filename);

    let mut scanner;
    match Scanner::new(filename) {
        Ok(scanner_struct) => scanner = scanner_struct,
        Err(e) => {
            println!("ERROR: {}", e.to_string());
            return;
        }
    }

    println!("Next token is {:?}", scanner.current_token());

    while scanner.current_token() != Token::EOS && !matches!(scanner.current_token(), Token::ERROR { .. }) {
        // this is technically bad since debug printing isn't stable
        // but it'll do since it's really only needed for this assignment
        let token_str = format!("{:?}", scanner.current_token());
        println!("{}", token_str.replace('(', "[").replace(')', "]"));

        scanner.next_token();

        println!("Next token is {:?}", scanner.current_token());
    }

    if let Token::ERROR(e) = scanner.current_token() {
        println!("ERROR: {e}");
    }
}
