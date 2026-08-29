use datomic::{Datomic, TextEdge};
use meta_signal_ethos_zero::*;
use protos::{PortionText, Text};

fn string<T: TryFrom<String>>(value: &str) -> T
where
    T::Error: std::fmt::Debug,
{
    value.to_owned().try_into().expect("representable string")
}

fn configuration() -> Configuration {
    Configuration {
        ordinary_socket_path: string("/run/ethos-zero.sock"),
        meta_socket_path: string("/run/ethos-zero-meta.sock"),
        source_manifest_path: string("/etc/ethos-zero/sources.datom"),
    }
}

fn source_index() -> SourceIndex {
    SourceIndex {
        sources: Sources(vec![Source {
            source_name: string("workspace"),
            relative_path: string("ethos/signal.ethos"),
        }]),
    }
}

fn frame(body: FrameBody) -> Frame {
    Frame {
        channel_contract_id: CHANNEL_CONTRACT_ID,
        channel_wire_revision: CHANNEL_WIRE_REVISION,
        protocol_version: PROTOCOL_VERSION,
        body,
    }
}

fn raw_length_prefixed(value: &Frame) -> Vec<u8> {
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(value).expect("archive frame");
    let length = u32::try_from(archive.len()).expect("frame length fits prefix");
    let mut bytes = length.to_le_bytes().to_vec();
    bytes.extend_from_slice(&archive);
    bytes
}

fn assert_frame_round_trip(value: Frame) {
    let encoded = value.encode_length_prefixed().expect("encode signal");
    assert_eq!(
        Frame::decode_length_prefixed(&encoded).expect("decode signal"),
        value
    );
}

fn assert_text_round_trip<T>(value: T)
where
    T: Datomic + Clone + std::fmt::Debug + PartialEq,
{
    let text = Datomic::portion(&value).canonical_text();
    assert_eq!(
        Text::<T>::from(text.as_ref())
            .embody()
            .expect("Datomic text embodiment"),
        value
    );
}

#[test]
fn every_root_executes_through_datomic_text_and_rkyv_frames() {
    let requests = [
        Request::Configure(configuration()),
        Request::Observe(MetaObservationSelection::Configuration),
        Request::Observe(MetaObservationSelection::Sources),
        Request::Subscribe(MetaSubscriptionRequest {
            selection: MetaObservationSelection::Sources,
        }),
        Request::Unsubscribe(MetaSubscriptionRequest {
            selection: MetaObservationSelection::Configuration,
        }),
    ];
    for request in requests {
        assert_text_round_trip(request.clone());
        assert_frame_round_trip(frame(FrameBody::Request(request)));
    }

    let replies = [
        Reply::Configured(configuration()),
        Reply::Observed(MetaObservation::Configuration(configuration())),
        Reply::Observed(MetaObservation::Sources(source_index())),
        Reply::ConfigurationRejected(ConfigurationRefusal::InvalidOrdinarySocketPath),
        Reply::ConfigurationRejected(ConfigurationRefusal::InvalidMetaSocketPath),
        Reply::ConfigurationRejected(ConfigurationRefusal::InvalidSourceManifestPath),
        Reply::ConfigurationRejected(ConfigurationRefusal::InvalidRelativePath(string(
            "../outside",
        ))),
        Reply::ConfigurationRejected(ConfigurationRefusal::UnreadableSourceManifest),
    ];
    for reply in replies {
        assert_text_round_trip(reply.clone());
        assert_frame_round_trip(frame(FrameBody::Reply(reply)));
    }

    let refusal = Refusal::InvalidRelativePath(string("../outside"));
    assert_text_round_trip(refusal.clone());
    assert_frame_round_trip(frame(FrameBody::Refusal(refusal)));

    let events = [
        Stream::ConfigurationChanged(configuration()),
        Stream::SourcesChanged(source_index()),
    ];
    for event in events {
        assert_text_round_trip(event.clone());
        assert_frame_round_trip(frame(FrameBody::Event(event)));
    }
}

#[test]
fn malformed_frames_and_wrong_metadata_are_rejected() {
    assert_eq!(
        Frame::decode_length_prefixed(&[]),
        Err(FrameCodecError::LengthPrefixMissing)
    );
    assert!(matches!(
        Frame::decode_length_prefixed(&[1, 0, 0, 0]),
        Err(FrameCodecError::LengthMismatch { .. })
    ));
    assert_eq!(
        Frame::decode_length_prefixed(&[1, 0, 0, 0, 0]),
        Err(FrameCodecError::ArchiveDecode)
    );

    let wrong_protocol = Frame {
        protocol_version: ProtocolVersion::new(0, 2, 0),
        ..frame(FrameBody::Request(Request::Observe(
            MetaObservationSelection::Sources,
        )))
    };
    assert!(matches!(
        wrong_protocol.encode_length_prefixed(),
        Err(FrameCodecError::UnsupportedProtocol { .. })
    ));
    assert!(matches!(
        Frame::decode_length_prefixed(&raw_length_prefixed(&wrong_protocol)),
        Err(FrameCodecError::UnsupportedProtocol { .. })
    ));

    let wrong_contract = Frame {
        channel_contract_id: ChannelContractId(1),
        ..frame(FrameBody::Request(Request::Observe(
            MetaObservationSelection::Sources,
        )))
    };
    assert!(matches!(
        wrong_contract.encode_length_prefixed(),
        Err(FrameCodecError::WrongChannelContract { .. })
    ));
    assert!(matches!(
        Frame::decode_length_prefixed(&raw_length_prefixed(&wrong_contract)),
        Err(FrameCodecError::WrongChannelContract { .. })
    ));

    let wrong_revision = Frame {
        channel_wire_revision: ChannelWireRevision(2),
        ..frame(FrameBody::Request(Request::Observe(
            MetaObservationSelection::Sources,
        )))
    };
    assert!(matches!(
        wrong_revision.encode_length_prefixed(),
        Err(FrameCodecError::WrongChannelWireRevision { .. })
    ));
    assert!(matches!(
        Frame::decode_length_prefixed(&raw_length_prefixed(&wrong_revision)),
        Err(FrameCodecError::WrongChannelWireRevision { .. })
    ));
}
