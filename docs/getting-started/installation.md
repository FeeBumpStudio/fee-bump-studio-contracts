# Installation

## Clone the Repository

```bash
git clone https://github.com/FeeBumpStudio/fee-bump-studio-contracts.git
cd fee-bump-studio-contracts
```

## Build the Contract

```bash
cargo build
```

This compiles the contract for the host target (for testing).

## Build for WASM (Deployment)

```bash
make build
# or
stellar contract build
```

Output: `target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm`

## Verify Installation

Run the test suite:

```bash
cargo test
```

Expected output:

```text
running 3 tests
test record_and_read_roundtrip ... ok
test read_unknown_actor_returns_none ... ok
test record_overwrites_previous_reference ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

## Next Steps

- [Configuration](configuration.md) — Set up environment variables
- [First Run](first-run.md) — Run tests and deploy
- [Contract Architecture](../contract-guide/architecture.md) — Understand the design