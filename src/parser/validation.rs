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

pub type VarStack = Vec<HashMap<String, ScopedVar>>;