use std::{collections::{HashMap, HashSet}, fmt, rc::Rc};

use crate::{scanner::Scanner, token::Token};

use super::{write_indent, Parameters, PrettyPrint, StmtSeq};

// calls find their target here at runtime
// Rc lets the tree keep its own handle for printing, and nothing mutates a function after parsing
pub type FunctionMap = HashMap<String, Rc<Function>>;

pub struct Function {
    params: Box<Parameters>,
    body: Box<StmtSeq>
}

// the spot in the decl-seq where a function was defined
pub struct FunctionTree {
    id: String,
    function: Rc<Function>
}

impl FunctionTree {
    pub fn new(s: &mut Scanner, funcs: &mut FunctionMap) -> Result<Box<Self>, String> {
        // should be guaranteed by decl_seq logic but check just in case
        if s.current_token() != Token::PROCEDURE {
            return Err(format!("Expected procedure definition, got {:?}", s.current_token()));
        }
        s.next_token();

        let Token::ID(id) = s.current_token() else {
            return Err(format!("Expected procedure name but got {:?}", s.current_token()));
        };
        s.next_token();

        if s.current_token() != Token::LPAREN {
            return Err(format!("Expected ( in definition of procedure {id}, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::OBJECT {
            return Err(format!("Expected 'object' in parameters of procedure {id}, got {:?}", s.current_token()));
        }
        s.next_token();

        let params = Parameters::new(s)?;

        // HashSet insert returns false if the name was already there
        let mut seen = HashSet::new();
        for param in params.ids() {
            if !seen.insert(param) {
                return Err(format!("Duplicate formal parameter '{param}' in procedure {id}"));
            }
        }

        if s.current_token() != Token::RPAREN {
            return Err(format!("Expected ) in definition of procedure {id}, got {:?}", s.current_token()));
        }
        s.next_token();

        if s.current_token() != Token::IS {
            return Err(format!("Expected 'is' in definition of procedure {id}, got {:?}", s.current_token()));
        }
        s.next_token();

        // every function needs at least one statement
        if s.current_token() == Token::END {
            return Err(format!("Missing body in definition of procedure {id}"));
        }
        let body = StmtSeq::new(s)?;

        if s.current_token() != Token::END {
            return Err(format!("Expected 'end' of procedure {id} but got {:?}", s.current_token()));
        }
        s.next_token();

        let function = Rc::new(Function{params, body});

        // insert returns the old value if this name was already defined
        match funcs.insert(id.clone(), Rc::clone(&function)) {
            Some(_) => Err(format!("Procedure {id} declared multiple times")),
            None => Ok(Box::new(FunctionTree{id, function}))
        }
    }
}

// indent the body, keeping the header and end aligned
impl PrettyPrint for FunctionTree {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        write_indent(f, level)?;
        writeln!(f, "procedure {}(object {}) is", self.id, self.function.params)?;

        self.function.body.fmt_indented(f, level + 1)?;

        write_indent(f, level)?;
        writeln!(f, "end")
    }
}

impl fmt::Display for FunctionTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(f, 0)
    }
}
