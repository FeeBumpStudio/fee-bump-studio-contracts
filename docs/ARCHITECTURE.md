# Contract architecture

## Responsibility

A developer workflow for inspecting a transaction, estimating a replacement fee, preparing fee-bump envelopes, and keeping the original transaction intent visible for review.

## Security boundary

Every state-changing operation must authenticate the actor that is allowed to cause
the change. Contract storage is intentionally smaller than the application database.

## Future specification

The generic development contract in this baseline must be replaced with the
project-specific state model before production deployment.
