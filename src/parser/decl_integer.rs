use std::fmt;

use crate::{parser::{ScopedVar, VarStack, VarType}, scanner::Scanner, token::Token};

use super::{write_indent, PrettyPrint};

pub struct DeclInteger {
    id: String
}

impl DeclInteger {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // should never happen due to Decl logic but it doesn't hurt to check
        if s.current_token() != Token::INTEGER {
            return Err(format!("Expected integer declaration, got {:?}", s.current_token()));
        }
        s.next_token();

        // save the name for validation and printing
        let Token::ID(id) = s.current_token() else {
            return Err(format!("Expected ID in integer decl, got {:?}", s.current_token()));
        };
        s.next_token();

        // finish the declaration before returning to the caller
        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in integer decl: {id}"));
        }
        s.next_token();

        Ok(Box::new(DeclInteger{id}))
    }

    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        // only the current scope matters for duplicate declarations
        let Some(curr_scope) = vars.last_mut() else {
            return Err("Variable declared outside any scope".to_string());
        };

        // insert returns the old value if this name was already declared here
        match curr_scope.insert(self.id.clone(), ScopedVar{var_type: VarType::Integer}) {
            Some(_) => Err(format!("Variable {} declared multiple times in the same scope", self.id)),
            None => Ok(())
        }
    }
}

// print one declaration at the requested indentation
impl PrettyPrint for DeclInteger {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        write_indent(f, level)?;
        writeln!(f, "integer {};", self.id)
    }
}

impl fmt::Display for DeclInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
