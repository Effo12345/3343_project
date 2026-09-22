use crate::{scanner::Scanner, token::Token};

use super::{DeclInteger, DeclObj};

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