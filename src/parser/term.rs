use std::fmt;

use crate::{parser::{VarStack, utils::exit_with_msg}, scanner::Scanner, token::Token};

use super::Factor;

// keep multiplication and division inside the term
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

    // check each factor and the rest of the term
    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        match self {
            Term::Fac(fac) => fac.validate(vars),
            Term::Mult(fac, term) => {
                fac.validate(vars)?;
                term.validate(vars)
            }
            Term::Div(fac, term) => {
                fac.validate(vars)?;
                term.validate(vars)
            }
        }
    }

    pub fn execute(&self, vars: &mut VarStack) -> i32 {
        match self {
            Term::Fac(fac) => fac.execute(vars),
            Term::Mult(fac, term) => {
                // wrap on overflow
                fac.execute(vars).wrapping_mul(term.execute(vars))
            }
            Term::Div(fac, term) => {
                // evaluate fac first so error messages are thrown left to right
                let fac_val = fac.execute(vars);

                let term_val = term.execute(vars);
                if term_val == 0 {
                    exit_with_msg("Division by zero");
                }

                fac_val.wrapping_div(term_val)
            }
        }
    }
}

// print the factors and any remaining term
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Fac(factor) => write!(f, "{factor}"),
            Term::Mult(factor, term) => write!(f, "{factor} * {term}"),
            Term::Div(factor, term) => write!(f, "{factor} / {term}")
        }
    }
}
