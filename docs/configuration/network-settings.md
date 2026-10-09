# Network Settings

## Network Configuration

The contract supports two Stellar networks:

| Network | Passphrase | Use Case |
|---------|------------|----------|
| **Testnet** | `Test SDF Network ; September 2015` | Development, testing, CI |
| **Mainnet** | `Public Global Stellar Network ; September 2015` | Production |

## RPC Endpoints

### Testnet

| Provider | URL | Notes |
|----------|-----|-------|
| **SDF Public** | `https://soroban-testnet.stellar.org` | Free, rate-limited, default |
| **QuickNode** | `https://your-endpoint.quicknode.com` | Paid, higher limits |
| **Self-hosted** | `https://your-rpc.example.com` | Full control |

### Mainnet

| Provider | URL | Notes |
|----------|-----|-------|
| **Stellar RPC** | `https://rpc.stellar.org/your-project-id` | Official managed service |
| **QuickNode** | `https://your-endpoint.quicknode.com` | Multi-chain provider |
| **Blockdaemon** | `https://your-endpoint.blockdaemon.com` | Enterprise |
| **Self-hosted** | `https://your-rpc.example.com` | Full control |

> **Mainnet requires a dedicated RPC endpoint.** No public default exists.

## Network Validation

The contract validates network configuration on deployment:

```rust
// In deployment scripts
async fn validate_network(rpc_url: &str, expected_passphrase: &str) -> Result<u32, Error> {
    let server = rpc::Server::new(rpc_url);
    let ledger = server.get_latest_ledger().await?;
    
    let network_config = server.get_network_config().await?;
    if network_config.network_passphrase != expected_passphrase {
        return Err(Error::NetworkMismatch);
    }
    
    Ok(ledger.sequence)
}
```

## Contract Deployment Per Network

Contracts must be deployed separately per network:

| Network | Contract ID | Deployment |
|---------|-------------|------------|
| Testnet | `CABC...TESTNET` | `make deploy-testnet` |
| Mainnet | `CXYZ...MAINNET` | `make deploy-mainnet` |

Set `STELLAR_CONTRACT_ID` to match the target network.

## Account Compatibility

| Account Type | Testnet | Mainnet |
|--------------|---------|---------|
| **Funded via Friendbot** | ✅ Yes | ❌ No |
| **Exchange withdrawal** | ✅ Yes | ✅ Yes |
| **Existing Mainnet account** | ❌ No | ✅ Yes |

Testnet and Mainnet are **completely separate** — accounts, balances, and contracts do not transfer.

## Rate Limits

| Provider | Testnet Limit | Mainnet Limit |
|----------|---------------|---------------|
| SDF Public | 100 req/min | N/A |
| Stellar RPC | N/A | Tier-based |
| QuickNode | Plan-based | Plan-based |
| Self-hosted | Configurable | Configurable |

## Monitoring

Recommended alerts:

- RPC error rate > 5%
- RPC latency > 2s (p95)
- Rate limit approaching 80%
- Ledger sync lag > 30s

## Troubleshooting

| Issue | Check |
|-------|-------|
| "Network mismatch" | Verify `STELLAR_NETWORK` matches RPC endpoint |
| "Contract not found" | Ensure contract deployed on target network |
| "Insufficient balance" | Fund account on correct network (Friendbot for Testnet) |
| "Transaction failed" | Check diagnostic events via RPC |

## Related Documentation

- [Environment Variables](environment-variables.md)
- [Deployment](../contract-guide/deployment.md)
- [Stellar Networks](https://developers.stellar.org/docs/learn/fundamentals/networks)