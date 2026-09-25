use std::fmt;

use crate::{parser::VarStack, scanner::Scanner, token::Token};

use super::Expr;

pub enum Cmpr {
    Equality(Box<Expr>, Box<Expr>), // expr1, expr2
    LT(Box<Expr>, Box<Expr>) // expr1, expr2
}

impl Cmpr {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // start with expr, then figure out operator
        let expr1 = Expr::new(s)?;

        match s.current_token() {
            Token::EQUAL => {
                s.next_token();
                Ok(Box::new(Cmpr::Equality(expr1, Expr::new(s)?)))
            },
            Token::LESS => {
                s.next_token();
                Ok(Box::new(Cmpr::LT(expr1, Expr::new(s)?)))
            },
            _ => Err(format!("Unexpected token at start of comparison: {:?}", s.current_token()))
        }
    }

    // both sides need to be valid expressions
    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        match self {
            Cmpr::Equality(expr1, expr2) | Cmpr::LT(expr1, expr2) => {
                expr1.validate(vars)?;
                expr2.validate(vars)
            }
        }
    }
}

// print the comparison between the two expressions
impl fmt::Display for Cmpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cmpr::Equality(expr1, expr2) => write!(f, "{expr1} == {expr2}"),
            Cmpr::LT(expr1, expr2) => write!(f, "{expr1} < {expr2}")
        }
    }
}