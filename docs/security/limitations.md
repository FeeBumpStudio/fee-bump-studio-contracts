# Known Limitations

## Current Baseline Limitations

### Functionality

| Limitation | Impact | Planned Resolution |
|------------|--------|-------------------|
| **Only 3 functions** | Minimal feature set | Add fee-bump event recording |
| **No custom events** | Hard to index | Add `#[contractevent]` |
| **Single reference per actor** | Overwrites previous | Support multiple events |
| **No query helpers** | Full scan needed | Add pagination/filtering |
| **Immutable** | Cannot fix bugs | Proxy pattern with timelock |
| **No fee-bump logic** | Core feature missing | Add structured event recording |

### Technical

| Limitation | Impact | Planned Resolution |
|------------|--------|-------------------|
| **No tests for auth edge cases** | Regression risk | Add auth_test.rs |
| **No integration tests** | Deployment gaps | Add integration_test.rs |
| **No formal verification** | Unknown bugs | TLA+/Coq spec |
| **Single admin** | Bus factor | Multi-sig admin |
| **No upgrade path** | Cannot fix bugs | Proxy pattern |

### Security

| Limitation | Impact | Planned Resolution |
|------------|--------|-------------------|
| **No audit** | Unknown vulnerabilities | Third-party audit |
| **Single admin key** | Compromise = full control | Multi-sig / timelock |
| **No event emission** | Hard to monitor | Custom events |
| **No rate limiting** | Spam possible | Contract-level limits |
| **No pause mechanism** | Cannot stop attacks | Pause function |

## Architecture Limitations

### Scalability

| Area | Current | Limit | Future |
|-------|---------|-------|--------|
| **Records per actor** | 1 | Overwrites | Multiple per actor |
| **Total records** | ~1000 | Storage limit | Sharding/archiving |
| **Query performance** | Full scan | O(n) | Indexes/pagination |
| **Concurrent writes** | Sequential | Sequential | Parallel (different actors) |

### Maintainability

| Area | Current | Risk |
|------|---------|------|
| **Code organization** | Single file | Growing complexity |
| **Type safety** | Basic | Runtime errors possible |
| **Event observability** | Diagnostic only | Hard to monitor |
| **Upgrade path** | None | Cannot fix bugs |

## Network-Specific Limitations

### Testnet

- **Rate limits** — 100 req/min on public RPC
- **Ledger reset** — Periodic; contracts wiped
- **No SLA** — Not for production workloads

### Mainnet (Future)

- **Cost** — Real XLM for deployment/fees
- **Finality** — ~5s ledger close; wait for confirmations
- **Regulatory** — Consider jurisdiction requirements
- **Immutability** — Real value at risk

## Dependency Risks

| Dependency | Risk | Mitigation |
|------------|------|------------|
| `soroban-sdk` | Breaking changes | Pin version; test upgrades |
| `stellar-cli` | Deployment changes | Pin version in CI |
| Soroban protocol | Network upgrades | Monitor SDF announcements |

## Upgrade Path

### From Baseline to Production

```
v0.1.0 (baseline)
    │
    ├─► v0.2.0: Custom events + multiple records
    │
    ├─► v0.3.0: Query helpers + pagination
    │
    ├─► v0.4.0: Proxy upgradeability + timelock
    │
    ├─► v0.5.0: Multi-sig admin + pause
    │
    ├─► v0.6.0: Tests + CI/CD + monitoring
    │
    ├─► v0.7.0: Security hardening + audit prep
    │
    └─► v1.0.0: Production release (post-audit)
```

## Acceptable Use

This baseline is **only suitable for**:

- Local development and testing
- Testnet experimentation
- Architecture evaluation
- Contributor onboarding

**Not suitable for**:

- Mainnet funds
- Production workloads
- User-facing demos without disclaimers
- Compliance-required environments

## Related Documentation

- [Roadmap](https://github.com/FeeBumpStudio/fee-bump-studio-contracts/blob/main/README.md#roadmap)
- [Security Assumptions](assumptions.md)
- [Responsible Disclosure](responsible-disclosure.md)
- [Architecture](../contract-guide/architecture.md)