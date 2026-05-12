use forge_core::error::ForgeResult;
use forge_core::types::{MemoryLayer, ProjectId};

pub struct MemoryCompactor;

impl MemoryCompactor {
    pub fn should_compact(item_count: usize, max_items: usize) -> bool {
        item_count > max_items
    }

    pub fn items_to_archive(item_count: usize, max_items: usize) -> usize {
        if item_count > max_items {
            item_count - max_items
        } else {
            0
        }
    }
}
