use std::fmt;

use crate::{parser::{VarStack, utils::{exit_with_msg, stdout_expect}, variables::VarStackExt}, scanner::Scanner, token::Token};

use super::{write_indent, PrettyPrint};

pub struct Read {
    id: String
}

impl Read {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // should never happen due to stmt logic but it doesn't hurt to check
        if s.current_token() != Token::READ {
            return Err(format!("Expected read statement, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::LPAREN {
            return Err(format!("Expected ( in read statement, got {:?}", s.current_token()));
        }
        s.next_token();

        // read takes a single variable name inside the parentheses
        let Token::ID(id) = s.current_token() else {
            return Err(format!("Expected ID in read statement, got {:?}", s.current_token()));
        };
        s.next_token();

        if s.current_token() != Token::RPAREN {
            return Err(format!("Expected ) in read statement, got {:?}", s.current_token()));
        }
        s.next_token();

        // consume the final semicolon before parsing the next statement
        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in read statement"));
        }
        s.next_token();

        Ok(Box::new(Read{id}))
    }

    fn validate_id(id: String, vars: &mut VarStack) -> Result<(), String> {
        let var = vars.resolve_var(&id);

        // make sure the variable exists before it can be read into
        let Some(_) = var else {
            return Err(format!("Undeclared variable '{}' used in read statement", id));
        };

        Ok(())
    }

    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        Read::validate_id(self.id.to_string(), vars)
    }

    pub fn execute(&self, vars: &mut VarStack, read_scanner: &mut Scanner) {
        let const_val = match read_scanner.current_token() {
            Token::CONST(val) => val,
            Token::EOS => exit_with_msg("Attempted to read past end of data sequence"),
            _ => exit_with_msg("Unknown token read in data sequence")
        };
        read_scanner.next_token();

        let var = stdout_expect(vars.resolve_var(&self.id), "Undeclared variable used somehow");
        var.var_type.set_int_or_default(&self.id, const_val as i32);
    }
}

// print the read statement on its own line
impl PrettyPrint for Read {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        write_indent(f, level)?;
        writeln!(f, "read({});", self.id)
    }
}

impl fmt::Display for Read {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
