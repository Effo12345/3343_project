use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
            _ => exit_with_msg(&format!("Attempted insertion into null object '{id}'"))
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

// one frame per active procedure call, with the running call at the top
// the first frame belongs to the main body and its first scope holds the globals
pub type CallStack = Vec<VarStack>;

pub trait CallStackExt {
    fn resolve_var(&mut self, id: &str) -> Option<&mut ScopedVar>;
    fn curr_scope(&mut self) -> Option<&mut HashMap<String, ScopedVar>>;
    fn push_scope(&mut self);
    fn pop_scope(&mut self);
}

impl CallStackExt for CallStack {
    fn resolve_var(&mut self, id: &str) -> Option<&mut ScopedVar> {
        let (top, below) = self.split_last_mut()?;
        // globals are the first scope in the first frame
        let globals = below.first_mut().and_then(|frame| frame.first_mut());

        // search from the innermost scope so shadowed variables work
        // only globals are visible past the current frame
        // chain iterates top first, then globals
        top.iter_mut().rev().chain(globals).find_map(|map| map.get_mut(id))
    }

    fn curr_scope(&mut self) -> Option<&mut HashMap<String, ScopedVar>> {
        // newest scope in the most recent frame
        self.last_mut()?.last_mut()
    }

    // scopes always go on the running call's frame
    fn push_scope(&mut self) {
        if let Some(frame) = self.last_mut() {
            frame.push(HashMap::new());
        }
    }

    fn pop_scope(&mut self) {
        if let Some(frame) = self.last_mut() {
            frame.pop();
        }
    }
}