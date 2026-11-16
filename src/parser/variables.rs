use std::{cell::{Cell, RefCell}, collections::HashMap, ops::Deref, rc::Rc};

use super::utils::exit_with_msg;

// running count of reachable objects, printed whenever it changes
thread_local! {
    static REACHABLE: Cell<usize> = const { Cell::new(0) };
}

fn update_reachable(delta: isize) {
    let n = REACHABLE.get().wrapping_add_signed(delta);
    REACHABLE.set(n);
    println!("gc:{n}");
}

// the Rc around this does the actual ref counting, this just reports when
// an object is made and when its last reference goes away
#[derive(PartialEq)]
pub struct HeapObj(RefCell<HashMap<String, i32>>);

impl HeapObj {
    pub fn new(key: String, value: i32) -> ObjectType {
        update_reachable(1);
        Rc::new(HeapObj(RefCell::new(HashMap::from([(key, value)]))))
    }
}

impl Drop for HeapObj {
    fn drop(&mut self) {
        update_reachable(-1);
    }
}

// lets callers keep using borrow/borrow_mut directly
impl Deref for HeapObj {
    type Target = RefCell<HashMap<String, i32>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type ObjectType = Rc<HeapObj>;

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