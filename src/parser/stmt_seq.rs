use crate::{scanner::Scanner, token::Token};

use super::Stmt;

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
}
