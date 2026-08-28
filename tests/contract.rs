use meta_signal_ethos_zero::*;

fn configuration() -> Configuration {
    Configuration {
        ordinary_socket_path: OrdinarySocketPath("/run/ethos-zero.sock".into()),
        meta_socket_path: MetaSocketPath("/run/ethos-zero-meta.sock".into()),
        source_manifest_path: SourceManifestPath("/etc/ethos-zero/sources.datom".into()),
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

#[test]
fn every_meta_request_and_reply_root_round_trips() {
    let requests = [
        Request::Configure(configuration()),
        Request::Observe(MetaObservationSelection::Configuration),
        Request::Observe(MetaObservationSelection::Sources),
    ];
    for request in requests {
        assert_frame_round_trip(frame(FrameBody::Request(request)));
    }

    let replies = [
        Reply::Configured(configuration()),
        Reply::Observed(MetaObservation::Configuration(configuration())),
        Reply::Observed(MetaObservation::Sources(SourceIndex {
            sources: Sources(vec![Source {
                source_name: SourceName("workspace".into()),
                relative_path: RelativePath("ethos/signal.ethos".into()),
            }]),
        })),
        Reply::ConfigurationRejected(ConfigurationRefusal::InvalidOrdinarySocketPath),
        Reply::ConfigurationRejected(ConfigurationRefusal::InvalidMetaSocketPath),
        Reply::ConfigurationRejected(ConfigurationRefusal::InvalidSourceManifestPath),
        Reply::ConfigurationRejected(ConfigurationRefusal::UnreadableSourceManifest),
    ];
    for reply in replies {
        assert_frame_round_trip(frame(FrameBody::Reply(reply)));
    }
}

#[test]
fn malformed_frames_are_rejected() {
    assert_eq!(
        Frame::decode_length_prefixed(&[]),
        Err(FrameCodecError::LengthPrefixMissing)
    );
    assert!(matches!(
        Frame::decode_length_prefixed(&[1, 0, 0, 0]),
        Err(FrameCodecError::LengthMismatch {
            expected: 1,
            found: 0
        })
    ));
    assert_eq!(
        Frame::decode_length_prefixed(&[1, 0, 0, 0, 0]),
        Err(FrameCodecError::ArchiveDecode)
    );
}

#[test]
fn protocol_versions_are_validated_on_encode_and_decode() {
    let value = Frame {
        protocol_version: ProtocolVersion::new(0, 1, 1),
        ..frame(FrameBody::Request(Request::Observe(
            MetaObservationSelection::Sources,
        )))
    };
    assert_eq!(
        value.encode_length_prefixed(),
        Err(FrameCodecError::UnsupportedProtocol {
            expected: PROTOCOL_VERSION,
            found: ProtocolVersion::new(0, 1, 1)
        })
    );
    assert_eq!(
        Frame::decode_length_prefixed(&raw_length_prefixed(&value)),
        Err(FrameCodecError::UnsupportedProtocol {
            expected: PROTOCOL_VERSION,
            found: ProtocolVersion::new(0, 1, 1)
        })
    );
}

#[test]
fn contract_identity_and_wire_revision_are_validated_on_encode_and_decode() {
    let wrong_contract = Frame {
        channel_contract_id: ChannelContractId(1),
        ..frame(FrameBody::Request(Request::Observe(
            MetaObservationSelection::Sources,
        )))
    };
    assert_eq!(
        wrong_contract.encode_length_prefixed(),
        Err(FrameCodecError::WrongChannelContract {
            expected: CHANNEL_CONTRACT_ID,
            found: ChannelContractId(1)
        })
    );
    assert_eq!(
        Frame::decode_length_prefixed(&raw_length_prefixed(&wrong_contract)),
        Err(FrameCodecError::WrongChannelContract {
            expected: CHANNEL_CONTRACT_ID,
            found: ChannelContractId(1)
        })
    );

    let wrong_revision = Frame {
        channel_wire_revision: ChannelWireRevision(1),
        ..frame(FrameBody::Request(Request::Observe(
            MetaObservationSelection::Sources,
        )))
    };
    assert_eq!(
        wrong_revision.encode_length_prefixed(),
        Err(FrameCodecError::WrongChannelWireRevision {
            expected: CHANNEL_WIRE_REVISION,
            found: ChannelWireRevision(1)
        })
    );
    assert_eq!(
        Frame::decode_length_prefixed(&raw_length_prefixed(&wrong_revision)),
        Err(FrameCodecError::WrongChannelWireRevision {
            expected: CHANNEL_WIRE_REVISION,
            found: ChannelWireRevision(1)
        })
    );
}
