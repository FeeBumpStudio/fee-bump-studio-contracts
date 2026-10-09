# Monitoring

## Overview

Monitoring the FeeBumpStudio contract involves tracking on-chain metrics via RPC and indexing.

## Key Metrics

### Contract Health

| Metric | Source | Alert Threshold |
|--------|--------|-----------------|
| **Contract exists** | `getContractInfo` | Missing = critical |
| **Admin set** | Instance storage | Missing = critical |
| **WASM hash matches** | Contract info | Mismatch = critical |

### Usage Metrics

| Metric | Source | Description |
|--------|--------|-------------|
| **Records created** | Indexed events | `record()` calls |
| **Unique actors** | Indexed events | Distinct actors |
| **Read calls** | RPC logs | `read()` calls |
| **Failed transactions** | Diagnostic events | Auth/budget errors |

### Performance

| Metric | Source | Target |
|--------|--------|--------|
| **Transaction latency** | RPC `getTransaction` | < 5s |
| **Simulation success rate** | Simulation results | > 99% |
| **Budget usage** | Transaction meta | < 80% max |

## Monitoring Implementation

### Health Check Script

```bash
#!/bin/bash
# health-check.sh

CONTRACT_ID=$1
RPC_URL=$2

# Check contract exists
stellar contract info --id $CONTRACT_ID --rpc-url $RPC_URL > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "CRITICAL: Contract not found"
    exit 2
fi

# Check admin set
ADMIN=$(stellar contract invoke --id $CONTRACT_ID --rpc-url $RPC_URL -- get-admin 2>/dev/null)
if [ -z "$ADMIN" ]; then
    echo "WARNING: Admin not set"
    exit 1
fi

echo "OK: Contract healthy"
exit 0
```

### Prometheus Metrics

```yaml
# prometheus.yml scrape config
scrape_configs:
  - job_name: 'feebumpstudio-contract'
    static_configs:
      - targets: ['contract-exporter:9090']
```

### Custom Exporter (Example)

```python
# contract_exporter.py
from prometheus_client import Gauge, Counter, Histogram

contract_exists = Gauge('feebumpstudio_contract_exists', 'Contract exists on network')
records_total = Counter('feebumpstudio_records_total', 'Total records created')
unique_actors = Gauge('feebumpstudio_unique_actors', 'Unique actors')
tx_latency = Histogram('feebumpstudio_tx_latency_seconds', 'Transaction latency')

# Scrape function
async def scrape_contract_metrics(contract_id, rpc_url):
    # Fetch contract info
    # Fetch recent events
    # Update metrics
    pass
```

## Alerting Rules

### Critical

```yaml
# Alert: Contract missing
- alert: FeeBumpStudioContractMissing
  expr: feebumpstudio_contract_exists == 0
  for: 1m
  labels:
    severity: critical
  annotations:
    summary: "FeeBumpStudio contract not found"

# Alert: Admin not set
- alert: FeeBumpStudioAdminMissing
  expr: feebumpstudio_admin_set == 0
  for: 5m
  labels:
    severity: critical
```

### Warning

```yaml
# Alert: High transaction failure rate
- alert: FeeBumpStudioHighFailureRate
  expr: rate(feebumpstudio_tx_failed_total[5m]) > 0.1
  for: 5m
  labels:
    severity: warning

# Alert: Budget usage high
- alert: FeeBumpStudioHighBudgetUsage
  expr: feebumpstudio_budget_usage_pct > 80
  for: 10m
  labels:
    severity: warning
```

## Log Aggregation

### Loki (Grafana)

```yaml
# promtail-config.yml
clients:
  - url: http://loki:3100/loki/api/v1/push

scrape_configs:
  - job_name: feebumpstudio-contract
    static_configs:
      - targets: [localhost]
        labels:
          job: feebumpstudio-contract
          __path__: /var/log/feebumpstudio/*.log
```

### Key Log Events

```json
{
  "level": "info",
  "event": "record_created",
  "actor": "GABC...",
  "reference": "TX-abc-001",
  "ledger": 12345678,
  "tx_hash": "a1b2c3..."
}
```

## Dashboard Panels (Grafana)

1. **Contract Status** — Exists, Admin set, WASM hash
2. **Records Over Time** — `rate(records_total[1h])`
3. **Unique Actors** — `unique_actors` gauge
4. **Transaction Latency** — `histogram_quantile(0.99, tx_latency)`
5. **Error Rate** — `rate(tx_failed_total[5m])`
6. **Budget Usage** — `budget_usage_pct`

## Related Documentation

- [Deployment](deployment.md)
- [Architecture](../contract-guide/architecture.md)
- [Backend Monitoring](../../fee-bump-studio-backend/docs/operations/monitoring.md)