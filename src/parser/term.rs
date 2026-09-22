use std::fmt;

use crate::{scanner::Scanner, token::Token};

use super::Factor;

pub enum Term {
    Fac(Box<Factor>),
    Mult(Box<Factor>, Box<Term>),
    Div(Box<Factor>, Box<Term>)
}

impl Term {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // start with factor, then figure out the operator
        let factor = Factor::new(s)?;
        
        Ok(Box::new(
            match s.current_token() {
                Token::MULTIPLY => {
                    s.next_token();
                    Term::Mult(factor, Term::new(s)?)
                },
                Token::DIVIDE => {
                    s.next_token();
                    Term::Div(factor, Term::new(s)?)
                },
                _ => Term::Fac(factor)
            }
        ))
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Fac(factor) => write!(f, "{factor}"),
            Term::Mult(factor, term) => write!(f, "{factor} * {term}"),
            Term::Div(factor, term) => write!(f, "{factor} / {term}")
        }
    }
}
