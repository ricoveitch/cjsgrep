use crate::ast::ASTNode;
use std::collections::HashMap;

type SymbolName = String;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub node: ASTNode,
    pub file_path: String,
}

#[derive(Debug)]
pub struct SymbolTable {
    pub table: HashMap<SymbolName, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            table: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Symbol> {
        self.table.get(key)
    }

    pub fn insert(&mut self, key: &str, symbol: Symbol) {
        self.table.insert(String::from(key), symbol);
    }
}

pub struct ProgramScope {
    scope: Vec<FileScope>,
}

impl ProgramScope {
    pub fn new() -> Self {
        Self { scope: Vec::new() }
    }

    pub fn current(&self) -> Option<&FileScope> {
        self.scope.last()
    }

    fn current_mut(&mut self) -> &mut FileScope {
        self.scope.last_mut().unwrap()
    }

    pub fn push_file(&mut self, path: &str) {
        self.scope.push(FileScope::new(path))
    }

    pub fn push_block(&mut self) {
        self.current_mut().push(ScopeKind::Block)
    }

    pub fn pop(&mut self) {
        let len = self.current_mut().pop();
        if len == 0 {
            self.scope.pop();
        }
    }

    pub fn find_symbol(&self, key: &str) -> Option<&Symbol> {
        match self.current() {
            Some(s) => s.find_symbol(key),
            None => None,
        }
    }

    pub fn insert_symbol(&mut self, key: &str, symbol: Symbol) {
        self.current_mut().insert_symbol(key, symbol)
    }
}

pub enum ScopeKind {
    Function,
    Block,
}

pub struct FileScope {
    pub file_path: String,
    global_table: SymbolTable,
    scope: Vec<Vec<SymbolTable>>,
}

impl FileScope {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_owned(),
            global_table: SymbolTable::new(),
            scope: vec![Vec::new()],
        }
    }

    pub fn push(&mut self, kind: ScopeKind) {
        match kind {
            ScopeKind::Block => self.scope.last_mut().unwrap().push(SymbolTable::new()),
            ScopeKind::Function => self.scope.push(Vec::new()),
        };
    }

    pub fn pop(&mut self) -> usize {
        self.scope.last_mut().unwrap().pop();
        if self.scope.last().unwrap().len() == 0 {
            self.scope.pop();
        }
        self.scope.len()
    }

    pub fn insert_symbol(&mut self, key: &str, symbol: Symbol) {
        if self.scope.last().unwrap().len() == 0 {
            return self.global_table.insert(key, symbol);
        }

        self.scope
            .last_mut()
            .unwrap()
            .last_mut()
            .unwrap()
            .insert(key, symbol)
    }

    pub fn find_symbol(&self, key: &str) -> Option<&Symbol> {
        for symbol_table in self.scope.last().unwrap().iter().rev() {
            if let Some(s) = symbol_table.get(key) {
                return Some(s);
            }
        }
        self.global_table.get(key)
    }
}
