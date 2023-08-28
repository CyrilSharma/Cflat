use std::collections::HashMap;
use super::ast::*;
pub trait Symbol {}
#[derive(Debug)]
pub struct VSymbol { 
    pub id: u32, 
    pub kind: Kind 
}
impl Symbol for VSymbol {}

#[derive(Debug)]
pub struct FSymbol {
    pub id: u32, 
    pub kind: Kind, 
    pub args: Vec<Kind> 
}
impl Symbol for FSymbol {}

pub struct SymbolTable<T: Symbol> {
    stk: Vec<HashMap<String, T>>,
    pub count: u32
}

impl SymbolTable<VSymbol> {
    pub fn insert(&mut self, name: &str, kind: Kind) -> u32 {
        let table = self.stk.last_mut().unwrap();
        table.insert(
            name.to_string(),
            VSymbol { id: self.count, kind }
        );
        self.count += 1;
        return self.count - 1;
    } 
}

impl SymbolTable<FSymbol> {
    pub fn insert(&mut self, func: &FunctionDeclaration) -> u32 {
        let table = self.stk.last_mut().unwrap();
        let kind = func.ret;
        let args = func.params.iter().map(|p| p.kind).collect();
        table.insert(
            func.name.to_string(), 
            FSymbol { id: self.count, kind, args}
        );
        self.count += 1;
        return self.count - 1;
    } 
}

impl<T: Symbol> SymbolTable<T> {
    pub fn new() -> Self {
        Self { stk: vec![HashMap::new()], count: 0 }
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