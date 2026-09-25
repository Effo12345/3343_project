use std::fmt;

use crate::{parser::{ScopedVar, VarStack, VarType}, scanner::Scanner, token::Token};

use super::{write_indent, Expr, PrettyPrint};

pub enum Assign {
    Expr(String, Box<Expr>),
    Subscript(String, String, Box<Expr>), // id, accessor, expr
    Object(String, String, Box<Expr>), // id, accessor, expr
    Colon(String, String) // id1, id2
}

impl Assign {
    fn parse_expr(id: String, s: &mut Scanner) -> Result<Self, String> {
        // = checked by new, consume it
        s.next_token();

        // differentiate expr vs. object type based on new keyword
        if s.current_token() == Token::NEW {
            return Assign::parse_object(id, s);
        }

        let expr = Expr::new(s)?;

        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in expression-based assignment: {id}"));
        }
        s.next_token();

        Ok(Assign::Expr(id, expr))
    }

    fn parse_subscript(id: String, s: &mut Scanner) -> Result<Self, String> {
        // [ checked by new, consume it
        s.next_token();

        let Token::STRING(accessor) = s.current_token() else {
            return Err(format!("Expected string accessor in subscripting assignment for {id}, got {:?}", s.current_token()));
        };
        s.next_token();

        if s.current_token() != Token::RSQUARE {
            return Err(format!("Expected closing ] in subscripting assignment for {id}, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::ASSIGN {
            return Err(format!("Expected assignment operator in subscripting assignment for {id}, got {:?}", s.current_token()));
        }
        s.next_token();

        let expr = Expr::new(s)?;

        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in subscripting assignment: {id}"));
        }
        s.next_token();

        Ok(Assign::Subscript(id, accessor, expr))
    }

    fn parse_object(id: String, s: &mut Scanner) -> Result<Self, String> {
        // new keyword already checked by parse_expr, consume it
        s.next_token();

        if s.current_token() != Token::OBJECT {
            return Err(format!("Expected 'object' keyword in object assignment for {id}, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::LPAREN {
            return Err(format!("Expected ( in object assignment for {id}, got {:?}", s.current_token()));
        }
        s.next_token();

        let Token::STRING(accessor) = s.current_token() else {
            return Err(format!("Expected accessor in object assignment for {id}, got {:?}", s.current_token()));
        };
        s.next_token();

        if s.current_token() != Token::COMMA {
            return Err(format!("Expected ',' between accessor and expression for {id}, got {:?}", s.current_token()));
        }
        s.next_token();

        let expr = Expr::new(s)?;

        if s.current_token() != Token::RPAREN {
            return Err(format!("Expected ) in object assignment for {id}, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in object assignment: {id}"));
        }
        s.next_token();

        Ok(Assign::Object(id, accessor, expr))
    }

    fn parse_colon(id: String, s: &mut Scanner) -> Result<Self, String> {
        // : checked by new, consume it
        s.next_token();

        let Token::ID(id2) = s.current_token() else {
            return Err(format!("Expected second identifier in : assignment for {id}, got {:?}", s.current_token()));
        };
        s.next_token();

        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in colon assignment: {id}"));
        }
        s.next_token();

        Ok(Assign::Colon(id, id2))
    }

    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        let Token::ID(id) = s.current_token() else {
            return Err(format!("Expected ID at start of assignment but got {:?}", s.current_token()))
        };
        s.next_token();

        match s.current_token() {
            Token::ASSIGN => Ok(Box::new(Assign::parse_expr(id, s)?)),
            Token::LSQUARE => Ok(Box::new(Assign::parse_subscript(id, s)?)),
            Token::COLON => Ok(Box::new(Assign::parse_colon(id, s)?)),
            _ => Err(format!("Unexpected token at the beginning of assignment: {:?}", s.current_token()))
        }
    }

    fn validate_id(id: String, id_type: Option<VarType>, vars: &VarStack, assignment_type: String) -> Result<(), String> {
        let mut var: Option<&ScopedVar> = None;

        for map in vars.iter().rev() {
            if let Some(found_var) = map.get(&id) {
                var = Some(found_var);
                break;
            }
        }

        let Some(scoped_var) = var else {
            return Err(format!("No such variable '{}' used in assignment", id));
        };

        if let Some(id_type_enum) = id_type && scoped_var.var_type != id_type_enum {
            return Err(format!("{} assignment using variable '{}' not valid for that variable type", assignment_type, id));
        };

        Ok(())
    }

    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        match self {
            Assign::Expr(id, expr) => {
                Assign::validate_id(id.to_string(), None, vars, String::new())?;
                expr.validate(vars)
            }
            Assign::Subscript(id, _, expr) => {
                Assign::validate_id(id.to_string(), Some(VarType::Object), vars, "Subscripting".to_string())?;
                expr.validate(vars)
            }
            Assign::Object(id, _, expr) => {
                Assign::validate_id(id.to_string(), Some(VarType::Object), vars, "Object type".to_string())?;
                expr.validate(vars)
            }
            Assign::Colon(id1, id2) => {
                Assign::validate_id(id1.to_string(), Some(VarType::Object), vars, "Colon type".to_string())?;
                Assign::validate_id(id2.to_string(), Some(VarType::Object), vars, "Colon type".to_string())
            }
        }
    }
}

impl PrettyPrint for Assign {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        write_indent(f, level)?;

        match self {
            Assign::Expr(id, expr) => writeln!(f, "{id} = {expr};"),
            Assign::Subscript(id, accessor, expr) => writeln!(f, "{id}['{accessor}'] = {expr};"),
            Assign::Object(id, accessor, expr) => writeln!(f, "{id} = new object('{accessor}', {expr});"),
            Assign::Colon(id1, id2) => writeln!(f, "{id1} : {id2};")
        }
    }
}

impl fmt::Display for Assign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
