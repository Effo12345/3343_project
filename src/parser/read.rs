use std::fmt;

use crate::{scanner::Scanner, token::Token};

use super::{write_indent, PrettyPrint};

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

impl PrettyPrint for Read {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        write_indent(f, level)?;
        writeln!(f, "read({});", self.id)
    }
}

impl fmt::Display for Read {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
