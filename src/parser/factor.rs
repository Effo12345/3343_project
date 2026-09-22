use std::fmt;

use crate::{scanner::Scanner, token::Token};

use super::Expr;

pub enum Factor {
    Id(String),
    IdSubscript(String, String), // id, accessor
    Const(u16),
    SubExpr(Box<Expr>)
}

impl Factor {
    fn parse_parenthetical(s: &mut Scanner) -> Result<Self, String> {
        // ( checked by new, consume it
        s.next_token();

        let expr = Expr::new(s)?;

        if s.current_token() != Token::RPAREN {
            return Err(format!("Missing ) in parenthetical factor, got {:?}", s.current_token()));
        }
        s.next_token();

        Ok(Factor::SubExpr(expr))
    }

    fn parse_id(id: String, s: &mut Scanner) -> Result<Self, String> {
        // consume ID
        s.next_token();

        Ok(
            match s.current_token() {
                Token::LSQUARE => Factor::parse_subscript(id, s)?,
                _ => Factor::Id(id)
            }
        )
    }

    fn parse_subscript(id: String, s: &mut Scanner) -> Result<Self, String> {
        // [ already checked by parse_id, consume it
        s.next_token();

        let Token::STRING(accessor) = s.current_token() else {
            return Err(format!("Expected string in subscripting factor, got {:?}", s.current_token()));
        };
        s.next_token();

        if s.current_token() != Token::RSQUARE {
            return Err(format!("Missing ] in subscripted factor, got {:?}", s.current_token()));
        }
        s.next_token();

        Ok(Factor::IdSubscript(id, accessor))
    }

    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        match s.current_token() {
            Token::CONST(constant) => {
                s.next_token();
                Ok(Box::new(Factor::Const(constant)))
            },
            Token::LPAREN => Ok(Box::new(Factor::parse_parenthetical(s)?)),
            Token::ID(id) => Ok(Box::new(Factor::parse_id(id, s)?)),
            _ => Err(format!("Unexpected token at start of factor: {:?}", s.current_token()))
        }
    }
}

impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Factor::Id(id) => write!(f, "{id}"),
            Factor::IdSubscript(id, accessor) => write!(f, "{id}['{accessor}']"),
            Factor::Const(val) => write!(f, "{val}"),
            Factor::SubExpr(expr) => write!(f, "({expr})")
        }
    }
}
