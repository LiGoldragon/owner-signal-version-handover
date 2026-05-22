# owner-signal-version-handover

`owner-signal-version-handover` is the owner-only administrative signal
contract for version handover.

The ordinary `signal-version-handover` contract carries daemon-to-daemon
handover messages. This owner contract carries authority operations used by
Persona: force a selector flip, roll back a selector flip, or quarantine a
component version from upgrade participation.
