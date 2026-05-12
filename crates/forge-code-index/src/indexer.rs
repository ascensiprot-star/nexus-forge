use std::collections::HashMap;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use forge_core::error::ForgeResult;
use forge_core::traits::{CodeIndexService, Symbol, SymbolKind, SymbolReference};

use crate::symbols::SymbolTable;

pub struct CodeIndexer {
    root_path: PathBuf,
    symbol_table: SymbolTable,
    file_hashes: HashMap<PathBuf, String>,
}

impl CodeIndexer {
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            root_path,
            symbol_table: SymbolTable::new(),
            file_hashes: HashMap::new(),
        }
    }

    fn walk_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        Self::walk_dir(&self.root_path, &mut files);
        files
    }

    fn walk_dir(dir: &Path, files: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with('.')
                    || name == "node_modules"
                    || name == "target"
                    || name == "__pycache__"
                    || name == "venv"
                {
                    continue;
                }
                Self::walk_dir(&path, files);
            } else if Self::is_supported_file(&path) {
                files.push(path);
            }
        }
    }

    fn is_supported_file(path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| {
                matches!(
                    ext,
                    "rs" | "py"
                        | "ts"
                        | "tsx"
                        | "js"
                        | "jsx"
                        | "go"
                        | "java"
                        | "cs"
                        | "rb"
                        | "php"
                        | "swift"
                        | "kt"
                )
            })
            .unwrap_or(false)
    }

    fn compute_file_hash(path: &Path) -> ForgeResult<String> {
        let content = std::fs::read(path)?;
        use sha2::{Digest, Sha256};
        let hash = Sha256::digest(&content);
        Ok(format!("{hash:x}"))
    }
}

#[async_trait]
impl CodeIndexService for CodeIndexer {
    async fn index_project(&self, root_path: &str) -> ForgeResult<()> {
        let files = self.walk_files();
        tracing::info!("indexing {} files from {}", files.len(), root_path);

        for file in &files {
            self.index_file(file.to_str().unwrap_or("")).await?;
        }

        Ok(())
    }

    async fn index_file(&self, file_path: &str) -> ForgeResult<()> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(path)?;
        let language = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let symbols = crate::parser::extract_symbols(&content, language, file_path);

        for symbol in symbols {
            self.symbol_table.insert(symbol);
        }

        tracing::debug!("indexed file: {file_path}");
        Ok(())
    }

    async fn get_symbols(&self, file_path: &str) -> ForgeResult<Vec<Symbol>> {
        Ok(self.symbol_table.get_by_file(file_path))
    }

    async fn find_references(&self, symbol_name: &str) -> ForgeResult<Vec<SymbolReference>> {
        Ok(self.symbol_table.find_references(symbol_name))
    }

    async fn get_dependencies(&self, file_path: &str) -> ForgeResult<Vec<String>> {
        let content = std::fs::read_to_string(file_path)?;
        Ok(crate::dependencies::extract_imports(&content, file_path))
    }
}
