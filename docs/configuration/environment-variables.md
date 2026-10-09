# Environment Variables

## Complete Reference

All environment variables for the FeeBumpStudio Contracts:

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `STELLAR_NETWORK` | Yes | `testnet` | Target network: `testnet` or `mainnet` |
| `STELLAR_RPC_URL` | No | (public default) | Soroban RPC endpoint URL |
| `STELLAR_CONTRACT_ID` | No* | — | Deployed contract ID for integration |

> *Required for app/backend integration after deployment.

## Variable Details

### STELLAR_NETWORK

```ini
# Testnet (development)
STELLAR_NETWORK=testnet

# Mainnet (production)
STELLAR_NETWORK=mainnet
```

Controls:
- Network passphrase for transactions
- Default RPC endpoint if `STELLAR_RPC_URL` not set
- Contract deployment target

### STELLAR_RPC_URL

```ini
# Use public Testnet (development only)
STELLAR_RPC_URL=

# Dedicated RPC (recommended for production)
STELLAR_RPC_URL=https://rpc.stellar.org/your-project-id
STELLAR_RPC_URL=https://your-endpoint.quicknode.com/your-token
```

If empty:
- Testnet → `https://soroban-testnet.stellar.org`
- Mainnet → **Required** (no public default)

### STELLAR_CONTRACT_ID

```ini
# Contract ID from deployment
STELLAR_CONTRACT_ID=CABC123...YOUR_CONTRACT_ID
```

Format: Stellar address starting with `C` (contract)

## Environment Files

### .env.example (Committed)

Template with all variables and defaults:

```ini
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=
STELLAR_CONTRACT_ID=
```

### .env (Local Only, Gitignored)

Your actual configuration:

```ini
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_CONTRACT_ID=CABC123...YOUR_CONTRACT_ID
```

### .env.production (Deployment)

For production deployments:

```ini
STELLAR_NETWORK=mainnet
STELLAR_RPC_URL=https://rpc.stellar.org/your-production-project
STELLAR_CONTRACT_ID=CXYZ789...YOUR_MAINNET_CONTRACT
```

## Loading Priority

The application loads in order (highest priority wins):

1. System environment variables
2. `.env` (via `dotenv` if used)
3. Defaults in code

## Validation

```rust
// Example validation in integration code
fn validate_config() -> Result<Config, Error> {
    let network = std::env::var("STELLAR_NETWORK")
        .unwrap_or_else(|_| "testnet".to_string());
    
    if !["testnet", "mainnet"].contains(&network.as_str()) {
        return Err(Error::InvalidNetwork(network));
    }
    
    if network == "mainnet" && std::env::var("STELLAR_RPC_URL").is_err() {
        return Err(Error::MissingRpcUrl);
    }
    
    Ok(Config { network, rpc_url: std::env::var("STELLAR_RPC_URL").ok() })
}
```

## Security Checklist

- [ ] Never commit `.env`, `.env.production`, or any `.env.*` with real values
- [ ] Use different contract IDs for Testnet/Mainnet
- [ ] Use dedicated RPC for production
- [ ] Rotate RPC credentials periodically
- [ ] Monitor RPC usage and rate limits

## CI/CD Variables

Set in GitHub repository settings → Secrets and variables → Actions:

| Secret | Purpose |
|--------|---------|
| `STELLAR_NETWORK` | Deployment target network |
| `STELLAR_RPC_URL` | RPC for deployment verification |
| `STELLAR_CONTRACT_ID` | Contract to verify against |

## Related Documentation

- [Network Settings](network-settings.md)
- [Deployment](../contract-guide/deployment.md)
- [Security - Assumptions](../security/assumptions.md)