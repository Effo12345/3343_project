use std::fmt;

use crate::{parser::VarStack, scanner::Scanner, token::Token};

use super::{Decl, PrettyPrint};

pub enum DeclSeq {
    Decl(Box<Decl>),
    Seq(Box<Decl>, Box<DeclSeq>)
}

impl DeclSeq {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // parse a decl to start, then decide if we need the seq later
        let decl = Decl::new(s)?;
        Ok(Box::new(
            // global declarations continue until the procedure body starts
            if s.current_token() != Token::BEGIN {
                DeclSeq::Seq(decl, DeclSeq::new(s)?)
            }
            else {
                DeclSeq::Decl(decl)
            }
        ))
    }

    // add declarations to the scope in the same order they appeared
    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        match self {
           DeclSeq::Decl(decl) => decl.validate(vars)?,
           DeclSeq::Seq(decl, decl_seq) => {
                decl.validate(vars)?;
                decl_seq.validate(vars)?
           }
        };

        Ok(())
    }
}

// declarations in the same sequence share an indentation level
impl PrettyPrint for DeclSeq {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        match self {
            DeclSeq::Decl(decl) => decl.fmt_indented(f, level),
            DeclSeq::Seq(decl, decl_seq) => {
                decl.fmt_indented(f, level)?;
                decl_seq.fmt_indented(f, level)
            }
        }
    }
}

impl fmt::Display for DeclSeq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
