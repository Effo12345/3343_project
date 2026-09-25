use std::{collections::HashMap, fmt};

use crate::{parser::VarStack, scanner::Scanner, token::Token};

use super::{write_indent, Cond, PrettyPrint, StmtSeq};

pub enum If {
    If(Box<Cond>, Box<StmtSeq>),
    IfElse(Box<Cond>, Box<StmtSeq>, Box<StmtSeq>) // cond, true seq, else seq
}

impl If {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        // assume if to start, handle else if it happens
        // also if token should be guaranteed by stmt logic, but check anyway
        if s.current_token() != Token::IF {
            return Err(format!("Expected if statement, got {:?}", s.current_token()));
        }
        s.next_token();

        let cond = Cond::new(s)?;

        if s.current_token() != Token::THEN {
            return Err(format!("Expected 'then' in if statement, got {:?}", s.current_token()));
        }
        s.next_token();

        let stmt_seq = StmtSeq::new(s)?;

        let mut else_seq: Option<Box<StmtSeq>> = None;
        if s.current_token() == Token::ELSE {
            // consume else then parse else-case code
            s.next_token();
            else_seq = Some(StmtSeq::new(s)?);
        }

        if s.current_token() != Token::END {
            return Err(format!("Missing 'end' in if statement, got {:?}", s.current_token()));
        }
        s.next_token();

        Ok(Box::new(
            match else_seq {
                Some(else_stmt_seq) => If::IfElse(cond, stmt_seq, else_stmt_seq),
                None => If::If(cond, stmt_seq)
            }
        ))
    }

    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        match self {
            If::If(cond, stmt_seq) => {
                cond.validate(vars)?;

                // push new scope
                vars.push(HashMap::new());

                stmt_seq.validate(vars)?;

                // pop if block scope
                vars.pop();
            }
            If::IfElse(cond, if_seq, else_seq) => {
                cond.validate(vars)?;

                // eval with if scope
                vars.push(HashMap::new());
                if_seq.validate(vars)?;
                vars.pop();

                // eval with else scope
                vars.push(HashMap::new());
                else_seq.validate(vars)?;
                vars.pop();
            }
        };

        Ok(())
    }
}

impl PrettyPrint for If {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        match self {
            If::If(cond, stmt_seq) => {
                write_indent(f, level)?;
                writeln!(f, "if {cond} then")?;

                stmt_seq.fmt_indented(f, level + 1)?;

                write_indent(f, level)?;
                writeln!(f, "end")
            },
            If::IfElse(cond, true_seq, else_seq) => {
                write_indent(f, level)?;
                writeln!(f, "if {cond} then")?;

                true_seq.fmt_indented(f, level + 1)?;

                write_indent(f, level)?;
                writeln!(f, "else")?;

                else_seq.fmt_indented(f, level + 1)?;

                write_indent(f, level)?;
                writeln!(f, "end")
            }
        }
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
