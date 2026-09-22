use crate::{scanner::Scanner, token::Token};

use super::{Assign, If, Loop, Print, Read, Decl};

pub enum Stmt {
    Assign(Box<Assign>),
    If(Box<If>),
    Loop(Box<Loop>),
    Print(Box<Print>),
    Read(Box<Read>),
    Decl(Box<Decl>)
}

impl Stmt {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        match s.current_token() {
            Token::ID(_) => Ok(Box::new(Stmt::Assign(Assign::new(s)?))),
            Token::IF => Ok(Box::new(Stmt::If(If::new(s)?))),
            Token::FOR => Ok(Box::new(Stmt::Loop(Loop::new(s)?))),
            Token::PRINT => Ok(Box::new(Stmt::Print(Print::new(s)?))),
            Token::READ => Ok(Box::new(Stmt::Read(Read::new(s)?))),
            Token::INTEGER | Token::OBJECT => Ok(Box::new(Stmt::Decl(Decl::new(s)?))),
            _ => Err(format!("Unexpected token at the beginning of statement: {:?}", s.current_token()))
        }
    }
}
