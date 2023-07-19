use std::collections::HashMap;
use crate::ast::*;
pub trait Symbol {}

#[derive(Clone)]
pub struct VSymbol { 
    pub id: u32, 
    pub kind: Kind 
}
impl Symbol for VSymbol {}

#[derive(Clone)]
pub struct FSymbol {
    pub id: u32, 
    pub kind: Kind, 
    pub args: Vec<Kind> 
}
impl Symbol for FSymbol {}

pub struct SymbolTable<T: Symbol> {
    stk: Vec<HashMap<String, T>>
}

impl<T: Symbol> SymbolTable<T> {
    pub fn insert(&mut self, key: &str, sym: T) {
        let table = match self.stk.last_mut() {
            None => {
                self.stk.push(HashMap::new());
                &mut self.stk[0]
            },
            Some(s) => s
        };
        table.insert(key.to_string(), sym);
    }
    pub fn contains_key_in_scope(&mut self, key: &str) -> bool {
        if let Some(table) = self.stk.last() {
            return table.contains_key(key);
        }
        return false;
    }
    pub fn get(&mut self, key: &str) -> Option<&T> {
        for table in &mut self.stk.iter().rev() {
            match table.get(key) { 
                None    => (),
                Some(s) => return Some(s)
            }
        }
        return None;
    }
    pub fn scope_in(&mut self) {
        self.stk.push(HashMap::new());
    }
    pub fn scope_out(&mut self) {
        self.stk.pop();
    }
}