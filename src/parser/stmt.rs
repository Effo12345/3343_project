use std::fmt;

use crate::{parser::VarStack, scanner::Scanner, token::Token};

use super::{Assign, Decl, If, Loop, PrettyPrint, Print, Read};

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
        // the first token tells us which statement parser to use
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

    // each statement handles its own variable and scope checks
    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        match self {
            Stmt::Assign(assign) => assign.validate(vars),
            Stmt::If(boxed_if) => boxed_if.validate(vars),
            Stmt::Loop(boxed_loop) => boxed_loop.validate(vars),
            Stmt::Print(print) => print.validate(vars),
            Stmt::Read(read) => read.validate(vars),
            Stmt::Decl(decl) => decl.validate(vars)
        }
    }
}

// let the statement print itself using the current indentation
impl PrettyPrint for Stmt {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        match self {
            Stmt::Assign(assign) => assign.fmt_indented(f, level),
            Stmt::If(boxed_if) => boxed_if.fmt_indented(f, level),
            Stmt::Loop(boxed_loop) => boxed_loop.fmt_indented(f, level),
            Stmt::Print(print) => print.fmt_indented(f, level),
            Stmt::Read(read) => read.fmt_indented(f, level),
            Stmt::Decl(decl) => decl.fmt_indented(f, level)
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
