mod token;
mod scanner;

use scanner::Scanner;

fn main() {
    let mut scanner;
    match Scanner::new("Correct/0.code") {
        Ok(scanner_struct) => scanner = scanner_struct,
        Err(e) => panic!("ERROR: {}", e.to_string())
    }

    scanner.next_token();
    println!("{:?}", scanner.current_token());
}
