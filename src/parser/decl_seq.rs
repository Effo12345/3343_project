use crate::{scanner::Scanner, token::Token};

use super::Decl;

pub enum DeclSeq {
    Decl(Box<Decl>),
    Seq(Box<Decl>, Box<DeclSeq>)
}

impl DeclSeq {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // parse a decl to start, then decide if we need the seq later
        let decl = Decl::new(s)?;
        Ok(Box::new(
            if s.current_token() != Token::BEGIN {
                DeclSeq::Seq(decl, DeclSeq::new(s)?)
            }
            else {
                DeclSeq::Decl(decl)
            }
        ))
    }
}
