//! Owner-only signal contract for version-handover authority.
//!
//! The ordinary `signal-version-handover` contract carries the private
//! daemon-to-daemon handover protocol. This owner contract carries
//! administrative authority for the persona engine: attempt an ordinary
//! handover, force an active selector flip, roll back a recent flip, or
//! quarantine a component version.

use nota_codec::{NotaEnum, NotaRecord};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
use signal_sema::SemaObservation;
use version_projection::{ComponentName, ContractVersion};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub label: VersionLabel,
    pub contract_version: ContractVersion,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    nota_codec::NotaTransparent,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct VersionLabel(String);

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    nota_codec::NotaTransparent,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct SocketPath(String);

impl Version {
    pub fn new(label: VersionLabel, contract_version: ContractVersion) -> Self {
        Self {
            label,
            contract_version,
        }
    }
}

impl VersionLabel {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl SocketPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum ForceReason {
    OperatorOverride,
    MarkerMismatchAccepted,
    EmergencyRecovery,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RollbackReason {
    PostCutoverFailure,
    OperatorOverride,
    RecoveryDrill,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum QuarantineReason {
    FailedUpgrade,
    SuspectState,
    OperatorHold,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ForceFlip {
    pub component: ComponentName,
    pub current_version: Version,
    pub target_version: Version,
    pub reason: ForceReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Rollback {
    pub component: ComponentName,
    pub active_version: Version,
    pub restore_version: Version,
    pub reason: RollbackReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Quarantine {
    pub component: ComponentName,
    pub version: Version,
    pub reason: QuarantineReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct VersionEndpoint {
    pub version: Version,
    pub owner_socket_path: SocketPath,
    pub upgrade_socket_path: SocketPath,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct AttemptHandover {
    pub component: ComponentName,
    pub current: VersionEndpoint,
    pub next: VersionEndpoint,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ForcedFlip {
    pub component: ComponentName,
    pub active_version: Version,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RolledBack {
    pub component: ComponentName,
    pub active_version: Version,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Quarantined {
    pub component: ComponentName,
    pub version: Version,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct HandoverSucceeded {
    pub component: ComponentName,
    pub active_version: Version,
    pub commit_sequence: u64,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RejectionReason {
    UnknownComponent,
    UnknownVersion,
    NotAllowed,
    AlreadyQuarantined,
    NotQuarantined,
    VersionQuarantined,
    HandoverRejected,
    UpgradeSocketUnavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Rejected {
    pub component: ComponentName,
    pub reason: RejectionReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    IntegrationNotLanded,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RequestUnimplemented {
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel OwnerVersionHandover {
        operation AttemptHandover(AttemptHandover),
        operation ForceFlip(ForceFlip),
        operation Rollback(Rollback),
        operation Quarantine(Quarantine),
    }
    reply Reply {
        HandoverSucceeded(HandoverSucceeded),
        FlipForced(ForcedFlip),
        RolledBack(RolledBack),
        Quarantined(Quarantined),
        Rejected(Rejected),
        RequestUnimplemented(RequestUnimplemented),
    }
    observable {
        filter default;
        operation_event OperationReceived;
        effect_event EffectEmitted;
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct OperationReceived {
    pub operation: OperationKind,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EffectEmitted {
    pub observation: SemaObservation,
}
