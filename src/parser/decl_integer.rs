use crate::{scanner::Scanner, token::Token};

pub struct DeclInteger {
    id: String
}

impl DeclInteger {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // should never happen due to Decl logic but it doesn't hurt to check
        if s.current_token() != Token::INTEGER {
            return Err(format!("Expected integer declaration, got {:?}", s.current_token()));
        }
        s.next_token();

        let Token::ID(id) = s.current_token() else {
            return Err(format!("Expected ID in integer decl, got {:?}", s.current_token()));
        };
        s.next_token();

        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in integer decl: {id}"));
        }
        s.next_token();

        Ok(Box::new(DeclInteger{id}))
    }
}