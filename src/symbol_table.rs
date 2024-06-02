use std::{collections::HashMap, vec};

use crate::ast::ASTNode;

type SymbolName = String;

#[derive(Debug)]
pub struct SymbolTable {
    pub scoped_table: Vec<HashMap<SymbolName, ASTNode>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            scoped_table: vec![(HashMap::new())],
        }
    }

    pub fn get(&self, key: &str) -> Option<&ASTNode> {
        for table in self.scoped_table.iter().rev() {
            if let Some(symbol) = table.get(key) {
                return Some(symbol);
            }
        }

        None
    }

    pub fn set(&mut self, key: &str, symbol: ASTNode) {
        if let Some(scope) = self.scoped_table.last_mut() {
            scope.insert(String::from(key), symbol);
        }
    }

    pub fn push_scope(&mut self) {
        self.scoped_table.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scoped_table.pop();
    }
}
