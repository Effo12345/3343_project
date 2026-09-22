use std::fmt;

use crate::{scanner::Scanner, token::Token};

use super::{write_indent, DeclSeq, PrettyPrint, StmtSeq};

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

impl PrettyPrint for Procedure {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        match self {
            Procedure::WithDecl(id, decl_seq, stmt_seq) => {
                write_indent(f, level)?;
                writeln!(f, "procedure {id} is")?;

                decl_seq.fmt_indented(f, level + 1)?;

                write_indent(f, level)?;
                writeln!(f, "begin")?;

                stmt_seq.fmt_indented(f, level + 1)?;

                write_indent(f, level)?;
                writeln!(f, "end")
            },
            Procedure::NoDecl(id, stmt_seq) => {
                write_indent(f, level)?;
                writeln!(f, "procedure {id} is")?;

                write_indent(f, level)?;
                writeln!(f, "begin")?;

                stmt_seq.fmt_indented(f, level + 1)?;

                write_indent(f, level)?;
                writeln!(f, "end")
            }
        }
    }
}

impl fmt::Display for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
