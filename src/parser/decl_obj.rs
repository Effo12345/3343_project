use crate::{scanner::Scanner, token::Token};

pub struct DeclObj {
    id: String
}

impl DeclObj {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // should never happen due to Decl logic but it doesn't hurt to check
        if s.current_token() != Token::OBJECT {
            return Err(format!("Expecting object declaration, got {:?}", s.current_token()));
        }
        s.next_token();

        let Token::ID(id) = s.current_token() else {
            return Err(format!("Expected ID in object decl, got {:?}", s.current_token()));
        };
        s.next_token();

        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in object decl: {id}"));
        }
        s.next_token();

        Ok(Box::new(DeclObj{id}))
    }
}
