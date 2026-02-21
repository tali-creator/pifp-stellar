use soroban_sdk::{contracttype, Address, Env};

use crate::types::{Project, Role};

/// Keys used for persistent contract storage.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Global auto-increment counter for project IDs.
    ProjectCount,
    /// Individual project keyed by its ID.
    Project(u64),
    /// Trusted oracle/verifier address.
    OracleKey,
    /// Initialized flag.
    AdminKey,
    /// Role management key.
    Role(Address, Role),
}

pub fn has_role(env: &Env, address: &Address, role: Role) -> bool {
    let key = DataKey::Role(address.clone(), role);
    env.storage().persistent().has(&key)
}

pub fn set_role(env: &Env, address: &Address, role: Role, authorized: bool) {
    let key = DataKey::Role(address.clone(), role);
    if authorized {
        env.storage().persistent().set(&key, &());
    } else {
        env.storage().persistent().remove(&key);
    }
}

pub fn set_admin(env: &Env, admin: &Address) {
    if env.storage().persistent().has(&DataKey::AdminKey) {
        panic!("already initialized");
    }
    env.storage().persistent().set(&DataKey::AdminKey, admin);
    set_role(env, admin, Role::Admin, true);
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

/// Store the trusted oracle address.
pub fn set_oracle(env: &Env, oracle: &Address) {
    env.storage().persistent().set(&DataKey::OracleKey, oracle);
}
