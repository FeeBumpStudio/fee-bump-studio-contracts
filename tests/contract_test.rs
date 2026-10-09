use soroban_sdk::{Env, String, Address, testutils::Address as _};

use fee_bump_studio_contracts::{FeeBumpStudioContract, FeeBumpStudioContractClient};

#[test]
fn record_and_read_roundtrip() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let actor = Address::generate(&env);

    let contract_id = env.register(FeeBumpStudioContract, ());
    let contract = FeeBumpStudioContractClient::new(&env, &contract_id);

    // Initialize the contract
    contract.initialize(&admin);

    let reference = String::from_str(&env, "TX-abc-001");

    contract.record(&actor, &reference);
    assert_eq!(
        contract.read(&actor),
        Some(String::from_str(&env, "TX-abc-001"))
    );
}

#[test]
fn read_unknown_actor_returns_none() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);

    let contract_id = env.register(FeeBumpStudioContract, ());
    let contract = FeeBumpStudioContractClient::new(&env, &contract_id);

    // Initialize the contract
    contract.initialize(&admin);

    assert_eq!(
        contract.read(&Address::generate(&env)),
        Option::<String>::None
    );
}

#[test]
fn record_overwrites_previous_reference() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let actor = Address::generate(&env);

    let contract_id = env.register(FeeBumpStudioContract, ());
    let contract = FeeBumpStudioContractClient::new(&env, &contract_id);

    // Initialize the contract
    contract.initialize(&admin);

    contract.record(&actor, &String::from_str(&env, "old"));
    contract.record(&actor, &String::from_str(&env, "ref-2"));

    assert_eq!(
        contract.read(&actor),
        Some(String::from_str(&env, "ref-2"))
    );
}