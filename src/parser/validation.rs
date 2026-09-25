use std::{collections::HashMap};

#[derive(PartialEq)]
pub enum VarType {
    Integer,
    Object
}

// pack into a struct in case we need more variable info later
pub struct ScopedVar {
    pub(crate) var_type: VarType
}

// one map per scope, with the innermost scope at the end
// searching backwards lets inner declarations shadow outer ones
pub type VarStack = Vec<HashMap<String, ScopedVar>>;