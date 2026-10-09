# Events

## Current State

The baseline contract does **not emit custom events**. It relies on Soroban's built-in diagnostic events for observability.

## Diagnostic Events

Soroban automatically emits diagnostic events for:

| Event Type | Trigger | Data |
|------------|---------|------|
| **Contract invoke** | Any function call | Function name, args, auth |
| **Storage write** | `set()` / `put()` | Key, value size |
| **Storage read** | `get()` | Key, hit/miss |
| **Auth success** | `require_auth()` passed | Signer address |
| **Auth failure** | `require_auth()` failed | Signer address, error |
| **Error/panic** | Any trap/panic | Error code, location |

## Viewing Diagnostic Events

### Via RPC

```bash
# Get events for a transaction
curl -X POST https://soroban-testnet.stellar.org \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getTransaction","params":{"hash":"TX_HASH"}}'
```

Response includes `diagnosticEventsXdr` with decoded events.

### Via TypeScript

```typescript
const result = await server.getTransaction(hash);

if (result.diagnosticEventsXdr) {
  const events = xdr.DiagnosticEvent.fromXDR(
    Buffer.from(result.diagnosticEventsXdr, "base64")
  );
  
  for (const event of events) {
    console.log("Topics:", event.topics());
    console.log("Data:", event.data());
  }
}
```

## Event Structure (Diagnostic)

```rust
// Soroban diagnostic event (simplified)
pub struct DiagnosticEvent {
    pub contract_id: Address,
    pub topics: Vec<ScVal>,      // Event classification
    pub data: Vec<ScVal>,        // Event payload
}
```

### Example Topics

| Scenario | Topics |
|----------|--------|
| **Initialize called** | `["invoke", "initialize"]` |
| **Record called** | `["invoke", "record"]` |
| **Auth success** | `["auth", "success", "GABC..."]` |
| **Storage write** | `["storage", "write", "persistent"]` |
| **Error** | `["error", "HostError", "AuthFailed"]` |

## Future: Custom Events

For production, the contract should emit structured custom events:

```rust
// Future: Custom event emission
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeeBumpRecorded {
    pub actor: Address,
    pub reference: String,
    pub timestamp: u64,  // Ledger timestamp
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractInitialized {
    pub admin: Address,
}

impl FeeBumpStudioContract {
    pub fn record(env: Env, actor: Address, reference: String) {
        actor.require_auth();
        env.storage().persistent().set(&actor, &reference);
        
        // Emit custom event
        env.events().publish(
            ("fee_bump_recorded",),  // Topics
            FeeBumpRecorded {        // Data
                actor: actor.clone(),
                reference: reference.clone(),
                timestamp: env.ledger().timestamp(),
            }
        );
    }
}
```

## Indexer Integration

### Event Filtering

```javascript
// Backend indexer filters
const filters = [
  {
    type: 'contract',
    contractIds: [CONTRACT_ID],
    // Optional: filter by custom topics
    // topics: [['fee_bump_recorded']]
  },
  {
    type: 'diagnostic',  // For error detection
  }
];
```

### Event Processing

```javascript
async function processContractEvent(event) {
  switch (event.topics[0]) {
    case 'fee_bump_recorded':
      await handleFeeBumpRecorded(event);
      break;
    case 'contract_initialized':
      await handleInitialized(event);
      break;
    default:
      // Diagnostic events
      if (event.topics.includes('error')) {
        await handleError(event);
      }
  }
}
```

## Event Schema (Future)

### `FeeBumpRecorded`

| Field | Type | Description |
|-------|------|-------------|
| `actor` | Address | Actor who recorded |
| `reference` | String | Reference string |
| `timestamp` | u64 | Ledger close time |

### `ContractInitialized`

| Field | Type | Description |
|-------|------|-------------|
| `admin` | Address | Admin address |

## Event Versioning

| Version | Changes |
|---------|---------|
| v1 | Initial events |
| v2 | Add `tx_hash` to events |
| v3 | Add `fee_amount` for fee-bump events |

## Related Documentation

- [Architecture](architecture.md)
- [Stellar Ingestion (Backend)](../api-reference/stellar-ingestion.md)
- [Operations - Monitoring](../operations/monitoring.md)