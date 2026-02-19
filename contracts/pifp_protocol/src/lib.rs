#![no_std]

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

mod storage;
mod types;

#[cfg(test)]
mod test;

use storage::{get_and_increment_project_id, load_project, save_project};
pub use types::{Project, ProjectStatus};

#[contract]
pub struct PifpProtocol;

#[contractimpl]
impl PifpProtocol {
    /// Register a new funding project.
    ///
    /// - `creator` must authorize the call.
    /// - `goal` is the target funding amount (must be > 0).
    /// - `proof_hash` is a content hash (e.g. IPFS CID digest) representing proof artifacts.
    /// - `deadline` is a ledger timestamp by which the project must be completed (must be in the future).
    ///
    /// Returns the persisted `Project` with a unique auto-incremented `id`.
    pub fn register_project(
        env: Env,
        creator: Address,
        goal: i128,
        proof_hash: BytesN<32>,
        deadline: u64,
    ) -> Project {
        creator.require_auth();

        if goal <= 0 {
            panic!("goal must be positive");
        }

        if deadline <= env.ledger().timestamp() {
            panic!("deadline must be in the future");
        }

        let id = get_and_increment_project_id(&env);

        let project = Project {
            id,
            creator,
            goal,
            balance: 0,
            proof_hash,
            deadline,
            status: ProjectStatus::Funding,
        };

        save_project(&env, &project);

        project
    }

    /// Retrieve a project by its ID.
    ///
    /// Panics if the project does not exist.
    pub fn get_project(env: Env, id: u64) -> Project {
        load_project(&env, id)
    }

    /// Verify proof of impact and release funds.
    ///
    /// NOTE: This is a skeleton. A real implementation would:
    /// - Load the project from the registry via `get_project`
    /// - Verify the submitted proof against `proof_hash`
    /// - Enforce milestones / attestations / oracle signatures
    /// - Transfer/release funds and update balances / status
    pub fn verify_and_release(_env: Env, _project_id: u64, _submitted_proof_hash: BytesN<32>) {
        // TODO: implement verification logic and release mechanism.
    }
}
