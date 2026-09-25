use std::fmt;

use crate::{parser::VarStack, scanner::Scanner, token::Token};

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

    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        match self {
            Expr::Term(term) => term.validate(vars),
            Expr::Add(term, expr) => {
                term.validate(vars)?;
                expr.validate(vars)
            }
            Expr::Sub(term, expr) => {
                term.validate(vars)?;
                expr.validate(vars)
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Term(term) => write!(f, "{term}"),
            Expr::Add(term, expr) => write!(f, "{term} + {expr}"),
            Expr::Sub(term, expr) => write!(f, "{term} - {expr}")
        }
    }
}
