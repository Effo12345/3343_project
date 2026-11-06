use std::fmt;

use crate::{parser::{CallStack, FunctionMap, VarType, utils::{exit_with_msg, stdout_expect}, variables::CallStackExt}, scanner::Scanner, token::Token};

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

    pub fn validate(&self, vars: &mut CallStack, funcs: &FunctionMap) -> Result<(), String> {
        // every function is already in the map by now, so forward calls work
        let Some(function) = funcs.get(&self.id) else {
            return Err(format!("Call to undeclared procedure '{}'", self.id));
        };

        let args = self.args.ids();
        if args.len() != function.param_count() {
            return Err(format!("Procedure {} expects {} argument(s) but was called with {}", self.id, function.param_count(), args.len()));
        }

        // the arguments must be declared in a visible scope
        for arg in args {
            if vars.resolve_var(arg).is_none() {
                return Err(format!("Undeclared variable '{arg}' used as argument to procedure {}", self.id));
            }
        }

        Ok(())
    }

    // call by sharing, the formal gets its own variable pointing to the same object
    fn share_arg(&self, arg: &str, vars: &mut CallStack) -> VarType {
        match &stdout_expect(vars.resolve_var(arg), "Undeclared variable used somehow").var_type {
            VarType::Object(map, key) => VarType::Object(map.clone(), key.clone()),
            _ => exit_with_msg(&format!("Non-object variable '{arg}' used as argument to procedure {}", self.id))
        }
    }

    pub fn execute(&self, vars: &mut CallStack, read_scanner: &mut Scanner, funcs: &FunctionMap) {
        let function = stdout_expect(funcs.get(&self.id), "Undeclared procedure called somehow");

        // resolve the arguments now while the caller's frame is still on top
        let args = self.args.ids().into_iter().map(|arg| self.share_arg(arg, vars)).collect();

        function.execute(args, vars, read_scanner, funcs);
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
