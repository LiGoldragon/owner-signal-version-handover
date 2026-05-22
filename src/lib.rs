//! Owner-only signal contract for version-handover authority.
//!
//! The ordinary `signal-version-handover` contract carries the private
//! daemon-to-daemon handover protocol. This owner contract carries emergency
//! and administrative authority for the persona engine: force an active
//! selector flip, roll back a recent flip, or quarantine a component version.

use nota_codec::{NotaEnum, NotaRecord};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
use signal_sema::SemaObservation;
use version_projection::{ComponentName, ContractVersion};

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
    pub current_version: ContractVersion,
    pub target_version: ContractVersion,
    pub reason: ForceReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Rollback {
    pub component: ComponentName,
    pub active_version: ContractVersion,
    pub restore_version: ContractVersion,
    pub reason: RollbackReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Quarantine {
    pub component: ComponentName,
    pub version: ContractVersion,
    pub reason: QuarantineReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ForcedFlip {
    pub component: ComponentName,
    pub active_version: ContractVersion,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RolledBack {
    pub component: ComponentName,
    pub active_version: ContractVersion,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Quarantined {
    pub component: ComponentName,
    pub version: ContractVersion,
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
        operation ForceFlip(ForceFlip),
        operation Rollback(Rollback),
        operation Quarantine(Quarantine),
    }
    reply Reply {
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
