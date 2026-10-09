# Contract Interface

## Rust Interface (Source)

```rust
// src/lib.rs
#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, String};

#[contract]
pub struct FeeBumpStudioContract;

#[contractimpl]
impl FeeBumpStudioContract {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&String::from_str(&env, "admin"), &admin);
    }

    pub fn record(env: Env, actor: Address, reference: String) {
        actor.require_auth();
        env.storage().persistent().set(&actor, &reference);
    }

    pub fn read(env: Env, actor: Address) -> Option<String> {
        env.storage().persistent().get(&actor)
    }
}
```

## Generated TypeScript Client

After building with `stellar contract bindings typescript`:

```typescript
// generated/contract/index.ts
export class FeeBumpStudioContract {
  constructor(options: ContractOptions);
  
  initialize(params: { admin: string }): TransactionBuilder<this>;
  record(params: { actor: string; reference: string }): TransactionBuilder<this>;
  read(params: { actor: string }): TransactionBuilder<Option<string>>;
}
```

## Function Details

### `initialize`

```rust
pub fn initialize(env: Env, admin: Address)
```

**Parameters:**
- `admin: Address` — The admin address to store

**Authorization:** `admin.require_auth()`

**Storage Write:** Instance storage key `"admin"` → `admin`

**Errors:**
- `AlreadyInitialized` — If admin already set (baseline panics)

### `record`

```rust
pub fn record(env: Env, actor: Address, reference: String)
```

**Parameters:**
- `actor: Address` — The actor recording the reference
- `reference: String` — The reference string to store

**Authorization:** `actor.require_auth()`

**Storage Write:** Persistent storage key `actor` → `reference`

**Behavior:** Overwrites any existing reference for the actor

**Errors:** None (baseline)

### `read`

```rust
pub fn read(env: Env, actor: Address) -> Option<String>
```

**Parameters:**
- `actor: Address` — The actor to read reference for

**Authorization:** None (public)

**Storage Read:** Persistent storage key `actor`

**Returns:** `Some(String)` if exists, `None` otherwise

**Errors:** None

## XDR Representation

### Initialize Transaction

```rust
// Operation: InvokeHostFunction
// Function: initialize
// Args: [admin: Address]
// Auth: [admin]
```

### Record Transaction

```rust
// Operation: InvokeHostFunction
// Function: record
// Args: [actor: Address, reference: String]
// Auth: [actor]
```

### Read Transaction

```rust
// Operation: InvokeHostFunction
// Function: read
// Args: [actor: Address]
// Auth: [] (public)
```

## TypeScript Usage

```typescript
import { FeeBumpStudioContract } from "./generated/contract";

const contract = new FeeBumpStudioContract({
  contractId: CONTRACT_ID,
  networkPassphrase: Networks.TESTNET,
  rpcUrl: RPC_URL,
});

// Initialize (admin only)
await contract.initialize({ admin: adminKeypair.publicKey() })
  .prepare({ authorize: [adminKeypair.publicKey()] })
  .sign(adminKeypair)
  .submit();

// Record reference
await contract.record({ 
  actor: actorKeypair.publicKey(), 
  reference: "TX-abc-001" 
})
  .prepare({ authorize: [actorKeypair.publicKey()] })
  .sign(actorKeypair)
  .submit();

// Read reference
const result = await contract.read({ actor: actorKeypair.publicKey() });
console.log(result); // "TX-abc-001" or null
```

## Rust Client Usage (Tests)

```rust
use fee_bump_studio_contracts::{FeeBumpStudioContract, FeeBumpStudioContractClient};

let env = Env::default();
env.mock_all_auths(); // Auto-authorize in tests

let admin = Address::generate(&env);
let actor = Address::generate(&env);

let contract_id = env.register(FeeBumpStudioContract, ());
let client = FeeBumpStudioContractClient::new(&env, &contract_id);

client.initialize(&admin);
client.record(&actor, &String::from_str(&env, "TX-abc-001"));
assert_eq!(client.read(&actor), Some(String::from_str(&env, "TX-abc-001")));
```

## Error Handling

| Error | Function | Cause |
|-------|----------|-------|
| `HostError: AuthFailed` | Any | Missing/invalid authorization |
| `StorageError` | `initialize`/`record` | Storage write failed |
| `ContractNotInitialized` | `record`/`read` | `initialize` not called |

## Related Documentation

- [Architecture](architecture.md)
- [Storage](storage.md)
- [Authorization](authorization.md)
- [TypeScript Client](../api-reference/typescript-client.md)