#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, String};

#[contract]
pub struct FeeBumpStudioContract;

#[contractimpl]
impl FeeBumpStudioContract {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&String::from_str(&env, "admin"), &admin);
    }

    pub fn record(env: Env, actor: Address, reference: String) {
        actor.require_auth();
        env.storage().persistent().set(&actor, &reference);
    }

    pub fn read(env: Env, actor: Address) -> Option<String> {
        env.storage().persistent().get(&actor)
    }
}
