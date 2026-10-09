# Local Development

## Development Workflow

```bash
# 1. Make changes to src/lib.rs

# 2. Run tests
cargo test

# 3. Build for host
cargo build

# 4. Build for WASM (deployment)
make build
# or
stellar contract build
```

## Available Commands

| Command | Purpose |
|---------|---------|
| `cargo test` | Run all tests |
| `cargo build` | Build for host (testing) |
| `make build` | Build WASM for deployment |
| `make test` | Alias for `cargo test` |
| `stellar contract build` | Official build command |

## Development Environment

### Rust Toolchain

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Verify
rustc --version
cargo --version
```

### Stellar CLI

```bash
# Install
cargo install --locked stellar-cli

# Verify
stellar --version
```

### Optional: Watch Mode

```bash
# Auto-run tests on file changes
cargo install cargo-watch

# Usage
cargo watch -x test
```

## Code Style

### Rust

- **Edition 2021** — Modern Rust
- **`no_std`** — Required for Soroban contracts
- **Clippy** — Run `cargo clippy` before committing
- **Format** — Run `cargo fmt` before committing

### Soroban Patterns

```rust
// Good: Explicit types
use soroban_sdk::{Env, Address, String};

pub fn record(env: Env, actor: Address, reference: String) {
    actor.require_auth();
    env.storage().persistent().set(&actor, &reference);
}

// Good: Error handling with expect/unwrap in tests
#[test]
fn test_example() {
    let env = Env::default();
    env.mock_all_auths();
    // ...
}
```

## Testing Locally

```bash
# Run all tests
cargo test

# Run specific test
cargo test record_and_read_roundtrip

# Verbose output
cargo test -- --nocapture

# Single-threaded (for debugging)
cargo test -- --test-threads=1
```

## Debugging

### Test Debugging

```rust
// Add debug output in tests
#[test]
fn debug_test() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    eprintln!("Admin: {:?}", admin);  // Print to stderr
    
    // ...
}
```

### WASM Inspection

```bash
# Check WASM size
ls -lh target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm

# Disassemble (requires wasm-tools)
wasm-tools print target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm
```

## Common Issues

| Issue | Solution |
|-------|----------|
| `linker` not found | `rustup component add llvm-tools-preview` |
| WASM too large | Verify release profile: `opt-level = "z"` |
| `stellar` command not found | `cargo install --locked stellar-cli` |
| Test auth failures | Add `env.mock_all_auths()` |
| Slow compilation | Use `cargo watch` for incremental |

## IDE Setup

### VS Code

Recommended extensions:
- **rust-analyzer** — Rust language server
- **Even Better TOML** — Cargo.toml editing
- **Error Lens** — Inline errors

Settings:
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

## Pre-commit Checks

```bash
# Install husky equivalent for Rust
cargo install cargo-husky
cargo husky install

# Add pre-commit hook
cargo husky add pre-commit "cargo fmt --check && cargo clippy && cargo test"
```

## Useful Commands

```bash
# Check formatting
cargo fmt --check

# Lint
cargo clippy

# Check dependencies
cargo tree

# Audit dependencies
cargo audit

# Update dependencies
cargo update

# Clean build
cargo clean && cargo build
```

## Related Documentation

- [Testing](../contract-guide/testing.md)
- [Building](building.md)
- [Contributing](contributing.md)
- [Deployment](../contract-guide/deployment.md)