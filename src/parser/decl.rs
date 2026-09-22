use std::fmt;

use crate::{scanner::Scanner, token::Token};

use super::{DeclInteger, DeclObj, PrettyPrint};

pub enum Decl {
    DeclInteger(Box<DeclInteger>),
    DeclObj(Box<DeclObj>)
}

impl Decl {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        match s.current_token() {
            Token::INTEGER => Ok(Box::new(Decl::DeclInteger(DeclInteger::new(s)?))),
            Token::OBJECT => Ok(Box::new(Decl::DeclObj(DeclObj::new(s)?))),
            _ => Err(format!("Expected 'integer' or 'object' to start declaration, got {:?}", s.current_token()))
        }
    }
}

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
