# First Run

## Run Tests

```bash
cargo test
```

Output:

```text
   Compiling fee-bump-studio-contracts v0.1.0
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.56s
     Running unittests src/lib.rs
     Running tests/contract_test.rs

test record_and_read_roundtrip ... ok
test read_unknown_actor_returns_none ... ok
test record_overwrites_previous_reference ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

## Build for Deployment

```bash
make build
# or
stellar contract build
```

Output:

```text
   Compiling fee-bump-studio-contracts v0.1.0
    Finished `release` profile [optimized + debuginfo] target(s) in 5.23s
```

WASM location: `target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm`

## Verify WASM

```bash
# Check WASM size
ls -lh target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm

# Should be ~100-200 KB optimized
```

## Deploy to Testnet (Optional)

```bash
# Requires funded admin account
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm \
  --source admin \
  --network testnet \
  --rpc-url https://soroban-testnet.stellar.org
```

Output includes contract ID (starts with `C`):
```
Contract deployed successfully!
Contract ID: CABC123...YOUR_CONTRACT_ID
```

## Initialize Contract

```bash
stellar contract invoke \
  --id CABC123...YOUR_CONTRACT_ID \
  --source admin \
  --network testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  -- initialize --admin <ADMIN_ADDRESS>
```

## Verify Deployment

```bash
# Read contract metadata
stellar contract info --id CABC123... --network testnet

# Test read (should return None for unknown actor)
stellar contract invoke \
  --id CABC123... \
  --source admin \
  --network testnet \
  -- read --actor <RANDOM_ADDRESS>
```

## Record a Reference

```bash
stellar contract invoke \
  --id CABC123... \
  --source actor \
  --network testnet \
  -- record --actor <ACTOR_ADDRESS> --reference "TX-abc-001"
```

## Read Back Reference

```bash
stellar contract invoke \
  --id CABC123... \
  --source actor \
  --network testnet \
  -- read --actor <ACTOR_ADDRESS>
# Returns: "TX-abc-001"
```

## Common First-Run Issues

| Issue | Solution |
|-------|----------|
| `linker` not found | Install `lld`: `rustup component add llvm-tools-preview` |
| WASM too large | Ensure `opt-level = "z"` and `panic = "abort"` in release profile |
| `stellar` command not found | Install stellar CLI: `cargo install --locked stellar-cli` |
| RPC rate limited | Use dedicated RPC or add delays |
| Insufficient balance | Fund admin/actor accounts via Friendbot (Testnet) |

## Next Steps

- Explore [Contract Architecture](../contract-guide/architecture.md)
- Read [Interface](../contract-guide/interface.md)
- Learn [Testing](../contract-guide/testing.md)
- See [Deployment](../contract-guide/deployment.md)