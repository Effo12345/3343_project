use crate::{scanner::Scanner, token::Token};

use super::{DeclSeq, StmtSeq};

pub enum Procedure {
    WithDecl(String, Box<DeclSeq>, Box<StmtSeq>),
    NoDecl(String, Box<StmtSeq>)
}

impl Procedure {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        if s.current_token() != Token::PROCEDURE {
            return Err(format!("Expected 'procedure' but got {:?}", s.current_token()));
        }
        s.next_token();

        let Token::ID(id_str) = s.current_token() else {
            return Err(format!("Expected procedure name but got {:?}", s.current_token()));
        };
        s.next_token();

        if s.current_token() != Token::IS {
            return Err(format!("Expected 'is' but got {:?}", s.current_token()));
        }
        s.next_token();

        let decl_seq = match s.current_token() {
            Token::BEGIN => None,
            _ => Some(DeclSeq::new(s)?)
        };

        if s.current_token() != Token::BEGIN {
            return Err(format!("Expected 'begin' but got {:?}", s.current_token()));
        }
        s.next_token();

        let stmt_seq = StmtSeq::new(s)?;

        Ok(Box::new(
            match decl_seq {
                Some(decls) => Procedure::WithDecl(id_str, decls, stmt_seq),
                None => Procedure::NoDecl(id_str, stmt_seq)
            }
        ))

    }
}
