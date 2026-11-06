use std::fmt;

use crate::{parser::{CallStack, FunctionMap}, scanner::Scanner, token::Token};

use super::{Assign, Call, Decl, If, Loop, PrettyPrint, Print, Read};

pub enum Stmt {
    Assign(Box<Assign>),
    If(Box<If>),
    Loop(Box<Loop>),
    Print(Box<Print>),
    Read(Box<Read>),
    Decl(Box<Decl>),
    Call(Box<Call>)
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
            Token::BEGIN => Ok(Box::new(Stmt::Call(Call::new(s)?))),
            _ => Err(format!("Unexpected token at the beginning of statement: {:?}", s.current_token()))
        }
    }

    // each statement handles its own variable and scope checks
    pub fn validate(&self, vars: &mut CallStack, funcs: &FunctionMap) -> Result<(), String> {
        match self {
            Stmt::Assign(assign) => assign.validate(vars),
            Stmt::If(boxed_if) => boxed_if.validate(vars, funcs),
            Stmt::Loop(boxed_loop) => boxed_loop.validate(vars, funcs),
            Stmt::Print(print) => print.validate(vars),
            Stmt::Read(read) => read.validate(vars),
            Stmt::Decl(decl) => decl.validate(vars),
            Stmt::Call(call) => call.validate(vars, funcs)
        }
    }

    pub fn execute(&self, vars: &mut CallStack, read_scanner: &mut Scanner, funcs: &FunctionMap) {
        match self {
            Stmt::Assign(assign) => assign.execute(vars),
            Stmt::If(boxed_if) => boxed_if.execute(vars, read_scanner, funcs),
            Stmt::Loop(boxed_loop) => boxed_loop.execute(vars, read_scanner, funcs),
            Stmt::Print(print) => print.execute(vars),
            Stmt::Read(read) => read.execute(vars, read_scanner),
            Stmt::Decl(decl) => decl.execute(vars),
            Stmt::Call(call) => call.execute(vars, read_scanner, funcs)
        };
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
            Stmt::Decl(decl) => decl.fmt_indented(f, level),
            Stmt::Call(call) => call.fmt_indented(f, level)
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
