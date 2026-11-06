use std::fmt;

use crate::{scanner::Scanner, token::Token};

use super::{write_indent, Parameters, PrettyPrint};

pub struct Call {
    id: String,
    args: Box<Parameters>
}

impl Call {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // should never happen due to stmt logic but it doesn't hurt to check
        if s.current_token() != Token::BEGIN {
            return Err(format!("Expected procedure call, got {:?}", s.current_token()));
        }
        s.next_token();

        let Token::ID(id) = s.current_token() else {
            return Err(format!("Expected procedure name in call, got {:?}", s.current_token()));
        };
        s.next_token();

        if s.current_token() != Token::LPAREN {
            return Err(format!("Expected ( in call to procedure {id}, got {:?}", s.current_token()));
        }
        s.next_token();

        let args = Parameters::new(s)?;

        if s.current_token() != Token::RPAREN {
            return Err(format!("Expected ) in call to procedure {id}, got {:?}", s.current_token()));
        }
        s.next_token();

        // consume the final semicolon before parsing the next statement
        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in call to procedure {id}"));
        }
        s.next_token();

        Ok(Box::new(Call{id, args}))
    }
}

// print the call on its own line
impl PrettyPrint for Call {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        write_indent(f, level)?;
        writeln!(f, "begin {}({});", self.id, self.args)
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
