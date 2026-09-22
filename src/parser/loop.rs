use crate::{scanner::Scanner, token::Token};

use super::{Expr, Cond, StmtSeq};

pub struct Loop {
    assignment: Box<Expr>,
    cond: Box<Cond>,
    increment: Box<Expr>,
    statements: Box<StmtSeq>
}

impl Loop {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // should never happen due to stmt logic but it doesn't hurt to check
        if s.current_token() != Token::FOR {
            return Err(format!("Expected for loop, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::LPAREN {
            return Err(format!("Expected ( in for loop, got {:?}", s.current_token()));
        }
        s.next_token();

        let assignment = Expr::new(s)?;

        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in for loop"));
        }
        s.next_token();

        let cond = Cond::new(s)?;

        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in for loop"));
        }
        s.next_token();

        let increment = Expr::new(s)?;

        if s.current_token() != Token::RPAREN {
            return Err(format!("Expected ) in for loop, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::DO {
            return Err(format!("Missing 'do' in for loop, got {:?}", s.current_token()));
        }
        s.next_token();

        let statements = StmtSeq::new(s)?;

        if s.current_token() != Token::END {
            return Err(format!("Missing 'end' in for loop, got {:?}", s.current_token()));
        }
        s.next_token();

        Ok(Box::new(Loop{assignment, cond, increment, statements}))
    }
}