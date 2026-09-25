use std::fmt;

use crate::{parser::VarStack, scanner::Scanner, token::Token};

use super::{DeclInteger, DeclObj, PrettyPrint};

pub enum Decl {
    DeclInteger(Box<DeclInteger>),
    DeclObj(Box<DeclObj>)
}

impl Decl {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // choose the declaration type from its starting keyword
        match s.current_token() {
            Token::INTEGER => Ok(Box::new(Decl::DeclInteger(DeclInteger::new(s)?))),
            Token::OBJECT => Ok(Box::new(Decl::DeclObj(DeclObj::new(s)?))),
            _ => Err(format!("Expected 'integer' or 'object' to start declaration, got {:?}", s.current_token()))
        }
    }

    // let the declaration add its variable to the current scope
    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        match self {
            Decl::DeclInteger(decl_int) => decl_int.validate(vars)?,
            Decl::DeclObj(decl_obj) => decl_obj.validate(vars)?
        };

        Ok(())
    }
}

// pass the current indentation through to the declaration
impl PrettyPrint for Decl {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        match self {
            Decl::DeclInteger(decl_integer) => decl_integer.fmt_indented(f, level),
            Decl::DeclObj(decl_obj) => decl_obj.fmt_indented(f, level)
        }
    }
}

impl fmt::Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
