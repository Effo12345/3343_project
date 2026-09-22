use crate::{scanner::Scanner, token::Token};

use super::Expr;

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
}
