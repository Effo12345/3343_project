use std::fmt;

use crate::{parser::{CallStack, FunctionMap}, scanner::Scanner, token::Token};

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
            // leave the closing keyword for the surrounding if, loop, or procedure
            match s.current_token() {
                Token::END | Token::ELSE => StmtSeq::Stmt(stmt),
                _ => StmtSeq::Seq(stmt, StmtSeq::new(s)?)
            }
        ))
    }

    // validate in order so a declaration only applies to later statements
    pub fn validate(&self, vars: &mut CallStack, funcs: &FunctionMap) -> Result<(), String> {
        match self {
            StmtSeq::Stmt(stmt) => stmt.validate(vars, funcs),
            StmtSeq::Seq(stmt, stmt_seq) => {
                stmt.validate(vars, funcs)?;
                stmt_seq.validate(vars, funcs)
            }
        }
    }

    pub fn execute(&self, vars: &mut CallStack, read_scanner: &mut Scanner, funcs: &FunctionMap) {
        match self {
            StmtSeq::Stmt(stmt) => stmt.execute(vars, read_scanner, funcs),
            StmtSeq::Seq(stmt, stmt_seq) => {
                stmt.execute(vars, read_scanner, funcs);
                stmt_seq.execute(vars, read_scanner, funcs)
            }
        }
    }
}

// all statements in this sequence belong at the same indentation level
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
