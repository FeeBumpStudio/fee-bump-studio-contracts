# Storage Keys

## Overview

The contract uses two storage types with specific key structures.

## Instance Storage

### Key: `"admin"`

| Property | Value |
|----------|-------|
| **Key** | `"admin"` (string) |
| **Value Type** | `Address` |
| **Set By** | `initialize(admin)` |
| **Read By** | Future admin operations |
| **Scope** | Contract instance |

### Access Pattern

```rust
// Write (initialize)
env.storage().instance().set(&String::from_str(&env, "admin"), &admin);

// Read (if needed)
let admin: Address = env.storage().instance()
    .get(&String::from_str(&env, "admin"))
    .expect("admin not set");
```

## Persistent Storage

### Key: `actor` (Address)

| Property | Value |
|----------|-------|
| **Key** | `actor` (Address) |
| **Value Type** | `String` (reference) |
| **Set By** | `record(actor, reference)` |
| **Read By** | `read(actor)` |
| **Scope** | Per-actor |
| **Overwrites** | Yes |

### Access Pattern

```rust
// Write (record)
env.storage().persistent().set(&actor, &reference);

// Read
let reference: Option<String> = env.storage().persistent().get(&actor);
```

## Storage Layout

```
Instance Storage:
┌─────────────────────────────────────┐
│ "admin"                             │ → Address (GABC...)
└─────────────────────────────────────┘

Persistent Storage:
┌─────────────────────────────────────┐
│ Actor: GABC123...                   │ → "TX-abc-001"
├─────────────────────────────────────┤
│ Actor: GBDE456...                   │ → "REF-xyz-789"
├─────────────────────────────────────┤
│ Actor: GXYZ789...                   │ → "FEE-bump-001"
└─────────────────────────────────────┘
```

## Key Encoding

### Address Keys

Soroban Address keys are automatically serialized:

```rust
// Address as key (automatic serialization)
env.storage().persistent().set(&actor, &value);
// Key bytes: [SCV_ADDRESS, 32-byte public key]
```

### String Keys

Instance storage uses string keys:

```rust
// String key (manual)
let key = String::from_str(&env, "admin");
env.storage().instance().set(&key, &value);
```

## Size Limits

| Storage Type | Limit | Typical Usage |
|--------------|-------|---------------|
| **Instance** | ~100 KB | Admin: ~100 bytes |
| **Persistent (per key)** | ~100 KB | Reference: variable |

## Reading Storage Externally

### Via RPC

```bash
# Instance storage
curl -X POST $RPC_URL \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getContractData","params":{"contractId":"CONTRACT_ID","key":{"type":"string","value":"admin"},"type":"instance"}}'

# Persistent storage
curl -X POST $RPC_URL \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getContractData","params":{"contractId":"CONTRACT_ID","key":{"type":"address","value":"GACTOR..."},"type":"persistent"}}'
```

### Via TypeScript

```typescript
import { Contract } from "@stellar/stellar-sdk";

// Instance storage
const admin = await contract.getInstanceStorage("admin");

// Persistent storage
const reference = await contract.getPersistentStorage(actorAddress);
```

## Future Storage Keys

| Feature | Key | Type | Description |
|---------|-----|------|-------------|
| Fee-bump events | `event_id` | Persistent | Event data |
| Actor index | `actor` | Persistent | `Vec<event_id>` |
| Event lookup | `tx_hash` | Persistent | `event_id` |
| Config | `"fee_bump_config"` | Instance | Contract config |

## Related Documentation

- [Architecture](../contract-guide/architecture.md#storage-model)
- [Functions](../api-reference/functions.md)
- [Interface](../contract-guide/interface.md)