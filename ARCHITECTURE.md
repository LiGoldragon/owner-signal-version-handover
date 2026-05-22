# owner-signal-version-handover Architecture

This contract is the owner-only authority surface for component version
handover. It does not run a handover, bind sockets, migrate databases, or
select active versions. Persona consumes this contract on its owner surface and
translates accepted authority orders into runtime behavior.

## Boundary

- `signal-version-handover` carries the ordinary private upgrade protocol
  between component versions.
- `owner-signal-version-handover` carries administrative authority for the
  engine that drives that protocol.
- `version-projection` carries cross-version type projection primitives.

## Operations

- `ForceFlip` asks Persona to flip a component's active selector from the
  current version to a target version even when the ordinary marker protocol
  would not choose that path automatically.
- `Rollback` asks Persona to restore a previous version as active after a
  recent handover.
- `Quarantine` marks one component version as ineligible for handover
  participation until owner policy changes.

## Constraints

- This is a pure signal contract crate: no daemon, no store, no socket policy.
- Every operation names a component and one or two `ContractVersion` values,
  using the same version identity type as `version-projection` and
  `signal-version-handover`.
- Runtime safety decisions remain in Persona. This crate only supplies typed
  owner vocabulary and typed replies.
- The prototype keeps Tap/Untap observability out of scope until a consuming
  daemon needs it.
