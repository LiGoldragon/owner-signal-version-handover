# INTENT — owner-signal-version-handover

*The owner-only authority contract for component version handover. Defines the
typed request/reply channel that Persona consumes on its owner surface to drive
the ordinary private handover protocol and to apply owner overrides: force
selector flip, rollback, and quarantine.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this owner-only
`owner-signal-version-handover` contract. Workspace-shape intent stays in the
primary workspace `primary/INTENT.md`. The ordinary private upgrade protocol
between component versions stays in `signal-version-handover/INTENT.md`.
Cross-version type projection primitives stay in `version-projection`.

## Why this repo exists

`owner-signal-version-handover` is the **owner-only authority surface** for
component version handover. It does not bind sockets, migrate databases, or
select active versions itself — Persona consumes this contract on its owner
surface and translates accepted authority orders into runtime behavior. The
companion `signal-version-handover` carries the ordinary private upgrade protocol
between component versions; `version-projection` carries cross-version type
projection primitives.

## The channel shape

The owner channel carries:

- **`AttemptHandover`** — the only normal-path operation: asks Persona to drive
  the ordinary private handover protocol for one component version pair. While
  the contract is in the prototype phase (before Persona has a full
  component-version catalog), the request carries the versioned ordinary owner and
  private upgrade socket paths.
- **`ForceFlip`** — owner override: flip a component's active selector to a target
  version even when the ordinary marker protocol would not choose that path.
- **`Rollback`** — owner override: restore a previous version as active after a
  recent handover.
- **`Quarantine`** — owner override: mark a component version ineligible for
  handover participation until owner policy changes.

Every operation names a component and version identity using the same
`ContractVersion` type as `version-projection` and `signal-version-handover`.

## Constraints

- This is a pure signal contract crate: no daemon, no store, no socket policy.
- `AttemptHandover` is the only normal-path operation; `ForceFlip`, `Rollback`,
  and `Quarantine` are owner overrides and must not forge a marker-backed handover
  fact.
- Runtime safety decisions remain in Persona — this crate only supplies typed
  owner vocabulary and typed replies.
- Version identity values reuse the `ContractVersion` type shared across the
  version-handover family.
- The prototype keeps Tap/Untap observability out of scope until a consuming
  daemon needs it.
- Wire enums are closed. Every operation and reply round-trips through both rkyv
  frames and NOTA text.

## Non-ownership

This crate does not own:

- any daemon, store, or socket policy;
- database migration or active-version selection (those are Persona runtime);
- the ordinary private upgrade protocol (lives in `signal-version-handover`);
- cross-version type projection primitives (live in `version-projection`).

## See also

- `ARCHITECTURE.md` — operations, the handover boundary, and constraints.
- `../signal-version-handover/INTENT.md` — ordinary private upgrade protocol.
- `../version-projection/ARCHITECTURE.md` — cross-version type projection primitives.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and authority tiers.
