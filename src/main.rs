mod token;
mod scanner;
mod parser;

use std::{env, process::exit};

use scanner::Scanner;
use parser::Procedure;

use crate::parser::VarStack;

fn main() {
    // get the input file from the command line
    let Some(filename) = env::args().nth(1) else {
        println!("ERROR: filename to parse must be provided as first argument");
        return;
    };

    // open the file and read the first token
    let mut scanner = match Scanner::new(&filename) {
        Ok(scanner_struct) => scanner_struct,
        Err(e) => {
            println!("ERROR: {}: {filename}", e.to_string());
            return;
        }
    };

    // build the syntax tree, stopping at the first parse error
    let proc: Procedure;
    match Procedure::new(&mut scanner) {
        Ok(procedure) => proc = *procedure,
        Err(e) => {
            println!("ERROR: {e}");
            exit(-1);
        }
    };
    
    // validate scopes and variable types before printing anything
    let mut vars: VarStack = Vec::new();

    if let Err(e) = proc.validate(&mut vars) {
        println!("ERROR: {e}");
    }
    else {
        // Display handles the indentation for the whole tree
        println!("{proc}");
    }
}
