# Deployment

## Build Process

### Development Build (Host)

```bash
cargo build
# Output: target/debug/fee_bump_studio_contracts
```

### Release Build (WASM)

```bash
make build
# or
stellar contract build
```

Output: `target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm`

### WASM Optimization

The release profile is tuned for minimal size:

```toml
[profile.release]
opt-level = "z"       # Maximum optimization
lto = true            # Link-time optimization
codegen-units = 1     # Single codegen unit
panic = "abort"       # No panic runtime
```

Expected WASM size: **~100-200 KB**

## Deploy to Testnet

### Prerequisites

1. **Stellar CLI installed**: `cargo install --locked stellar-cli`
2. **Funded admin account**: Get Testnet XLM from [Friendbot](https://laboratory.stellar.org/#account-creator?network=testnet)
3. **RPC endpoint**: `https://soroban-testnet.stellar.org` (or dedicated)

### Deploy Command

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm \
  --source admin \
  --network testnet \
  --rpc-url https://soroban-testnet.stellar.org
```

### Output

```
Contract deployed successfully!
Contract ID: CABC123...YOUR_CONTRACT_ID
```

Save this Contract ID for initialization and integration.

## Initialize Contract

```bash
stellar contract invoke \
  --id CABC123...YOUR_CONTRACT_ID \
  --source admin \
  --network testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  -- initialize --admin <ADMIN_ADDRESS>
```

### Verify Initialization

```bash
# Check contract info
stellar contract info --id CABC123... --network testnet

# Try reading (should return None for unknown actor)
stellar contract invoke \
  --id CABC123... \
  --source admin \
  --network testnet \
  -- read --actor <RANDOM_ADDRESS>
```

## Deploy to Mainnet

### Prerequisites

1. **Mainnet XLM** for fees (real cost)
2. **Dedicated RPC** (no public Mainnet RPC)
3. **Secure key management** (hardware wallet recommended)

### Deploy

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm \
  --source admin \
  --network mainnet \
  --rpc-url https://rpc.stellar.org/your-project-id
```

### Initialize

```bash
stellar contract invoke \
  --id CXYZ...MAINNET_CONTRACT_ID \
  --source admin \
  --network mainnet \
  --rpc-url https://rpc.stellar.org/your-project-id \
  -- initialize --admin <ADMIN_ADDRESS>
```

## Network-Specific Contract IDs

| Network | Contract ID | Deployment |
|---------|-------------|------------|
| **Testnet** | `CABC...TESTNET` | Separate deployment |
| **Mainnet** | `CXYZ...MAINNET` | Separate deployment |

**Never reuse contract IDs across networks.**

## Integration Configuration

After deployment, update integration configs:

### App (`.env.local`)

```ini
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_CONTRACT_ID=CABC123...YOUR_CONTRACT_ID
```

### Backend (`.env`)

```ini
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_CONTRACT_ID=CABC123...YOUR_CONTRACT_ID
```

## Generate TypeScript Bindings

```bash
stellar contract bindings typescript \
  --contract-id CABC123...YOUR_CONTRACT_ID \
  --network testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --output-dir ./generated/contract
```

Usage in app:

```typescript
import { FeeBumpStudioContract } from "./generated/contract";

const contract = new FeeBumpStudioContract({
  contractId: "CABC123...",
  networkPassphrase: Networks.TESTNET,
  rpcUrl: "https://soroban-testnet.stellar.org",
});
```

## Verify Deployment

### Test Record/Read

```bash
# Record a reference
stellar contract invoke \
  --id CABC123... \
  --source actor \
  --network testnet \
  -- record --actor <ACTOR_ADDRESS> --reference "TX-abc-001"

# Read it back
stellar contract invoke \
  --id CABC123... \
  --source actor \
  --network testnet \
  -- read --actor <ACTOR_ADDRESS>
# Returns: "TX-abc-001"
```

### Check Contract Info

```bash
stellar contract info --id CABC123... --network testnet
```

Output:
```
Contract ID: CABC123...
Network: testnet
WASM Hash: abc123...
Deployed: 2024-01-15T10:30:00Z
```

## Upgradeability (Future)

Current baseline is **immutable**. For production:

### Proxy Pattern

```rust
// Proxy contract
#[contract]
pub struct UpgradeableProxy;

#[contractimpl]
impl UpgradeableProxy {
    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        // Timelock + admin auth
        // env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
```

### Timelock

```rust
// Require delay before upgrade executes
const UPGRADE_DELAY: u64 = 7 * 24 * 60 * 60; // 7 days
```

## Monitoring Deployment

### Health Checks

```bash
# Verify contract responds
stellar contract invoke --id CONTRACT_ID -- read --actor <KNOWN_ACTOR>

# Check RPC connectivity
curl -X POST $RPC_URL -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'
```

### Metrics

| Metric | Target |
|--------|--------|
| Deploy success | 100% |
| Init success | 100% |
| WASM size | < 200 KB |
| Deploy cost | < 100 XLM (Testnet) |

## Rollback

If deployment fails:

1. **Don't initialize** — Contract unused, no state
2. **Redeploy** — New WASM, new contract ID
3. **Update configs** — Point to new contract ID

If initialization fails:

1. **Contract exists but uninitialized** — Safe to re-initialize
2. **Wrong admin** — Redeploy (cannot change admin in baseline)

## Checklist

- [ ] WASM builds successfully (`make build`)
- [ ] Tests pass (`cargo test`)
- [ ] Admin account funded
- [ ] RPC endpoint accessible
- [ ] Contract deployed
- [ ] Contract ID saved
- [ ] Contract initialized
- [ ] Integration configs updated
- [ ] TypeScript bindings generated
- [ ] Test record/read works
- [ ] Contract info verified

## Related Documentation

- [Testing](testing.md)
- [Operations - Monitoring](../operations/monitoring.md)
- [Upgradeability](../operations/upgradeability.md)
- [Network Settings](../configuration/network-settings.md)