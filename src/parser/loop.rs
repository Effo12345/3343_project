use std::fmt;

use crate::{scanner::Scanner, token::Token};

use super::{write_indent, Cond, Expr, PrettyPrint, StmtSeq};

pub struct Loop {
    assignment_id: String,
    assignment_expr: Box<Expr>,
    cond: Box<Cond>,
    increment: Box<Expr>,
    statements: Box<StmtSeq>
}

impl Loop {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // should never happen due to stmt logic but it doesn't hurt to check
        if s.current_token() != Token::FOR {
            return Err(format!("Expected for loop, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::LPAREN {
            return Err(format!("Expected ( in for loop, got {:?}", s.current_token()));
        }
        s.next_token();
        
        let Token::ID(assignment_id) = s.current_token() else {
            return Err(format!("Expected an identifier in for loop assignment, got {:?}", s.current_token()));
        };
        s.next_token();

        if s.current_token() != Token::ASSIGN {
            return Err(format!("Expected = in for loop assignment, got {:?}", s.current_token()));
        }
        s.next_token();

        let assignment_expr = Expr::new(s)?;

        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in for loop"));
        }
        s.next_token();

        let cond = Cond::new(s)?;

        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in for loop"));
        }
        s.next_token();

        let increment = Expr::new(s)?;

        if s.current_token() != Token::RPAREN {
            return Err(format!("Expected ) in for loop, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::DO {
            return Err(format!("Missing 'do' in for loop, got {:?}", s.current_token()));
        }
        s.next_token();

        let statements = StmtSeq::new(s)?;

        if s.current_token() != Token::END {
            return Err(format!("Missing 'end' in for loop, got {:?}", s.current_token()));
        }
        s.next_token();

        Ok(Box::new(Loop{assignment_id, assignment_expr, cond, increment, statements}))
    }
}

impl PrettyPrint for Loop {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        write_indent(f, level)?;
        writeln!(f, "for ({} = {}; {}; {}) do", self.assignment_id, self.assignment_expr, self.cond, self.increment)?;

        self.statements.fmt_indented(f, level + 1)?;

        write_indent(f, level)?;
        writeln!(f, "end")
    }
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
