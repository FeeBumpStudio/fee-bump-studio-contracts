# Prerequisites

## System Requirements

| Requirement | Version | Notes |
|-------------|---------|-------|
| **Rust** | ≥ 1.75 | Via `rustup` |
| **Cargo** | Included with Rust | Package manager |
| **Git** | ≥ 2.x | Version control |

## Stellar CLI

Required for contract deployment and interaction:

```bash
cargo install --locked stellar-cli
```

Verify installation:

```bash
stellar --version
# stellar-cli 22.x.x
```

## Soroban RPC Access

The contract interacts with Stellar via Soroban RPC:

| Network | Endpoint | Notes |
|---------|----------|-------|
| **Testnet** | `https://soroban-testnet.stellar.org` | Free, public, rate-limited |
| **Mainnet** | Dedicated provider required | Stellar RPC, QuickNode, self-hosted |

## Development Tools

| Tool | Purpose |
|------|---------|
| **VS Code + rust-analyzer** | Recommended IDE |
| **Stellar Laboratory** | Transaction inspection |
| **Stellar Expert** | Network explorer |
| **cargo-watch** | Auto-rebuild on changes |

```bash
cargo install cargo-watch
# Usage: cargo watch -x test
```

## Network Access

- **Testnet**: Free accounts via [Friendbot](https://laboratory.stellar.org/#account-creator?network=testnet)
- **Mainnet**: Real XLM required; acquire from exchange

## WASM Target

The contract compiles to WebAssembly:

```bash
rustup target add wasm32-unknown-unknown
```

## Optional: Docker

For consistent build environment:

```bash
docker pull stellar/quickstart:latest
# Or use the stellar CLI Docker image
```