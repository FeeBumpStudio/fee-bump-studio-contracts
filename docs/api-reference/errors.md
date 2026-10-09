# Error Codes

## Overview

Errors in the FeeBumpStudio contract manifest as Soroban `HostError` values. This document maps common errors to their causes and resolutions.

## Host Errors

### `AuthFailed`

**Error:** `HostError: Error(Auth, InvalidAction)`

**Functions:** `initialize`, `record`

**Causes:**
- Missing authorization for required signer
- Incorrect signer (signed by wrong address)
- Authorization expired (TTL exceeded)

**Resolution:**
- Ensure correct signer in `authorize` array
- Re-prepare transaction if TTL expired
- Verify signer matches function parameter

**Example (TypeScript):**

```typescript
// Correct: signer matches parameter
await contract.record({ 
  actor: actorKeypair.publicKey(),  // Must match this
  reference: "TX-abc-001" 
})
  .prepare({ authorize: [actorKeypair.publicKey()] })  // Must include this
  .sign(actorKeypair)
  .submit();
```

---

### `StorageError`

**Error:** `HostError: Error(Context, StorageError)`

**Functions:** `initialize`, `record`

**Causes:**
- Storage quota exceeded
- Key too large
- Value too large
- Internal storage failure

**Resolution:**
- Reduce reference string length
- Check contract storage limits
- Retry transaction

---

### `AlreadyInitialized` (Baseline Panic)

**Error:** `HostError: Error(Context, InvalidAction)` with panic message

**Function:** `initialize`

**Cause:** Contract already has admin set

**Resolution:**
- Contract can only be initialized once (baseline)
- For new admin, redeploy contract

---

### `ContractNotInitialized`

**Error:** `HostError: Error(Context, MissingValue)` when reading admin

**Functions:** `record` (indirectly)

**Cause:** `initialize` never called, or admin not set

**Resolution:**
- Call `initialize` first with admin authorization

---

### `BudgetExceeded`

**Error:** `HostError: Error(Budget, BudgetExceeded)`

**Functions:** Any (complex operations)

**Cause:** Transaction exceeds instruction budget

**Resolution:**
- Optimize contract logic
- Increase fee
- Split into multiple transactions

---

## Diagnostic Events

Errors also appear in diagnostic events:

```json
{
  "topics": ["error", "AuthFailed"],
  "data": ["Unauthorized function call for address", "GABC..."]
}
```

## Error Handling Patterns

### TypeScript

```typescript
try {
  await contract.record({ actor, reference })
    .prepare({ authorize: [actor] })
    .sign(keypair)
    .submit();
} catch (error) {
  if (error.message.includes("AuthFailed")) {
    // Handle auth error
  } else if (error.message.includes("BudgetExceeded")) {
    // Handle budget error
  } else {
    // Handle other errors
  }
}
```

### Rust (Tests)

```rust
#[test]
#[should_panic(expected = "AuthFailed")]
fn record_without_auth_fails() {
    let env = Env::default();
    // NO mock_all_auths()
    
    let actor = Address::generate(&env);
    let contract_id = env.register(FeeBumpStudioContract, ());
    let client = FeeBumpStudioContractClient::new(&env, &contract_id);
    
    // Panics with AuthFailed
    client.record(&actor, &String::from_str(&env, "test"));
}
```

## Common Error Messages

| Message | Error Type | Function |
|---------|------------|----------|
| `"Unauthorized function call for address"` | `AuthFailed` | `initialize`, `record` |
| `"constructor invocation has failed"` | Various | `initialize` |
| `"calling unknown contract function"` | `InvalidAction` | Any (wrong function name) |
| `"BudgetExceeded"` | `BudgetExceeded` | Complex ops |
| `"StorageError"` | `StorageError` | `record`, `initialize` |

## Debugging Tips

1. **Enable debug logging** in TypeScript:
   ```typescript
   Config.setLogLevel("debug");
   ```

2. **Check diagnostic events** in transaction result:
   ```typescript
   const result = await server.getTransaction(hash);
   if (result.diagnosticEventsXdr) {
     // Parse and log events
   }
   ```

3. **Use simulation** before submission:
   ```typescript
   const sim = await contract.record({ actor, reference })
     .prepare({ authorize: [actor] })
     .simulate();
   // Check sim for errors
   ```

## Related Documentation

- [Functions](../api-reference/functions.md)
- [Authorization](../contract-guide/authorization.md)
- [Testing](../contract-guide/testing.md)
- [TypeScript Client](./typescript-client.md)