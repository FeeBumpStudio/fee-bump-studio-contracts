# Storage

## Overview

The contract uses two storage types provided by Soroban:

| Storage Type | Scope | Persistence | Size Limit | Use Case |
|--------------|-------|-------------|------------|----------|
| **Instance** | Contract-level | Contract lifetime | ~100 KB | Metadata, config |
| **Persistent** | Per-key | Until deleted | ~100 KB/key | User data |

## Instance Storage

### Current Usage

```rust
// Initialize: store admin
env.storage().instance().set(&String::from_str(&env, "admin"), &admin);

// Read admin (if needed)
let admin: Address = env.storage().instance().get(&String::from_str(&env, "admin")).unwrap();
```

### Key: `"admin"`

| Property | Value |
|----------|-------|
| **Type** | `Address` |
| **Set by** | `initialize()` |
| **Read by** | Future admin operations |
| **Updatable** | No (baseline) |

## Persistent Storage

### Current Usage

```rust
// Record: store reference under actor
env.storage().persistent().set(&actor, &reference);

// Read: get reference for actor
let reference: Option<String> = env.storage().persistent().get(&actor);
```

### Key: `actor` (Address)

| Property | Value |
|----------|-------|
| **Type** | `String` (reference) |
| **Set by** | `record()` |
| **Read by** | `read()` |
| **Updatable** | Yes (overwrites) |

### Storage Layout

```
Persistent Storage:
┌─────────────────────────────────────┐
│ Actor Address (GABC...)             │ → "TX-abc-001"
├─────────────────────────────────────┤
│ Actor Address (GBDE...)             │ → "REF-xyz-789"
├─────────────────────────────────────┤
│ Actor Address (GXYZ...)             │ → "FEE-bump-001"
└─────────────────────────────────────┘
```

## Storage Lifecycle

```mermaid
flowchart TD
    A[Contract Deployed] --> B[Instance: empty]
    B --> C[initialize(admin)]
    C --> D[Instance: "admin" = admin]
    D --> E[record(actor1, ref1)]
    E --> F[Persistent: actor1 = ref1]
    F --> G[record(actor1, ref2)]
    G --> H[Persistent: actor1 = ref2]  // Overwrite
    H --> I[record(actor2, ref3)]
    I --> J[Persistent: actor2 = ref3]
```

## Size Considerations

### Instance Storage (~100 KB)

- Admin address: ~100 bytes
- Plenty of room for future config

### Persistent Storage (~100 KB per key)

- Reference string: Variable (keep < 1 KB recommended)
- Actor address as key: ~100 bytes
- Theoretical max entries: ~1000 per contract

### Optimization Tips

| Technique | Savings |
|-----------|---------|
| Short references | Use hashes/IDs instead of full descriptions |
| Batch operations | Single transaction for multiple records |
| Compress off-chain | Store hash on-chain, full data off-chain |

## Reading Storage (External)

### Via RPC

```bash
# Get instance storage (admin)
stellar contract invoke --id CONTRACT_ID -- get-instance-storage

# Get persistent storage for actor
stellar contract invoke --id CONTRACT_ID -- get-persistent-storage --key ACTOR_ADDRESS
```

### Via TypeScript

```typescript
// Instance storage
const admin = await contract.getInstanceStorage("admin");

// Persistent storage
const reference = await contract.getPersistentStorage(actorAddress);
```

## Future Storage Extensions

| Feature | Storage Type | Design |
|---------|--------------|--------|
| **Fee-bump events** | Persistent | `event_id` → `EventData` |
| **Actor index** | Persistent | `actor` → `Vec<event_id>` |
| **Event lookup** | Persistent | `tx_hash` → `event_id` |
| **Config** | Instance | `fee_bump_config` → `Config` |

## Related Documentation

- [Architecture](architecture.md)
- [Interface](interface.md)
- [Authorization](authorization.md)
- [Deployment](../contract-guide/deployment.md)