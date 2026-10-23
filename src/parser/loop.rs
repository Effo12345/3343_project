use std::{collections::HashMap, fmt};

use crate::{parser::{VarStack, utils::stdout_expect, variables::VarStackExt}, scanner::Scanner, token::Token};

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
        
        // the first part of the header assigns to an existing variable
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

        // the middle part is the loop condition
        let cond = Cond::new(s)?;

        if s.current_token() != Token::SEMICOLON {
            return Err(format!("Missing ; in for loop"));
        }
        s.next_token();

        // the last part is just an expression, not another assignment
        let increment = Expr::new(s)?;

        if s.current_token() != Token::RPAREN {
            return Err(format!("Expected ) in for loop, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::DO {
            return Err(format!("Missing 'do' in for loop, got {:?}", s.current_token()));
        }
        s.next_token();

        // parse the body after do, then consume its closing end
        let statements = StmtSeq::new(s)?;

        if s.current_token() != Token::END {
            return Err(format!("Missing 'end' in for loop, got {:?}", s.current_token()));
        }
        s.next_token();

        Ok(Box::new(Loop{assignment_id, assignment_expr, cond, increment, statements}))
    }

    fn validate_id(id: String, vars: &mut VarStack) -> Result<(), String> {
        let var = vars.resolve_var(&id);

        // the loop variable has to be declared before the loop
        let Some(_) = var else {
            return Err(format!("Undeclared variable '{}' used in for loop assignment", id));
        };

        Ok(())
    }

    pub fn validate(&self, vars: &mut VarStack) -> Result<(), String> {
        // validate the header before introducing the body scope
        Loop::validate_id(self.assignment_id.to_string(), vars)?;

        self.assignment_expr.validate(vars)?;
        self.cond.validate(vars)?;
        self.increment.validate(vars)?;
        
        // validate using separate loop scope
        vars.push(HashMap::new());
        self.statements.validate(vars)?;
        vars.pop();

        Ok(())
    }
    
    // look the index up fresh each time since vars changes between writes
    fn set_index(&self, vars: &mut VarStack, value: i32) {
        let idx = stdout_expect(vars.resolve_var(&self.assignment_id), "Undeclared variable used somehow");
        idx.var_type.set_int_or_default(&self.assignment_id, value);
    }

    pub fn execute(&self, vars: &mut VarStack, read_scanner: &mut Scanner) {
        let init = self.assignment_expr.execute(vars);
        self.set_index(vars, init);

        while self.cond.execute(vars) {
            // each loop run gets its own scope
            vars.push(HashMap::new());
            self.statements.execute(vars, read_scanner);
            vars.pop();

            let next = self.increment.execute(vars);
            self.set_index(vars, next);
        }
    }
}

// keep the header on one line and indent the body underneath it
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
