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

#[test]
fn concrete_configuration_round_trips_through_the_framed_signal() {
    let value = frame(FrameBody::Request(Request::Configure(configuration())));
    let encoded = value
        .encode_length_prefixed()
        .expect("encode configuration");
    assert_eq!(
        Frame::decode_length_prefixed(&encoded).expect("decode configuration"),
        value
    );
}

#[test]
fn concrete_observations_and_refusal_round_trip() {
    let values = [
        frame(FrameBody::Reply(Reply::Configured(configuration()))),
        frame(FrameBody::Reply(Reply::Observed(
            MetaObservation::Configuration(configuration()),
        ))),
        frame(FrameBody::Reply(Reply::Observed(MetaObservation::Sources(
            SourceIndex {
                sources: Sources(vec![Source {
                    source_name: SourceName("workspace".into()),
                    relative_path: RelativePath("ethos/signal.ethos".into()),
                }]),
            },
        )))),
        frame(FrameBody::Reply(Reply::ConfigurationRejected(
            ConfigurationRefusal::InvalidOrdinarySocketPath,
        ))),
        frame(FrameBody::Reply(Reply::ConfigurationRejected(
            ConfigurationRefusal::InvalidMetaSocketPath,
        ))),
        frame(FrameBody::Reply(Reply::ConfigurationRejected(
            ConfigurationRefusal::InvalidSourceManifestPath,
        ))),
        frame(FrameBody::Reply(Reply::ConfigurationRejected(
            ConfigurationRefusal::UnreadableSourceManifest,
        ))),
    ];
    for value in values {
        let encoded = value.encode_length_prefixed().expect("encode reply");
        assert_eq!(
            Frame::decode_length_prefixed(&encoded).expect("decode reply"),
            value
        );
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
fn protocol_versions_are_validated() {
    let value = Frame {
        protocol_version: ProtocolVersion::new(0, 1, 1),
        body: FrameBody::Request(Request::Observe(MetaObservationSelection::Sources)),
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
