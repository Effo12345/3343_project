use std::fmt;

use crate::{parser::VarStack, scanner::Scanner, token::Token};

use super::{PrettyPrint, Stmt};

pub enum StmtSeq {
    Stmt(Box<Stmt>),
    Seq(Box<Stmt>, Box<StmtSeq>)
}

impl StmtSeq {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // parse a stmt to start, then decide if we need the seq later
        let stmt = Stmt::new(s)?;

        Ok(Box::new(
            match s.current_token() {
                Token::END | Token::ELSE => StmtSeq::Stmt(stmt),
                _ => StmtSeq::Seq(stmt, StmtSeq::new(s)?)
            }
        ))
    }

    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        match self {
            StmtSeq::Stmt(stmt) => stmt.validate(vars),
            StmtSeq::Seq(stmt, stmt_seq) => {
                stmt.validate(vars)?;
                stmt_seq.validate(vars)
            }
        }
    }
}

impl PrettyPrint for StmtSeq {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        match self {
            StmtSeq::Stmt(stmt) => stmt.fmt_indented(f, level),
            StmtSeq::Seq(stmt, stmt_seq) => {
                stmt.fmt_indented(f, level)?;
                stmt_seq.fmt_indented(f, level)
            }
        }
    }
}

impl fmt::Display for StmtSeq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
