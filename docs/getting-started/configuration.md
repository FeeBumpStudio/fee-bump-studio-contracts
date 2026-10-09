# Configuration

## Environment Variables

Copy the example file and customize it:

```bash
cp .env.example .env
```

### `.env.example` Reference

```ini
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=
STELLAR_CONTRACT_ID=
```

### Variable Details

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `STELLAR_NETWORK` | Yes | `testnet` | Target network: `testnet` or `mainnet` |
| `STELLAR_RPC_URL` | No | (public default) | Soroban RPC endpoint URL. Leave empty for public Testnet default. For production, use a dedicated RPC provider. |
| `STELLAR_CONTRACT_ID` | No* | — | Deployed contract ID for app/backend integration. Not hard-coded in contract. |

> *Required for integration with app/backend after deployment.

## Network Configuration

### Testnet (Development)

```ini
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_CONTRACT_ID=CABC...YOUR_TESTNET_CONTRACT
```

### Mainnet (Production)

```ini
STELLAR_NETWORK=mainnet
STELLAR_RPC_URL=https://your-dedicated-rpc-provider.com
STELLAR_CONTRACT_ID=CXYZ...YOUR_MAINNET_CONTRACT
```

> **Important**: Never commit `.env` or any file containing real credentials. The `.gitignore` excludes `.env*` files while keeping `.env.example`.

## Contract Identity

Contract addresses are **environment-specific**:

- **Testnet contract** ≠ **Mainnet contract**
- Deploy separately per network
- Set `STELLAR_CONTRACT_ID` to match the target network

The contract code itself does not contain network-specific addresses.

## Cargo Configuration

### `Cargo.toml` Key Settings

```toml
[profile.release]
opt-level = "z"       # Optimize for size
lto = true            # Link-time optimization
codegen-units = 1     # Single codegen unit
panic = "abort"       # Smaller WASM
```

### Features

| Feature | Purpose |
|---------|---------|
| `testutils` | Enables `soroban_sdk::testutils` for host testing |

## Verification

After configuration, verify the setup:

```bash
# Build for host (tests)
cargo build

# Build for WASM (deployment)
make build

# Run tests
cargo test
```