use forge_core::error::ForgeResult;
use std::path::PathBuf;

pub struct FileWatcher {
    root_path: PathBuf,
}

impl FileWatcher {
    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }

    pub fn root(&self) -> &PathBuf {
        &self.root_path
    }
}
