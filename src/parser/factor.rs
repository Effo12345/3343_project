use std::fmt;

use crate::{parser::{ScopedVar, VarStack, VarType}, scanner::Scanner, token::Token};

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

    fn validate_id(id: String, id_type: Option<VarType>, vars: &VarStack, factor_type: String) -> Result<(), String> {
        let mut var: Option<&ScopedVar> = None;

        for map in vars.iter().rev() {
            if let Some(found_var) = map.get(&id) {
                var = Some(found_var);
                break;
            }
        }

        let Some(scoped_var) = var else {
            return Err(format!("Undeclared variable '{}' used in factor", id));
        };

        if let Some(id_type_enum) = id_type && scoped_var.var_type != id_type_enum {
            return Err(format!("{} factor using variable '{}' not valid for factor type", factor_type, id));
        };

        Ok(())
    }

    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        match self {
            Factor::Id(id) => Factor::validate_id(id.to_string(), None, vars, "".to_string()),
            Factor::IdSubscript(id, _) =>
                Factor::validate_id(id.to_string(), Some(VarType::Object), vars, "Subscripting".to_string()),
            Factor::Const(_) => Ok(()),
            Factor::SubExpr(expr) => expr.validate(vars)
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
