# Contract Functions

## Overview

The `FeeBumpStudioContract` exposes three functions for verifiable fee-bump event recording.

## Function Reference

### `initialize`

```rust
pub fn initialize(env: Env, admin: Address)
```

**Description:** Sets the admin address in instance storage. Can only be called once.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `env` | `Env` | Soroban environment |
| `admin` | `Address` | Admin address to authorize and store |

**Authorization:** `admin.require_auth()`

**Storage:** Instance storage key `"admin"` → `admin`

**Returns:** `()`

**Errors:**
- Panics if already initialized (baseline behavior)

**Example (TypeScript):**

```typescript
await contract.initialize({ admin: adminKeypair.publicKey() })
  .prepare({ authorize: [adminKeypair.publicKey()] })
  .sign(adminKeypair)
  .submit();
```

**Example (CLI):**

```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source admin \
  --network testnet \
  -- initialize --admin <ADMIN_ADDRESS>
```

---

### `record`

```rust
pub fn record(env: Env, actor: Address, reference: String)
```

**Description:** Persists a reference string under the actor's address in persistent storage. Overwrites any existing reference.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `env` | `Env` | Soroban environment |
| `actor` | `Address` | Actor recording the reference (must authorize) |
| `reference` | `String` | Reference string to store |

**Authorization:** `actor.require_auth()`

**Storage:** Persistent storage key `actor` → `reference`

**Returns:** `()`

**Behavior:** Overwrites previous reference for the same actor

**Example (TypeScript):**

```typescript
await contract.record({ 
  actor: actorKeypair.publicKey(), 
  reference: "TX-abc-001" 
})
  .prepare({ authorize: [actorKeypair.publicKey()] })
  .sign(actorKeypair)
  .submit();
```

**Example (CLI):**

```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source actor \
  --network testnet \
  -- record --actor <ACTOR_ADDRESS> --reference "TX-abc-001"
```

---

### `read`

```rust
pub fn read(env: Env, actor: Address) -> Option<String>
```

**Description:** Reads the stored reference for an actor from persistent storage.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `env` | `Env` | Soroban environment |
| `actor` | `Address` | Actor to read reference for |

**Authorization:** None (public)

**Storage:** Persistent storage key `actor`

**Returns:** `Option<String>` — `Some(reference)` if exists, `None` otherwise

**Example (TypeScript):**

```typescript
const result = await contract.read({ actor: actorKeypair.publicKey() });
// result: "TX-abc-001" | null
```

**Example (CLI):**

```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source actor \
  --network testnet \
  -- read --actor <ACTOR_ADDRESS>
# Output: "TX-abc-001" or null
```

---

## Function Summary

| Function | Auth Required | Storage | Idempotent |
|----------|---------------|---------|------------|
| `initialize` | Admin | Instance | No |
| `record` | Actor | Persistent | Yes (overwrites) |
| `read` | None | Persistent (read) | N/A |

## Authorization Matrix

| Function | Required Signer |
|----------|-----------------|
| `initialize` | `admin` |
| `record` | `actor` |
| `read` | None |

## Gas Estimation

| Function | Estimated Cost | Notes |
|----------|----------------|-------|
| `initialize` | ~50,000 | One-time setup |
| `record` | ~30,000 | Per reference |
| `read` | ~10,000 | Read-only |

*Estimates for Testnet; actual costs vary by network conditions.*

## Error Codes

| Error | Function | Cause |
|-------|----------|-------|
| `AuthFailed` | All | Missing/invalid authorization |
| `StorageError` | `initialize`, `record` | Storage write failed |
| `AlreadyInitialized` | `initialize` | Admin already set (baseline panics) |

## Related Documentation

- [Storage Keys](../api-reference/storage.md)
- [Error Codes](../api-reference/errors.md)
- [TypeScript Client](../api-reference/typescript-client.md)
- [Authorization](../contract-guide/authorization.md)