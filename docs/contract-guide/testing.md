# Testing

## Overview

Contract tests use Soroban's `testutils` feature for deterministic host-side testing. Tests run in a simulated Soroban environment without network dependencies.

## Test Configuration

### Cargo.toml

```toml
[dev-dependencies]
soroban-sdk = { version = "28", features = ["testutils"] }
```

### Test Module

```rust
// tests/contract_test.rs
use soroban_sdk::{Env, String, Address, testutils::Address as _};
use fee_bump_studio_contracts::{FeeBumpStudioContract, FeeBumpStudioContractClient};

#[test]
fn record_and_read_roundtrip() {
    let env = Env::default();
    env.mock_all_auths();  // Auto-authorize all calls
    
    let admin = Address::generate(&env);
    let actor = Address::generate(&env);
    
    let contract_id = env.register(FeeBumpStudioContract, ());
    let client = FeeBumpStudioContractClient::new(&env, &contract_id);
    
    client.initialize(&admin);
    client.record(&actor, &String::from_str(&env, "TX-abc-001"));
    
    assert_eq!(
        client.read(&actor),
        Some(String::from_str(&env, "TX-abc-001"))
    );
}
```

## Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test record_and_read_roundtrip

# With output
cargo test -- --nocapture

# Verbose
cargo test -- --test-threads=1
```

## Test Patterns

### 1. Basic Roundtrip

```rust
#[test]
fn record_and_read_roundtrip() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let actor = Address::generate(&env);
    
    let contract_id = env.register(FeeBumpStudioContract, ());
    let client = FeeBumpStudioContractClient::new(&env, &contract_id);
    
    client.initialize(&admin);
    client.record(&actor, &String::from_str(&env, "TX-abc-001"));
    
    assert_eq!(
        client.read(&actor),
        Some(String::from_str(&env, "TX-abc-001"))
    );
}
```

### 2. Unknown Actor Returns None

```rust
#[test]
fn read_unknown_actor_returns_none() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let contract_id = env.register(FeeBumpStudioContract, ());
    let client = FeeBumpStudioContractClient::new(&env, &contract_id);
    
    client.initialize(&admin);
    
    assert_eq!(
        client.read(&Address::generate(&env)),
        Option::<String>::None
    );
}
```

### 3. Overwrite Behavior

```rust
#[test]
fn record_overwrites_previous_reference() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let actor = Address::generate(&env);
    
    let contract_id = env.register(FeeBumpStudioContract, ());
    let client = FeeBumpStudioContractClient::new(&env, &contract_id);
    
    client.initialize(&admin);
    client.record(&actor, &String::from_str(&env, "old"));
    client.record(&actor, &String::from_str(&env, "ref-2"));
    
    assert_eq!(
        client.read(&actor),
        Some(String::from_str(&env, "ref-2"))
    );
}
```

### 4. Authorization Failure (Expected)

```rust
#[test]
#[should_panic(expected = "AuthFailed")]
fn record_without_auth_fails() {
    let env = Env::default();
    // NO mock_all_auths()
    
    let admin = Address::generate(&env);
    let actor = Address::generate(&env);
    
    let contract_id = env.register(FeeBumpStudioContract, ());
    let client = FeeBumpStudioContractClient::new(&env, &contract_id);
    
    client.initialize(&admin);
    // This will panic: actor didn't authorize
    client.record(&actor, &String::from_str(&env, "TX-abc-001"));
}
```

## Test Utilities

### `mock_all_auths()`

Automatically authorizes all `require_auth()` calls in tests:

```rust
let env = Env::default();
env.mock_all_auths();  // All auths pass
```

### `Address::generate(&env)`

Generates a deterministic test address:

```rust
let admin = Address::generate(&env);  // Unique per call
```

### `env.register(Contract, ())`

Registers contract in test environment:

```rust
let contract_id = env.register(FeeBumpStudioContract, ());
// Or with constructor args:
let contract_id = env.register(FeeBumpStudioContract, (&admin,));
```

### `FeeBumpStudioContractClient`

Type-safe client for testing:

```rust
let client = FeeBumpStudioContractClient::new(&env, &contract_id);
client.initialize(&admin);
client.record(&actor, &reference);
let result = client.read(&actor);
```

## Test Organization

```
tests/
├── contract_test.rs          # Main contract behavior tests
├── auth_test.rs              # Authorization edge cases
├── storage_test.rs           # Storage boundary tests
└── integration_test.rs       # Multi-contract scenarios (future)
```

## CI Integration

```yaml
# .github/workflows/ci.yml
- name: Test
  run: cargo test
```

## Coverage

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Run with coverage
cargo llvm-cov --html
```

## Best Practices

| Practice | Reason |
|----------|--------|
| `mock_all_auths()` in tests | Focus on logic, not auth setup |
| Test error cases | Verify proper failures |
| Use `Address::generate()` | Deterministic, unique addresses |
| One assert per test | Clear failure messages |
| Test storage boundaries | Verify persistence behavior |

## Related Documentation

- [Interface](interface.md)
- [Authorization](authorization.md)
- [Deployment](deployment.md)
- [CI Workflow](../configuration/ci-cd.md)