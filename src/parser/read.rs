use crate::{scanner::Scanner, token::Token};

pub struct Read {
    id: String
}

impl Read {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // should never happen due to stmt logic but it doesn't hurt to check
        if s.current_token() != Token::READ {
            return Err(format!("Expected read statement, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::LPAREN {
            return Err(format!("Expected ( in read statement, got {:?}", s.current_token()));
        }
        s.next_token();

        let Token::ID(id) = s.current_token() else {
            return Err(format!("Expected ID in read statement, got {:?}", s.current_token()));
        };
        s.next_token();

        if s.current_token() != Token::RPAREN {
            return Err(format!("Expected ) in read statement, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in read statement"));
        }
        s.next_token();

        Ok(Box::new(Read{id}))
    }
}
