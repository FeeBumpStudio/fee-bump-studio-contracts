# Building

## Build Commands

### Host Build (Testing)

```bash
cargo build
# Output: target/debug/fee_bump_studio_contracts
```

### Release Build (WASM)

```bash
make build
# or
stellar contract build
# Output: target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm
```

### Makefile Targets

```makefile
build:
	stellar contract build

test:
	cargo test
```

## Release Profile

The `Cargo.toml` release profile is optimized for WASM size:

```toml
[profile.release]
opt-level = "z"       # Optimize for size
lto = true            # Link-time optimization
codegen-units = 1     # Single codegen unit
panic = "abort"       # No panic runtime
```

### Optimization Details

| Setting | Effect |
|---------|--------|
| `opt-level = "z"` | Maximum size optimization |
| `lto = true` | Cross-crate optimization |
| `codegen-units = 1` | Better optimization |
| `panic = "abort"` | Removes panic runtime (~30 KB) |

## WASM Output

### Location

```
target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm
```

### Size Verification

```bash
# Check size
ls -lh target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm

# Expected: ~100-200 KB
```

### WASM Analysis

```bash
# Install wasm-tools
cargo install wasm-tools

# Inspect imports/exports
wasm-tools print target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm

# Component model (future)
wasm-tools component print ...
```

## Stellar Contract Build

The `stellar contract build` command wraps `cargo build` with:

1. Correct target (`wasm32-unknown-unknown`)
2. Release profile
3. WASM optimization passes
4. Metadata embedding

### Manual Equivalent

```bash
cargo build --target wasm32-unknown-unknown --release
# Then optimize:
soroban contract optimize target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm
```

## Build Verification

### Check WASM Validity

```bash
# Verify WASM structure
wasm-tools validate target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm

# Check exports
wasm-tools print target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm | grep export
```

### Expected Exports

```
export "initialize" (func)
export "record" (func)
export "read" (func)
export "memory" (memory)
```

## CI Build

```yaml
# .github/workflows/ci.yml
- name: Build
  run: cargo build --target wasm32-unknown-unknown --release

- name: Verify WASM size
  run: |
    SIZE=$(stat -c%s target/wasm32-unknown-unknown/release/fee_bump_studio_contracts.wasm)
    if [ $SIZE -gt 200000 ]; then
      echo "WASM too large: $SIZE bytes"
      exit 1
    fi
```

## Build Artifacts

| Artifact | Purpose |
|----------|---------|
| `fee_bump_studio_contracts.wasm` | Deployable contract |
| `build-info.json` | Build metadata (hash, timestamp) |
| `.wasm` (debug) | Debug build with symbols |

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `linker` not found | `rustup component add llvm-tools-preview` |
| WASM > 200 KB | Check release profile, remove debug symbols |
| `no_std` errors | Ensure `#![no_std]` in lib.rs |
| Missing `testutils` | Add `features = ["testutils"]` to dev-deps |
| Slow build | Use `cargo build --release` (caches) |

## Related Documentation

- [Local Development](local-development.md)
- [Testing](../contract-guide/testing.md)
- [Deployment](../contract-guide/deployment.md)
- [CI Workflow](../configuration/ci-cd.md)