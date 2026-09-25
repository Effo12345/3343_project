use std::fmt;

use crate::{parser::VarStack, scanner::Scanner, token::Token};

use super::{write_indent, Expr, PrettyPrint};

pub struct Print {
    expr: Box<Expr>
}

impl Print {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // should never happen due to stmt logic but it doesn't hurt to check
        if s.current_token() != Token::PRINT {
            return Err(format!("Expected print statement, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::LPAREN {
            return Err(format!("Expected ( in print statement, got {:?}", s.current_token()));
        }
        s.next_token();

        let expr = Expr::new(s)?;

        if s.current_token() != Token::RPAREN {
            return Err(format!("Expected ) in print statement, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in print statement"));
        }
        s.next_token();

        Ok(Box::new(Print{expr}))
    }

    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        self.expr.validate(vars)
    }
}

impl PrettyPrint for Print {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        write_indent(f, level)?;
        writeln!(f, "print({});", self.expr)
    }
}

impl fmt::Display for Print {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
