use std::fmt;

use crate::{parser::VarStack, scanner::Scanner, token::Token};

use super::Cmpr;

// keep brackets and not in the tree for printing later
pub enum Cond {
    Cmpr(Box<Cmpr>),
    Not(Box<Cond>),
    Bracket(Box<Cond>),
    Or(Box<Cmpr>, Box<Cond>),
    And(Box<Cmpr>, Box<Cond>)
}

impl Cond {
    fn parse_bracket(s: &mut Scanner) -> Result<Self, String> {
        // [ checked by new, consume it
        s.next_token();

        // parse the entire condition inside the brackets
        let cond = Cond::new(s)?;

        if s.current_token() != Token::RSQUARE {
            return Err(format!("Missing ] in bracketed condition, got {:?}", s.current_token()));
        }
        s.next_token();

        Ok(Cond::Bracket(cond))
    }

    fn parse_bare_and_or(s: &mut Scanner) -> Result<Self, String> {
        // start with a comparison, then check for a connecting operator
        let cmpr = Cmpr::new(s)?;

        Ok(
            match s.current_token() {
                Token::OR => {
                    s.next_token();
                    Cond::Or(cmpr, Cond::new(s)?)
                },
                Token::AND => {
                    s.next_token();
                    Cond::And(cmpr, Cond::new(s)?)
                },
                _ => Cond::Cmpr(cmpr)
            }
        )
    }

    // not and brackets start their own condition forms
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        Ok(Box::new(
            match s.current_token() {
                Token::NOT => {
                    // consume not token
                    s.next_token();
                    Cond::Not(Cond::new(s)?)
                }
                Token::LSQUARE => Cond::parse_bracket(s)?,
                _ => Cond::parse_bare_and_or(s)?
            }
        ))
    }

    // check variables in every comparison, including nested conditions
    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        match self {
            Cond::Cmpr(cmpr) => cmpr.validate(vars),
            Cond::Not(cond) | Cond::Bracket(cond) => cond.validate(vars),
            Cond::Or(cmpr, cond) | Cond::And(cmpr, cond) => {
                cmpr.validate(vars)?;
                cond.validate(vars)
            }
        }
    }
}

// put the condition operators and brackets back
impl fmt::Display for Cond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cond::Cmpr(cmpr) => write!(f, "{cmpr}"),
            Cond::Not(cond) => write!(f, "not {cond}"),
            Cond::Bracket(cond) => write!(f, "[{cond}]"),
            Cond::Or(cmpr, cond) => write!(f, "{cmpr} or {cond}"),
            Cond::And(cmpr, cond) => write!(f, "{cmpr} and {cond}")
        }
    }
}
