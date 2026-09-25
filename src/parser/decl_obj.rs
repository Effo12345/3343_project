use std::fmt;

use crate::{parser::{ScopedVar, VarStack, VarType}, scanner::Scanner, token::Token};

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

        let Token::ID(id) = s.current_token() else {
            return Err(format!("Expected ID in object decl, got {:?}", s.current_token()));
        };
        s.next_token();

        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in object decl: {id}"));
        }
        s.next_token();

        Ok(Box::new(DeclObj{id}))
    }

    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        let Some(curr_scope) = vars.last_mut() else {
            return Err(format!("Variable {} declared outside any scope", self.id));
        };

        match curr_scope.insert(self.id.clone(), ScopedVar{var_type: VarType::Object}) {
            Some(_) => Err(format!("Variable {} declared multiple times in the same scope", self.id)),
            None => Ok(())
        }
    }
}

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
