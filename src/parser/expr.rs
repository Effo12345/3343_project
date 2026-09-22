use crate::{scanner::Scanner, token::Token};

use super::Term;

pub enum Expr {
    Term(Box<Term>),
    Add(Box<Term>, Box<Expr>),
    Sub(Box<Term>, Box<Expr>)
}

impl Expr {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // start with term, then figure out the operator
        let term = Term::new(s)?;
        
        Ok(Box::new(
            match s.current_token() {
                Token::ADD => {
                    s.next_token();
                    Expr::Add(term, Expr::new(s)?)
                },
                Token::SUBTRACT => {
                    s.next_token();
                    Expr::Sub(term, Expr::new(s)?)
                },
                _ => Expr::Term(term)
            }
        ))
    }
}