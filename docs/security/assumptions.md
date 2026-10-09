# Security Assumptions

## Threat Model

### Assumptions

| Assumption | Justification |
|------------|---------------|
| **Soroban runtime is correct** | Audited, formally verified components |
| **Stellar consensus is Byzantine fault tolerant** | Proven protocol, live since 2015 |
| **Admin key is secure** | Offline storage, hardware wallet |
| **RPC endpoint is trusted** | TLS + known provider |
| **No side-channel leaks** | Soroban sandbox isolation |

### Trust Boundaries

```
┌─────────────────────────────────────────────────────────────┐
│                      STELLAR NETWORK                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │  Soroban    │  │  Consensus  │  │  Contract (Trusted) │  │
│  │  Runtime    │◄─┤  Protocol   │◄─┤  FeeBumpStudio      │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                          ▲
              RPC/HTTPS   │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                      INTEGRATION LAYER                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   App       │  │  Backend    │  │  CLI/Tools          │  │
│  │  (Untrusted)│  │  (Trusted)  │  │  (User-controlled)  │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Security Properties

### Confidentiality

- **No secrets on-chain** — Only public references stored
- **Admin key off-chain** — Never in contract storage

### Integrity

- **Authorization enforced** — `require_auth()` on all writes
- **Storage tamper-evident** — Blockchain immutability
- **Deterministic execution** — Same input → same output

### Availability

- **No single point of failure** — Stellar network redundancy
- **Read-only access public** — No auth needed for `read()`
- **Graceful degradation** — Failed writes don't affect reads

## Attack Surface Analysis

### Contract Functions

| Vector | Mitigation |
|--------|------------|
| **Auth bypass** | `require_auth()` on all state changes |
| **Storage exhaustion** | Bounded string size, single key per actor |
| **Reentrancy** | No external calls in baseline |
| **Integer overflow** | Soroban SDK checked arithmetic |
| **Access control** | Explicit `require_auth` per function |

### Storage

| Vector | Mitigation |
|--------|------------|
| **Key collision** | Address keys are unique |
| **Value size** | String bounded by reference format |
| **Unauthorized read** | Public read is intentional |
| **Unauthorized write** | `require_auth` enforced |

### Authorization

| Vector | Mitigation |
|--------|------------|
| **Missing auth** | `require_auth()` on all writes |
| **Wrong signer** | Parameter-matched auth |
| **Auth replay** | Soroban auth TTL (ledger-based) |
| **Multi-sig bypass** | Single-signer baseline |

## Cryptographic Assumptions

| Primitive | Algorithm | Source |
|-----------|-----------|--------|
| Signatures | Ed25519 | Stellar SDK / Wallet |
| Hashing | SHA-256 | Soroban SDK |
| Auth TTL | Ledger sequence | Soroban protocol |
| Key derivation | PBKDF2 (wallet) | Wallet implementation |

## Compliance Considerations

| Regulation | Applicability | Approach |
|------------|---------------|----------|
| GDPR | No PII stored | N/A |
| SOC2 | Not certified | Document controls |
| PCI DSS | No payments | N/A |

## Future Hardening

- [ ] Formal verification of auth logic
- [ ] Fuzzing with `soroban-test-utils`
- [ ] Formal specification (TLA+/Coq)
- [ ] Penetration testing before Mainnet
- [ ] Upgradeability with timelock (if needed)
- [ ] Rate limiting via contract (future)

## Related Documentation

- [Responsible Disclosure](responsible-disclosure.md)
- [Known Limitations](limitations.md)
- [Authorization](../contract-guide/authorization.md)
- [Architecture](../contract-guide/architecture.md)