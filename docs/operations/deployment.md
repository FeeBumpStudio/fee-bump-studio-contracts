# Deployment

## Overview

Contract deployment process for FeeBumpStudio Contracts on Stellar networks.

## Prerequisites

| Requirement | Details |
|-------------|---------|
| **Stellar CLI** | `cargo install --locked stellar-cli` |
| **Rust + WASM target** | `rustup target add wasm32-unknown-unknown` |
| **Funded account** | Testnet: Friendbot; Mainnet: Exchange |
| **RPC endpoint** | Testnet: public; Mainnet: dedicated |

## Build

```bash
# Build WASM
make build
# or
stellar contract build
```

Output: `target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm`

Verify size:
```bash
ls -lh target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm
# Expected: ~100-200 KB
```

## Deploy

### Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm \
  --source admin \
  --network testnet \
  --rpc-url https://soroban-testnet.stellar.org
```

### Mainnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm \
  --source admin \
  --network mainnet \
  --rpc-url https://rpc.stellar.org/your-project-id
```

## Initialize

```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source admin \
  --network testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  -- initialize --admin <ADMIN_ADDRESS>
```

## Verify

```bash
# Contract info
stellar contract info --id CONTRACT_ID --network testnet

# Test record/read
stellar contract invoke \
  --id CONTRACT_ID \
  --source actor \
  --network testnet \
  -- record --actor <ACTOR_ADDRESS> --reference "TX-abc-001"

stellar contract invoke \
  --id CONTRACT_ID \
  --source actor \
  --network testnet \
  -- read --actor <ACTOR_ADDRESS>
```

## Network-Specific IDs

| Network | Contract ID | Deployment |
|---------|-------------|------------|
| Testnet | `CABC...TESTNET` | Separate |
| Mainnet | `CXYZ...MAINNET` | Separate |

## Generate Bindings

```bash
stellar contract bindings typescript \
  --contract-id CONTRACT_ID \
  --network testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --output-dir ./generated/contract
```

## Checklist

- [ ] WASM builds successfully
- [ ] Tests pass (`cargo test`)
- [ ] Admin account funded
- [ ] Contract deployed
- [ ] Contract initialized
- [ ] Integration configs updated
- [ ] TypeScript bindings generated
- [ ] Test record/read works

## Related Documentation

- [Testing](../contract-guide/testing.md)
- [Environment Variables](../configuration/environment-variables.md)
- [Upgradeability](upgradeability.md)