use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use owner_signal_version_handover::{
    ForceFlip, ForceReason, ForcedFlip, Frame, FrameBody, Operation, OperationKind, Quarantine,
    QuarantineReason, Quarantined, Rejected, RejectionReason, Reply, Rollback, RollbackReason,
    RolledBack, Version, VersionLabel,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply as FrameReply, RequestPayload,
    SessionEpoch, SubReply,
};
use version_projection::{ComponentName, ContractVersion};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn component() -> ComponentName {
    ComponentName::new("persona-spirit")
}

fn version(byte: u8) -> ContractVersion {
    ContractVersion::new([byte; 32])
}

fn component_version(label: &str, byte: u8) -> Version {
    Version::new(VersionLabel::new(label), version(byte))
}

fn force_flip() -> ForceFlip {
    ForceFlip {
        component: component(),
        current_version: component_version("v0.1.0", 1),
        target_version: component_version("v0.1.1", 2),
        reason: ForceReason::OperatorOverride,
    }
}

fn rollback() -> Rollback {
    Rollback {
        component: component(),
        active_version: component_version("v0.1.1", 2),
        restore_version: component_version("v0.1.0", 1),
        reason: RollbackReason::PostCutoverFailure,
    }
}

fn quarantine() -> Quarantine {
    Quarantine {
        component: component(),
        version: component_version("v0.1.1", 2),
        reason: QuarantineReason::FailedUpgrade,
    }
}

fn round_trip_request(operation: Operation) -> Operation {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: operation.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn round_trip_reply(reply: Reply) -> Reply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: FrameReply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            FrameReply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted frame reply, got {other:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode nota");
    let encoded = encoder.into_string();
    assert_eq!(encoded, expected);

    let mut decoder = Decoder::new(&encoded);
    let recovered = T::decode(&mut decoder).expect("decode nota");
    assert_eq!(recovered, value);
    assert!(
        CANONICAL.contains(expected),
        "examples/canonical.nota missing line: {expected}"
    );
}

#[test]
fn owner_requests_round_trip_through_signal_frames() {
    let operations = [
        Operation::ForceFlip(force_flip()),
        Operation::Rollback(rollback()),
        Operation::Quarantine(quarantine()),
    ];

    for operation in operations {
        assert_eq!(round_trip_request(operation.clone()), operation);
    }
}

#[test]
fn owner_replies_round_trip_through_signal_frames() {
    let replies = [
        Reply::FlipForced(ForcedFlip {
            component: component(),
            active_version: component_version("v0.1.1", 2),
        }),
        Reply::RolledBack(RolledBack {
            component: component(),
            active_version: component_version("v0.1.0", 1),
        }),
        Reply::Quarantined(Quarantined {
            component: component(),
            version: component_version("v0.1.1", 2),
        }),
        Reply::Rejected(Rejected {
            component: component(),
            reason: RejectionReason::AlreadyQuarantined,
        }),
    ];

    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn operation_kinds_are_generated_from_authority_operations() {
    assert_eq!(
        Operation::ForceFlip(force_flip()).kind(),
        OperationKind::ForceFlip
    );
    assert_eq!(
        Operation::Rollback(rollback()).kind(),
        OperationKind::Rollback
    );
    assert_eq!(
        Operation::Quarantine(quarantine()).kind(),
        OperationKind::Quarantine
    );
}

#[test]
fn canonical_nota_examples_round_trip() {
    round_trip_nota(
        Operation::ForceFlip(force_flip()),
        r#"(ForceFlip (persona-spirit ("v0.1.0" #0101010101010101010101010101010101010101010101010101010101010101) ("v0.1.1" #0202020202020202020202020202020202020202020202020202020202020202) OperatorOverride))"#,
    );
    round_trip_nota(
        Operation::Rollback(rollback()),
        r#"(Rollback (persona-spirit ("v0.1.1" #0202020202020202020202020202020202020202020202020202020202020202) ("v0.1.0" #0101010101010101010101010101010101010101010101010101010101010101) PostCutoverFailure))"#,
    );
    round_trip_nota(
        Operation::Quarantine(quarantine()),
        r#"(Quarantine (persona-spirit ("v0.1.1" #0202020202020202020202020202020202020202020202020202020202020202) FailedUpgrade))"#,
    );
    round_trip_nota(
        Reply::FlipForced(ForcedFlip {
            component: component(),
            active_version: component_version("v0.1.1", 2),
        }),
        r#"(FlipForced (persona-spirit ("v0.1.1" #0202020202020202020202020202020202020202020202020202020202020202)))"#,
    );
    round_trip_nota(
        Reply::RolledBack(RolledBack {
            component: component(),
            active_version: component_version("v0.1.0", 1),
        }),
        r#"(RolledBack (persona-spirit ("v0.1.0" #0101010101010101010101010101010101010101010101010101010101010101)))"#,
    );
    round_trip_nota(
        Reply::Quarantined(Quarantined {
            component: component(),
            version: component_version("v0.1.1", 2),
        }),
        r#"(Quarantined (persona-spirit ("v0.1.1" #0202020202020202020202020202020202020202020202020202020202020202)))"#,
    );
    round_trip_nota(
        Reply::Rejected(Rejected {
            component: component(),
            reason: RejectionReason::AlreadyQuarantined,
        }),
        "(Rejected (persona-spirit AlreadyQuarantined))",
    );
}
