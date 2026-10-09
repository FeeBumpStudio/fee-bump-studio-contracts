# Pull Request Template

## Description

Please include a summary of the change and which issue is fixed. Please also include relevant motivation and context.

Fixes # (issue)

## Type of Change

Please delete options that are not relevant.

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Refactoring (no functional changes)

## Testing

Please describe the tests that you ran to verify your changes. Provide instructions so we can reproduce.

- [ ] Tests pass with `cargo test`
- [ ] Build passes with `cargo build`
- [ ] WASM build passes with `make build` (if stellar CLI available)

## Contract Changes

If this PR modifies the contract interface:

- [ ] New/changed functions are documented in README.md
- [ ] Storage layout changes are noted (if any)
- [ ] Events are added/updated for indexers
- [ ] Authorization requirements are clear

## Checklist

- [ ] My code follows the style guidelines of this project
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have not committed any credentials, private keys, or production endpoints