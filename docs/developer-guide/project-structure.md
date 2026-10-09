# Project Structure

```
fee-bump-studio-contracts/
├── src/                    # Contract source
│   └── lib.rs              # Main contract (3 functions)
├── tests/                  # Contract tests
│   └── contract_test.rs    # Host-side tests (3 tests)
├── docs/                   # Documentation source (MkDocs)
│   ├── index.md
│   ├── getting-started/
│   ├── contract-guide/
│   ├── developer-guide/
│   ├── api-reference/
│   ├── configuration/
│   ├── operations/
│   ├── security/
│   └── faq.md
├── target/                 # Build artifacts (gitignored)
│   ├── debug/              # Host build
│   └── wasm32-unknown-unknown/
│       └── release/        # WASM build
│           └── fee_bump_studio_contracts.wasm
├── .github/
│   ├── workflows/
│   │   ├── ci.yml          # CI: test, build, doc validation
│   │   └── docs.yml        # Documentation deployment
│   ├── ISSUE_TEMPLATE/
│   └── PULL_REQUEST_TEMPLATE/
├── .env.example            # Environment template
├── .gitignore
├── Cargo.toml              # Crate manifest
├── Cargo.lock              # Dependency lockfile
├── Makefile                # Build shortcuts
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
├── CODEOWNERS
└── mkdocs.yml              # MkDocs configuration
```

## Key Files

| File | Purpose |
|------|---------|
| `src/lib.rs` | Contract implementation (3 functions) |
| `tests/contract_test.rs` | Host-side tests |
| `Cargo.toml` | Dependencies, release profile |
| `Makefile` | `build` / `test` shortcuts |
| `mkdocs.yml` | Documentation site config |
| `.github/workflows/ci.yml` | CI pipeline |

## Source Structure

### `src/lib.rs`

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, String};

#[contract]
pub struct FeeBumpStudioContract;

#[contractimpl]
impl FeeBumpStudioContract {
    pub fn initialize(env: Env, admin: Address) { ... }
    pub fn record(env: Env, actor: Address, reference: String) { ... }
    pub fn read(env: Env, actor: Address) -> Option<String> { ... }
}
```

### `tests/contract_test.rs`

```rust
use soroban_sdk::{Env, String, Address, testutils::Address as _};
use fee_bump_studio_contracts::{FeeBumpStudioContract, FeeBumpStudioContractClient};

#[test]
fn record_and_read_roundtrip() { ... }
#[test]
fn read_unknown_actor_returns_none() { ... }
#[test]
fn record_overwrites_previous_reference() { ... }
```

## Build Artifacts

### Host Build (`cargo build`)

```
target/debug/
├── deps/
├── examples/
├── fee_bump_studio_contracts          # Host test binary
└── fee_bump_studio_contracts.d        # Dependency info
```

### WASM Build (`make build` / `stellar contract build`)

```
target/wasm32-unknown-unknown/release/
├── deps/
├── fee_bump_studio_contracts.wasm     # Deployable WASM
├── fee_bump_studio_contracts.wasm.d   # Dependency info
└── build-info.json                    # Build metadata
```

## Configuration Files

| File | Description |
|------|-------------|
| `Cargo.toml` | Crate manifest, dependencies, release profile |
| `Makefile` | `build` / `test` shortcuts |
| `.env.example` | Environment template |
| `.gitignore` | Excludes `target/`, `.env*`, `Cargo.lock` |

## Planned Structure (As Features Grow)

```
src/
├── lib.rs                    # Contract entry point
├── contract/
│   ├── mod.rs               # Main contract logic
│   ├── admin.rs             # Admin functions
│   ├── records.rs           # Record functions
│   └── events.rs            # Event definitions
├── storage/
│   ├── mod.rs               # Storage abstractions
│   ├── instance.rs          # Instance storage
│   └── persistent.rs        # Persistent storage
├── auth/
│   └── mod.rs               # Authorization helpers
├── error.rs                 # Custom error types
└── types.rs                 # Shared types

tests/
├── contract_test.rs         # Main behavior tests
├── auth_test.rs             # Authorization tests
├── storage_test.rs          # Storage tests
├── upgrade_test.rs          # Upgradeability tests (future)
└── integration_test.rs      # Multi-contract scenarios
```