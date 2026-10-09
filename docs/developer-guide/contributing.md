# Contributing

## Before You Start

1. **Read the README** — Understand the project purpose and status
2. **Run the project locally** — Verify your environment works
3. **Check existing issues** — Avoid duplicate work
4. **Read this guide** — Follow the workflow

## Development Setup

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/fee-bump-studio-contracts.git
cd fee-bump-studio-contracts

# Install Rust toolchain
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli

# Verify
cargo test
```

## Contribution Workflow

### 1. Find or Create an Issue

- Check [GitHub Issues](https://github.com/FeeBumpStudio/fee-bump-studio-contracts/issues)
- Look for `good first issue`, `help wanted` labels
- Create new issue if needed — describe problem and proposed solution

### 2. Create a Branch

```bash
git checkout main
git pull origin main
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 3. Make Changes

- Keep changes focused and atomic
- Follow code style (Rust 2021, `no_std`, Clippy clean)
- Update documentation for interface changes
- Add tests for new functionality

### 4. Validate Locally

```bash
# Format
cargo fmt

# Lint
cargo clippy

# Test
cargo test

# Build WASM
make build
```

### 5. Commit

```bash
git add .
git commit -m "feat: add fee-bump event recording

- Add record_fee_bump function with structured event
- Emit FeeBumpRecorded event for indexing
- Update tests and documentation"
```

**Commit Message Format:**
```
<type>: <short description>

<body with details>

Fixes #<issue-number>
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `style`

### 6. Push and Open PR

```bash
git push origin feature/your-feature-name
```

Open PR against `main` branch. Fill out the PR template.

## Code Review Guidelines

### For Authors

- Keep PRs small (< 400 lines changed)
- Self-review before requesting review
- Respond to feedback promptly
- Update docs/tests with changes

### For Reviewers

- Check: correctness, style, tests, docs, security
- Be constructive and specific
- Approve when ready, request changes when needed

## Code Standards

### Rust

- **Edition 2021** — Modern Rust features
- **`no_std`** — Required for Soroban contracts
- **Clippy clean** — `cargo clippy` passes
- **Formatted** — `cargo fmt` applied
- **Explicit types** — Avoid inference in public APIs

### Soroban Contracts

- **Explicit authorization** — `require_auth()` on all writes
- **Minimal storage** — Only what needs on-chain proof
- **Deterministic** — Same input → same output
- **Testable** — Host tests with `mock_all_auths()`

### Security

- **No secrets in code** — Use environment variables
- **No mainnet in CI** — Testnet only for automated tests
- **Audit dependencies** — `cargo audit` before merging
- **Report vulnerabilities** — See [SECURITY.md](../../SECURITY.md)

## Documentation Updates

When adding features, update:

- [ ] Contract interface docs
- [ ] API reference if new functions
- [ ] Storage docs if schema changes
- [ ] Events docs if new events
- [ ] Deployment guide if process changes
- [ ] CHANGELOG (if maintained)

## Testing Requirements

All PRs must:

- [ ] Pass `cargo test`
- [ ] Pass `cargo clippy`
- [ ] Pass `cargo fmt --check`
- [ ] Include tests for new functionality
- [ ] Build WASM successfully (`make build`)

## Release Process

1. Maintainer creates release branch
2. Version bump in `Cargo.toml`
3. Changelog updated
4. Tagged release `vX.Y.Z`
5. GitHub Actions builds and verifies WASM

## Getting Help

- **GitHub Discussions** — Design questions, architecture
- **GitHub Issues** — Bugs, feature requests
- **Security** — Private disclosure per [SECURITY.md](../../SECURITY.md)
- **Soroban Discord** — #soroban channel for SDK questions

## Recognition

Contributors are recognized in:

- Release notes
- Contributors list (future)
- Project documentation

Thank you for contributing to FeeBumpStudio!