# meta-signal-version-handover Architecture

This contract is the meta authority surface for component version
handover. It does not bind sockets, migrate databases, or select active
versions itself. Persona consumes this contract on its meta surface and
translates accepted authority orders into runtime behavior.

## Boundary

- `signal-version-handover` carries the ordinary private upgrade protocol
  between component versions.
- `meta-signal-version-handover` carries administrative authority for the
  engine that drives that protocol.
- `version-projection` carries cross-version type projection primitives.

## Operations

- `AttemptHandover` asks Persona to drive the ordinary private handover
  protocol for one component version pair. The request carries the versioned
  ordinary meta and private upgrade socket paths because the contract is still
  in the prototype phase before Persona has a full component-version catalog.
- `ForceFlip` asks Persona to flip a component's active selector from the
  current version to a target version even when the ordinary marker protocol
  would not choose that path automatically.
- `Rollback` asks Persona to restore a previous version as active after a
  recent handover.
- `Quarantine` marks one component version as ineligible for handover
  participation until meta policy changes.

## Constraints

- This is a pure signal contract crate: no daemon, no store, no socket policy.
- Every operation names a component and version identity values, using the
  same `ContractVersion` type as `version-projection` and
  `signal-version-handover`.
- `AttemptHandover` is the only normal-path operation. `ForceFlip`,
  `Rollback`, and `Quarantine` are meta overrides and must not forge a
  marker-backed handover fact.
- Runtime safety decisions remain in Persona. This crate only supplies typed
  meta vocabulary and typed replies.
- The prototype keeps Tap/Untap observability out of scope until a consuming
  daemon needs it.
