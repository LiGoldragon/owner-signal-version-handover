# meta-signal-version-handover

`meta-signal-version-handover` is the meta administrative signal
contract for version handover.

The ordinary `signal-version-handover` contract carries daemon-to-daemon
handover messages. This meta contract carries authority operations used by
Persona: force a selector flip, roll back a selector flip, or quarantine a
component version from upgrade participation.
