# TypeScript Client

## Overview

Generate a TypeScript client for the FeeBumpStudio contract using the Stellar CLI.

## Generation

```bash
stellar contract bindings typescript \
  --contract-id CABC123...YOUR_CONTRACT_ID \
  --network testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --output-dir ./generated/contract
```

## Generated Files

```
generated/contract/
├── index.ts              # Main client class
├── types.ts              # Type definitions
├── FeeBumpStudioContract.ts
└── package.json          # Dependency info
```

## Client Class

### Constructor

```typescript
import { FeeBumpStudioContract } from "./generated/contract";

const contract = new FeeBumpStudioContract({
  contractId: "CABC123...",      // Required: Contract address
  networkPassphrase: Networks.TESTNET,  // Required: Network
  rpcUrl: "https://soroban-testnet.stellar.org",  // Required: RPC
  allowHttp: false,              // Optional: Allow HTTP (dev only)
  publicKey: "GABC...",          // Optional: For read-only calls
});
```

### Options

| Option | Required | Type | Description |
|--------|----------|------|-------------|
| `contractId` | Yes | `string` | Contract address (starts with `C`) |
| `networkPassphrase` | Yes | `string` | Stellar network passphrase |
| `rpcUrl` | Yes | `string` | Soroban RPC endpoint |
| `allowHttp` | No | `boolean` | Allow HTTP (default: false) |
| `publicKey` | No | `string` | Public key for read-only calls |

## Methods

### `initialize`

```typescript
initialize(params: { admin: string }): TransactionBuilder<this>
```

**Parameters:**

| Param | Type | Description |
|-------|------|-------------|
| `admin` | `string` | Admin address (must authorize) |

**Returns:** `TransactionBuilder` — Chainable builder

**Example:**

```typescript
await contract.initialize({ admin: adminKeypair.publicKey() })
  .prepare({ authorize: [adminKeypair.publicKey()] })
  .sign(adminKeypair)
  .submit();
```

---

### `record`

```typescript
record(params: { actor: string; reference: string }): TransactionBuilder<this>
```

**Parameters:**

| Param | Type | Description |
|-------|------|-------------|
| `actor` | `string` | Actor address (must authorize) |
| `reference` | `string` | Reference string to store |

**Returns:** `TransactionBuilder` — Chainable builder

**Example:**

```typescript
await contract.record({ 
  actor: actorKeypair.publicKey(), 
  reference: "TX-abc-001" 
})
  .prepare({ authorize: [actorKeypair.publicKey()] })
  .sign(actorKeypair)
  .submit();
```

---

### `read`

```typescript
read(params: { actor: string }): TransactionBuilder<Option<string>>
```

**Parameters:**

| Param | Type | Description |
|-------|------|-------------|
| `actor` | `string` | Actor address to read |

**Returns:** `TransactionBuilder<Option<string>>` — Resolves to `string | null`

**Example:**

```typescript
const result = await contract.read({ actor: actorKeypair.publicKey() });
// result: "TX-abc-001" or null
```

## TransactionBuilder

The `TransactionBuilder` provides a fluent API for transaction construction.

### Methods

| Method | Description |
|--------|-------------|
| `prepare(options)` | Prepare transaction (simulate, add auth) |
| `sign(keypair)` | Sign with keypair |
| `submit()` | Submit to network |
| `simulate()` | Simulate without submitting |

### `prepare`

```typescript
prepare(options?: {
  authorize?: string[];      // Addresses that must authorize
  fee?: string | number;     // Fee in stroops
  timeout?: number;          // Timeout in seconds
}): TransactionBuilder
```

**Options:**

| Option | Type | Description |
|--------|------|-------------|
| `authorize` | `string[]` | Addresses that must sign |
| `fee` | `string \| number` | Custom fee (default: auto) |
| `timeout` | `number` | Transaction timeout (default: 30s) |

**Example:**

```typescript
const prepared = await contract.record({ actor, reference })
  .prepare({ 
    authorize: [actor],           // Required signers
    fee: "1000000",               // 0.0001 XLM
    timeout: 60                   // 60 seconds
  });
```

### `sign`

```typescript
sign(keypair: Keypair): TransactionBuilder
```

**Example:**

```typescript
await prepared.sign(actorKeypair).submit();
```

### `submit`

```typescript
submit(): Promise<SubmitTransactionResponse>
```

**Returns:** Transaction result with hash

**Example:**

```typescript
const result = await contract.initialize({ admin })
  .prepare({ authorize: [admin] })
  .sign(adminKeypair)
  .submit();

console.log("Transaction hash:", result.hash);
```

### `simulate`

```typescript
simulate(): Promise<SimulateTransactionResponse>
```

**Use Case:** Validate transaction before spending fees

**Example:**

```typescript
const simulation = await contract.record({ actor, reference })
  .prepare({ authorize: [actor] })
  .simulate();

if (Api.isSimulationError(simulation)) {
  console.error("Simulation failed:", simulation.error);
  // Check simulation.events for contract errors
} else {
  // Safe to submit
  await contract.record({ actor, reference })
    .prepare({ authorize: [actor] })
    .sign(keypair)
    .submit();
}
```

## Complete Workflow Example

```typescript
import { 
  FeeBumpStudioContract, 
  Networks, 
  Keypair,
  rpc,
  Api
} from "@stellar/stellar-sdk";
import { FeeBumpStudioContract } from "./generated/contract";

async function main() {
  // Setup
  const adminKeypair = Keypair.fromSecret("SADMIN...");
  const actorKeypair = Keypair.fromSecret("SACTOR...");
  
  const contract = new FeeBumpStudioContract({
    contractId: "CABC123...",
    networkPassphrase: Networks.TESTNET,
    rpcUrl: "https://soroban-testnet.stellar.org",
  });
  
  // 1. Initialize (admin only)
  console.log("Initializing contract...");
  await contract.initialize({ admin: adminKeypair.publicKey() })
    .prepare({ authorize: [adminKeypair.publicKey()] })
    .sign(adminKeypair)
    .submit();
  console.log("Initialized!");
  
  // 2. Record reference (actor)
  console.log("Recording reference...");
  await contract.record({ 
    actor: actorKeypair.publicKey(), 
    reference: "TX-abc-001" 
  })
    .prepare({ authorize: [actorKeypair.publicKey()] })
    .sign(actorKeypair)
    .submit();
  console.log("Recorded!");
  
  // 3. Read reference (public)
  console.log("Reading reference...");
  const result = await contract.read({ actor: actorKeypair.publicKey() });
  console.log("Reference:", result); // "TX-abc-001"
}

main().catch(console.error);
```

## Error Handling

```typescript
import { Api } from "@stellar/stellar-sdk";

try {
  await contract.record({ actor, reference })
    .prepare({ authorize: [actor] })
    .sign(keypair)
    .submit();
} catch (error) {
  if (error instanceof Api.ApiError) {
    console.error("API Error:", error.message);
  } else if (error.message.includes("AuthFailed")) {
    console.error("Authorization failed - check signer");
  } else if (error.message.includes("BudgetExceeded")) {
    console.error("Transaction too complex - increase fee");
  } else {
    console.error("Unexpected error:", error);
  }
}
```

## Read-Only Calls (No Wallet)

For read-only calls without wallet:

```typescript
const contract = new FeeBumpStudioContract({
  contractId: "CABC123...",
  networkPassphrase: Networks.TESTNET,
  rpcUrl: "https://soroban-testnet.stellar.org",
  publicKey: "GVIEWONLY...",  // Read-only identity
});

const result = await contract.read({ actor: "GACTOR..." });
// Works without signing
```

## Related Documentation

- [Contract Functions](../api-reference/functions.md)
- [Error Codes](../api-reference/errors.md)
- [Authorization](../contract-guide/authorization.md)
- [Deployment](../contract-guide/deployment.md)