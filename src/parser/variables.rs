use std::{cell::RefCell, collections::HashMap, rc::Rc, thread::Scope};

use super::utils::exit_with_msg;

pub type ObjectType = Rc<RefCell<HashMap<String, i32>>>;

#[derive(PartialEq)]
pub enum VarType {
    Integer(i32),
    Object(Option<ObjectType>, String) // map, default key
}

impl VarType {
    // type-only placeholder for validation, no map allocated
    pub fn empty_object() -> Self {
        VarType::Object(None, String::new())
    }

    // fresh object for execution
    pub fn new_object() -> Self {
        VarType::Object(Some(ObjectType::default()), String::new())
    }

    // insert into an object's map using the caller's key
    pub fn insert_object_entry(&self, id: &str, key: String, value: i32) {
        match self {
            VarType::Object(Some(map), _) => {
                map.borrow_mut().insert(key, value);
            }
            _ => exit_with_msg(&format!("Attempted insertion into null object '{id})'"))
        }
    }

    // shared by "id = <expr>" and the loop header, objects set their default key
    // NOTE: assumes variable with name id has been declared already
    pub fn set_int_or_default(&mut self, id: &str, value: i32) {
        match self {
            VarType::Integer(val) => *val = value,
            VarType::Object(_, default_key) => {
                let key = default_key.clone();
                self.insert_object_entry(id, key, value);
            }
        }
    }
}

// pack into a struct in case we need more variable info later
pub struct ScopedVar {
    pub(crate) var_type: VarType
}

// one map per scope, with the innermost scope at the end
// searching backwards lets inner declarations shadow outer ones
pub type VarStack = Vec<HashMap<String, ScopedVar>>;

pub trait VarStackExt {
    fn resolve_var(&mut self, id: &str) -> Option<&mut ScopedVar>;
}

impl VarStackExt for VarStack {
    fn resolve_var(&mut self, id: &str) -> Option<&mut ScopedVar> {
        // search from the innermost scope so shadowed variables work
        self.iter_mut().rev().find_map(|map| map.get_mut(id))
    }
}