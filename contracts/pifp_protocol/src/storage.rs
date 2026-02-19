use soroban_sdk::{contracttype, Env};

use crate::types::Project;

/// Keys used for persistent contract storage.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Global auto-increment counter for project IDs.
    ProjectCount,
    /// Individual project keyed by its ID.
    Project(u64),
}

/// Atomically reads, increments, and stores the project counter.
/// Returns the ID to use for the *current* project (pre-increment value).
pub fn get_and_increment_project_id(env: &Env) -> u64 {
    let key = DataKey::ProjectCount;
    let current: u64 = env.storage().persistent().get(&key).unwrap_or(0);
    env.storage().persistent().set(&key, &(current + 1));
    current
}

/// Persist a project to contract storage.
pub fn save_project(env: &Env, project: &Project) {
    let key = DataKey::Project(project.id);
    env.storage().persistent().set(&key, project);
}

/// Load a project from contract storage.
/// Panics if the project does not exist.
pub fn load_project(env: &Env, id: u64) -> Project {
    let key = DataKey::Project(id);
    env.storage()
        .persistent()
        .get(&key)
        .expect("project not found")
}
