mod token;
mod scanner;
mod parser;

use std::env;

use scanner::Scanner;
use parser::Procedure;

use crate::parser::{VarStack, utils::exit_with_msg};

fn create_scanner(env_idx: usize, arg_error: &str) -> Option<Scanner> {
    // get the input file from the command line
    let Some(filename) = env::args().nth(env_idx) else {
        println!("ERROR: {arg_error}");
        return None;
    };

    // open the file and read the first token
    let scanner = match Scanner::new(&filename) {
        Ok(scanner_struct) => scanner_struct,
        Err(e) => {
            println!("ERROR: {}: {filename}", e.to_string());
            return None;
        }
    };

    Some(scanner)
}

fn main() {
    // get the code scanner
    let Some(mut code_scanner) = create_scanner(1, "Provide code file path as first argument") else {
        return;
    };

    // get the data scanner
    let Some(mut data_scanner) = create_scanner(2, "Provide data file path as second argument") else {
        return;
    };

    // build the syntax tree, stopping at the first parse error
    let proc: Procedure;
    match Procedure::new(&mut code_scanner) {
        Ok(procedure) => proc = *procedure,
        Err(e) => exit_with_msg(&e)
    };
    
    // validate scopes and variable types before printing anything
    let mut vars: VarStack = Vec::new();

    if let Err(e) = proc.validate(&mut vars) {
        exit_with_msg(&e);
    }

    // display handles the indentation for the whole tree
    // keep if we ever need to print again in future
    // println!("{proc}");

    // safe to reuse vars here since proc.validate pops everything off
    proc.execute(&mut vars, &mut data_scanner);
}
