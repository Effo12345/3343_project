use crate::{scanner::Scanner, token::Token};

use super::{Cond, StmtSeq};

pub enum If {
    If(Box<Cond>, Box<StmtSeq>),
    IfElse(Box<Cond>, Box<StmtSeq>, Box<StmtSeq>) // cond, true seq, else seq
}

impl If {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // assume if to start, handle else if it happens
        // also if token should be guaranteed by stmt logic, but check anyway
        if s.current_token() != Token::IF {
            return Err(format!("Expected if statement, got {:?}", s.current_token()));
        }
        s.next_token();

        let cond = Cond::new(s)?;

        if s.current_token() != Token::THEN {
            return Err(format!("Expected 'then' in if statement, got {:?}", s.current_token()));
        }
        s.next_token();

        let stmt_seq = StmtSeq::new(s)?;

        let mut else_seq: Option<Box<StmtSeq>> = None;
        if s.current_token() == Token::ELSE {
            // consume else then parse else-case code
            s.next_token();
            else_seq = Some(StmtSeq::new(s)?);
        }

        if s.current_token() != Token::END {
            return Err(format!("Missing 'end' in if statement, got {:?}", s.current_token()));
        }
        s.next_token();

        Ok(Box::new(
            match else_seq {
                Some(else_stmt_seq) => If::IfElse(cond, stmt_seq, else_stmt_seq),
                None => If::If(cond, stmt_seq)
            }
        ))
    }
}