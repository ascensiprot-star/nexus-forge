use std::collections::{HashMap, HashSet, VecDeque};

use forge_core::error::{ForgeError, ForgeResult};
use forge_core::types::{Task, TaskId};

pub struct DependencyResolver;

impl DependencyResolver {
    pub fn has_cycle(dependencies: &HashMap<TaskId, HashSet<TaskId>>) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node in dependencies.keys() {
            if Self::dfs_cycle(node, dependencies, &mut visited, &mut rec_stack) {
                return true;
            }
        }

        false
    }

    fn dfs_cycle(
        node: &TaskId,
        dependencies: &HashMap<TaskId, HashSet<TaskId>>,
        visited: &mut HashSet<TaskId>,
        rec_stack: &mut HashSet<TaskId>,
    ) -> bool {
        if rec_stack.contains(node) {
            return true;
        }
        if visited.contains(node) {
            return false;
        }

        visited.insert(node.clone());
        rec_stack.insert(node.clone());

        if let Some(deps) = dependencies.get(node) {
            for dep in deps {
                if Self::dfs_cycle(dep, dependencies, visited, rec_stack) {
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        false
    }

    pub fn topological_sort(
        tasks: &HashMap<TaskId, Task>,
        dependencies: &HashMap<TaskId, HashSet<TaskId>>,
    ) -> ForgeResult<Vec<TaskId>> {
        let mut in_degree: HashMap<TaskId, usize> = HashMap::new();
        for task_id in tasks.keys() {
            in_degree.entry(task_id.clone()).or_insert(0);
        }
        for (task_id, deps) in dependencies {
            *in_degree.entry(task_id.clone()).or_insert(0) += deps.len();
        }

        let mut queue: VecDeque<TaskId> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(id, _)| id.clone())
            .collect();

        let mut result = Vec::new();

        while let Some(task_id) = queue.pop_front() {
            result.push(task_id.clone());

            for (dependent, deps) in dependencies.iter() {
                if deps.contains(&task_id) {
                    if let Some(deg) = in_degree.get_mut(dependent) {
                        *deg = deg.saturating_sub(1);
                        if *deg == 0 {
                            queue.push_back(dependent.clone());
                        }
                    }
                }
            }
        }

        if result.len() != tasks.len() {
            return Err(ForgeError::CyclicDependency {
                task_id: "unknown".to_string(),
            });
        }

        Ok(result)
    }

    pub fn critical_path(
        tasks: &HashMap<TaskId, Task>,
        dependencies: &HashMap<TaskId, HashSet<TaskId>>,
    ) -> Vec<TaskId> {
        let mut longest: HashMap<TaskId, (u32, Vec<TaskId>)> = HashMap::new();

        let topo = match Self::topological_sort(tasks, dependencies) {
            Ok(order) => order,
            Err(_) => return vec![],
        };

        for task_id in &topo {
            let effort = tasks
                .get(task_id)
                .and_then(|t| t.estimated_effort.as_ref())
                .map(|e| e.expected_minutes)
                .unwrap_or(1);

            let deps = dependencies.get(task_id);
            let (max_dep_len, mut max_path) = deps
                .map(|d| {
                    d.iter()
                        .filter_map(|dep_id| longest.get(dep_id))
                        .max_by_key(|(len, _)| *len)
                        .cloned()
                        .unwrap_or((0, vec![]))
                })
                .unwrap_or((0, vec![]));

            max_path.push(task_id.clone());
            longest.insert(task_id.clone(), (max_dep_len + effort, max_path));
        }

        longest
            .into_values()
            .max_by_key(|(len, _)| *len)
            .map(|(_, path)| path)
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_cycle() {
        let mut deps: HashMap<TaskId, HashSet<TaskId>> = HashMap::new();
        let mut set = HashSet::new();
        set.insert(TaskId::from_str("a"));
        deps.insert(TaskId::from_str("b"), set);

        assert!(!DependencyResolver::has_cycle(&deps));
    }

    #[test]
    fn test_cycle_detected() {
        let mut deps: HashMap<TaskId, HashSet<TaskId>> = HashMap::new();

        let mut ab = HashSet::new();
        ab.insert(TaskId::from_str("a"));
        deps.insert(TaskId::from_str("b"), ab);

        let mut ba = HashSet::new();
        ba.insert(TaskId::from_str("b"));
        deps.insert(TaskId::from_str("a"), ba);

        assert!(DependencyResolver::has_cycle(&deps));
    }
}
