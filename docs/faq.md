# Frequently Asked Questions

## General

### What is FeeBumpStudio Contracts?

FeeBumpStudio Contracts is the Soroban smart contract layer providing minimal on-chain state for verifiable fee-bump event recording. It stores references authorized by actors and exposes them for public verification.

### Is this production-ready?

**No.** This is a development baseline (v0.1.0). It is **not audited** and **not production-ready**. Do not deploy with real value on Mainnet.

### Who maintains this?

**Jubilee** ([@Jubilee-001](https://github.com/Jubilee-001)).

### Where is the source code?

- **Contracts**: https://github.com/FeeBumpStudio/fee-bump-studio-contracts
- **App**: https://github.com/FeeBumpStudio/fee-bump-studio-app
- **Backend**: https://github.com/FeeBumpStudio/fee-bump-studio-backend

## Getting Started

### Do I need Rust to use the contract?

**For development:** Yes — Rust 1.75+ for building/testing.

**For integration:** No — Use TypeScript client or CLI.

### Which wallet should I use for deployment?

- **CLI**: Stellar CLI with secret key
- **App**: Freighter, Albedo, Rabet via wallet adapter

### Why Testnet by default?

Testnet is free, resets periodically, and safe for development. Mainnet requires real XLM and dedicated RPC.

## Technical

### What's the tech stack?

- **Language**: Rust (2021 edition, `no_std`)
- **SDK**: soroban-sdk 28
- **Target**: WebAssembly (wasm32-unknown-unknown)
- **Tests**: Host-side with `testutils`

### How does authorization work?

Every state-changing function requires explicit authorization via `require_auth()`:
- `initialize`: Admin must sign
- `record`: Actor must sign
- `read`: Public (no auth)

### Can I upgrade the contract?

**Baseline: No** — Immutable by design. Future versions may use proxy pattern with timelock.

### What's the storage model?

| Storage | Keys | Use Case |
|---------|------|----------|
| Instance | `"admin"` | Contract metadata |
| Persistent | Actor Address | Actor → Reference |

### What's the WASM size?

~100-200 KB optimized (release profile: `opt-level = "z"`, `panic = "abort"`)

## Deployment

### Where do I get a contract ID?

Deploy via Stellar CLI:
```bash
stellar contract deploy --wasm target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm --source admin --network testnet
```

### Why do I need a dedicated RPC for Mainnet?

There is **no public Mainnet RPC**. Use Stellar RPC, QuickNode, Blockdaemon, or self-hosted.

### Can I run without a contract ID?

Yes, for local testing. Integration features require deployed contract.

## Integration

### How do I generate TypeScript bindings?

```bash
stellar contract bindings typescript \
  --contract-id CABC123... \
  --network testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --output-dir ./generated/contract
```

### How do I call from TypeScript?

```typescript
const contract = new FeeBumpStudioContract({
  contractId: "CABC123...",
  networkPassphrase: Networks.TESTNET,
  rpcUrl: "https://soroban-testnet.stellar.org",
});

await contract.record({ actor, reference })
  .prepare({ authorize: [actor] })
  .sign(keypair)
  .submit();
```

### How do I call from CLI?

```bash
stellar contract invoke --id CONTRACT_ID --source actor --network testnet -- record --actor ACTOR --reference "TX-abc-001"
```

## Troubleshooting

### "AuthFailed" error

**Cause:** Missing or incorrect authorization.

**Fix:** Ensure correct signer in `authorize` array:
```typescript
.prepare({ authorize: [actorKeypair.publicKey()] })  // Must match function param
```

### "Contract not found"

**Cause:** Contract ID doesn't exist on target network.

**Fix:** Deploy contract on that network and update `STELLAR_CONTRACT_ID`.

### "Insufficient balance"

**Cause:** Account needs XLM for fees.

**Fix:** Fund account on correct network (Friendbot for Testnet).

### Transaction times out

**Cause:** RPC slow or network congestion.

**Fix:** Increase timeout, use faster RPC, or check status via hash.

### WASM too large

**Cause:** Release profile not optimized.

**Fix:** Verify `Cargo.toml`:
```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
```

## Contributing

### How can I contribute?

1. Check [GitHub Issues](https://github.com/FeeBumpStudio/fee-bump-studio-contracts/issues)
2. Look for `good first issue` labels
3. Fork, create branch, make changes
4. Open PR with description

### What's the development setup?

```bash
git clone https://github.com/FeeBumpStudio/fee-bump-studio-contracts.git
cd fee-bump-studio-contracts
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli
cargo test
```

### Are there coding standards?

- Rust 2021 edition, `no_std`
- `cargo fmt` and `cargo clippy` must pass
- `cargo test` must pass
- `make build` must succeed

## Roadmap

### What's next?

See the [README Roadmap](https://github.com/FeeBumpStudio/fee-bump-studio-contracts/blob/main/README.md#roadmap):

1. Custom events + multiple records per actor
2. Query helpers + pagination
3. Proxy upgradeability + timelock
4. Multi-sig admin + pause
5. Tests + CI/CD + monitoring
6. Security audit

### When will v1.0 release?

No fixed timeline. Depends on community contributions and audit completion.

## Still Have Questions?

- **GitHub Discussions** — Design questions, architecture
- **GitHub Issues** — Bugs, feature requests
- **Discord** — Community chat (if available)
- **Email** — security@feebumpstudio.example.com (security only)