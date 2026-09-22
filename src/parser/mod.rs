use std::fmt;

mod assign;
mod cmpr;
mod cond;
mod decl;
mod decl_integer;
mod decl_obj;
mod decl_seq;
mod expr;
mod factor;
mod r#if;
mod r#loop;
mod print;
mod procedure;
mod read;
mod stmt;
mod stmt_seq;
mod term;

// Re-export the AST types through `parser`.
pub use assign::Assign;
pub use cmpr::Cmpr;
pub use cond::Cond;
pub use decl::Decl;
pub use decl_integer::DeclInteger;
pub use decl_obj::DeclObj;
pub use decl_seq::DeclSeq;
pub use expr::Expr;
pub use factor::Factor;
pub use r#if::If;
pub use r#loop::Loop;
pub use print::Print;
pub use procedure::Procedure;
pub use read::Read;
pub use stmt::Stmt;
pub use stmt_seq::StmtSeq;
pub use term::Term;

const INDENT_WIDTH: usize = 4;

pub(super) trait PrettyPrint {
    fn fmt_indented(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result;
}

pub(super) fn write_indent(f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
    write!(f, "{:width$}", "", width = level * INDENT_WIDTH)
}
