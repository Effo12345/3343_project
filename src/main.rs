mod token;
mod scanner;

use std::env;

use token::Token;
use scanner::Scanner;

fn main() {
    let Some(filename) = env::args().nth(1) else {
        println!("ERROR: filename to parse must be provided as first argument");
        return;
    };

    let mut scanner = match Scanner::new(&filename) {
        Ok(scanner_struct) => scanner_struct,
        Err(e) => {
            println!("ERROR: {}: {filename}", e.to_string());
            return;
        }
    };

    while scanner.current_token() != Token::EOS && !matches!(scanner.current_token(), Token::ERROR { .. }) {
        // this is technically bad since debug printing isn't stable
        // but it'll do since it's really only needed for this assignment
        // and doing it correctly with fmt would be a pain
        let token_str = format!("{:?}", scanner.current_token());

        // this is also comically inefficient but ¯\_(ツ)_/¯
        let token_str: String = token_str.replacen('(', "[", 1).chars().rev().collect();
        let token_str: String = token_str.replacen(')', "]", 1).chars().rev().collect();
        let token_str = token_str.replace('"', "");
        println!("{}", token_str);

        scanner.next_token();
    }

    if let Token::ERROR(e) = scanner.current_token() {
        println!("ERROR: {e}");
    }
}
