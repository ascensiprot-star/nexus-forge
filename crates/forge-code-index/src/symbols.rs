use std::collections::HashMap;
use std::sync::RwLock;

use forge_core::traits::{Symbol, SymbolReference};

pub struct SymbolTable {
    symbols: RwLock<HashMap<String, Vec<Symbol>>>,
    by_file: RwLock<HashMap<String, Vec<Symbol>>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: RwLock::new(HashMap::new()),
            by_file: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert(&self, symbol: Symbol) {
        {
            let mut symbols = self.symbols.write().unwrap();
            symbols
                .entry(symbol.name.clone())
                .or_default()
                .push(symbol.clone());
        }
        {
            let mut by_file = self.by_file.write().unwrap();
            by_file
                .entry(symbol.file_path.clone())
                .or_default()
                .push(symbol);
        }
    }

    pub fn get_by_name(&self, name: &str) -> Vec<Symbol> {
        self.symbols
            .read()
            .unwrap()
            .get(name)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_by_file(&self, file_path: &str) -> Vec<Symbol> {
        self.by_file
            .read()
            .unwrap()
            .get(file_path)
            .cloned()
            .unwrap_or_default()
    }

    pub fn find_references(&self, symbol_name: &str) -> Vec<SymbolReference> {
        self.symbols
            .read()
            .unwrap()
            .get(symbol_name)
            .map(|symbols| {
                symbols
                    .iter()
                    .map(|s| SymbolReference {
                        file_path: s.file_path.clone(),
                        line: s.start_line,
                        column: 0,
                        context: s.signature.clone().unwrap_or_default(),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn clear_file(&self, file_path: &str) {
        {
            let mut by_file = self.by_file.write().unwrap();
            by_file.remove(file_path);
        }
        {
            let mut symbols = self.symbols.write().unwrap();
            for symbols_list in symbols.values_mut() {
                symbols_list.retain(|s| s.file_path != file_path);
            }
            symbols.retain(|_, v| !v.is_empty());
        }
    }

    pub fn total_symbols(&self) -> usize {
        self.symbols.read().unwrap().values().map(|v| v.len()).sum()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
