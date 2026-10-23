use std::fmt;

use crate::{parser::{ScopedVar, VarStack, VarType, variables::VarStackExt, utils::stdout_expect}, scanner::Scanner, token::Token};

use super::Expr;

// a factor is a value, an object access, or a parenthesized expression
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

        // parse the expression inside the parentheses
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

        // object accessors must be strings
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

    // choose the factor type from the first token
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

    fn validate_id(id: String, id_type: Option<VarType>, vars: &mut VarStack, factor_type: String) -> Result<(), String> {
        let var = vars.resolve_var(&id);

        // the ID must be declared in a visible scope
        let Some(scoped_var) = var else {
            return Err(format!("Undeclared variable '{}' used in factor", id));
        };

        // only check the type if the caller requested one
        if let Some(id_type_enum) = id_type && scoped_var.var_type != id_type_enum {
            return Err(format!("{} factor using variable '{}' not valid for factor type", factor_type, id));
        };

        Ok(())
    }

    // plain IDs allow either type, but subscripting needs an object
    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        match self {
            Factor::Id(id) => Factor::validate_id(id.to_string(), None, vars, "".to_string()),
            Factor::IdSubscript(id, _) =>
                Factor::validate_id(id.to_string(), Some(VarType::empty_object()), vars, "Subscripting".to_string()),
            Factor::Const(_) => Ok(()),
            Factor::SubExpr(expr) => expr.validate(vars)
        }
    }

    pub fn execute(&self, vars: &mut VarStack) -> i32 {
        match self {
            Factor::Id(id) => {
                let var = stdout_expect(vars.resolve_var(id), "Undeclared variable used somehow");

                match &var.var_type {
                    VarType::Integer(val) => *val,
                    VarType::Object(map_opt, default_key) => {
                        let map = stdout_expect(map_opt.as_ref(), &format!("Used null object '{id}' as factor r-value"));

                        // guaranteed to contain default key since object is not null
                        *(map.borrow().get(default_key).unwrap())
                    }
                }
            }
            Factor::IdSubscript(id, accessor) => {
                let var = stdout_expect(vars.resolve_var(id), "Undeclared variable used somehow");

                match &var.var_type {
                    VarType::Object(map_opt, _) => {
                        let map = stdout_expect(map_opt.as_ref(), &format!("Attempted to subscript object '{id}' using accessor '{accessor}' as factor r-value"));

                        *stdout_expect(map.borrow().get(accessor), &format!("Undefined key '{accessor}' in object '{id}'"))
                    },
                    _ => unreachable!() // by semantic checks
                }
            }
            Factor::Const(val) => *val as i32,
            Factor::SubExpr(expr) => expr.execute(vars)
        }
    }
}

// put the accessor quotes and expression parentheses back
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
