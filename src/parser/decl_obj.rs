use std::fmt;

use crate::{parser::{ScopedVar, VarStack, VarType, utils::stdout_expect}, scanner::Scanner, token::Token};

use super::{write_indent, PrettyPrint};

pub struct DeclObj {
    id: String
}

impl DeclObj {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // should never happen due to Decl logic but it doesn't hurt to check
        if s.current_token() != Token::OBJECT {
            return Err(format!("Expecting object declaration, got {:?}", s.current_token()));
        }
        s.next_token();

        // save the name for validation and printing
        let Token::ID(id) = s.current_token() else {
            return Err(format!("Expected ID in object decl, got {:?}", s.current_token()));
        };
        s.next_token();

        // finish the declaration before returning to the caller
        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in object decl: {id}"));
        }
        s.next_token();

        Ok(Box::new(DeclObj{id}))
    }

    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        // only the current scope matters for duplicate declarations
        let Some(curr_scope) = vars.last_mut() else {
            return Err(format!("Variable {} declared outside any scope", self.id));
        };

        // insert returns the old value if this name was already declared here
        match curr_scope.insert(self.id.clone(), ScopedVar{var_type: VarType::empty_object()}) {
            Some(_) => Err(format!("Variable {} declared multiple times in the same scope", self.id)),
            None => Ok(())
        }
    }

     pub fn execute(&self, vars: &mut VarStack) {
        // this runtime error should never happen due to syntax checks
        let curr_scope = stdout_expect(vars.last_mut(), "Bad variable declaration");

        // objects get initialized by the "new" keyword, so don't bother now
        curr_scope.insert(self.id.clone(), ScopedVar{var_type: VarType::empty_object()});
     }
}

// print one declaration at the requested indentation
impl PrettyPrint for DeclObj {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        write_indent(f, level)?;
        writeln!(f, "object {};", self.id)
    }
}

impl fmt::Display for DeclObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
