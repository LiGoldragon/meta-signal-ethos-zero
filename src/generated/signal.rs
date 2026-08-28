//! Hand-written generation-zero projection of `ethos/signal.ethos`.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

pub const INTERFACE_VERSION: ProtocolVersion = ProtocolVersion::new(0, 2, 0);
pub const CHANNEL_CONTRACT_ID: ChannelContractId = ChannelContractId(2);
pub const CHANNEL_WIRE_REVISION: ChannelWireRevision = ChannelWireRevision(2);
pub const PROTOCOL_VERSION: ProtocolVersion = INTERFACE_VERSION;

/// The only binary boundary for this contract.
pub trait SignalFrameCodec: Sized {
    fn encode_length_prefixed(&self) -> Result<Vec<u8>, FrameCodecError>;
    fn decode_length_prefixed(bytes: &[u8]) -> Result<Self, FrameCodecError>;
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}
impl ProtocolVersion {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChannelContractId(pub u32);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChannelWireRevision(pub u16);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct OrdinarySocketPath(pub String);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct MetaSocketPath(pub String);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SourceManifestPath(pub String);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Configuration {
    pub ordinary_socket_path: OrdinarySocketPath,
    pub meta_socket_path: MetaSocketPath,
    pub source_manifest_path: SourceManifestPath,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum MetaObservationSelection {
    Configuration,
    Sources,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SourceName(pub String);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RelativePath(pub String);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Source {
    pub source_name: SourceName,
    pub relative_path: RelativePath,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Sources(pub Vec<Source>);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SourceIndex {
    pub sources: Sources,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum MetaObservation {
    Configuration(Configuration),
    Sources(SourceIndex),
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum ConfigurationRefusal {
    InvalidOrdinarySocketPath,
    InvalidMetaSocketPath,
    InvalidSourceManifestPath,
    UnreadableSourceManifest,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Configure(Configuration),
    Observe(MetaObservationSelection),
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Reply {
    Configured(Configuration),
    Observed(MetaObservation),
    ConfigurationRejected(ConfigurationRefusal),
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FrameBody {
    Request(Request),
    Reply(Reply),
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    pub channel_contract_id: ChannelContractId,
    pub channel_wire_revision: ChannelWireRevision,
    pub protocol_version: ProtocolVersion,
    pub body: FrameBody,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FrameCodecError {
    LengthPrefixMissing,
    LengthMismatch {
        expected: usize,
        found: usize,
    },
    LengthTooLarge,
    ArchiveEncode,
    ArchiveDecode,
    WrongChannelContract {
        expected: ChannelContractId,
        found: ChannelContractId,
    },
    WrongChannelWireRevision {
        expected: ChannelWireRevision,
        found: ChannelWireRevision,
    },
    UnsupportedProtocol {
        expected: ProtocolVersion,
        found: ProtocolVersion,
    },
}

impl SignalFrameCodec for Frame {
    fn encode_length_prefixed(&self) -> Result<Vec<u8>, FrameCodecError> {
        if self.channel_contract_id != CHANNEL_CONTRACT_ID {
            return Err(FrameCodecError::WrongChannelContract {
                expected: CHANNEL_CONTRACT_ID,
                found: self.channel_contract_id,
            });
        }
        if self.channel_wire_revision != CHANNEL_WIRE_REVISION {
            return Err(FrameCodecError::WrongChannelWireRevision {
                expected: CHANNEL_WIRE_REVISION,
                found: self.channel_wire_revision,
            });
        }
        if self.protocol_version != PROTOCOL_VERSION {
            return Err(FrameCodecError::UnsupportedProtocol {
                expected: PROTOCOL_VERSION,
                found: self.protocol_version,
            });
        }
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map_err(|_| FrameCodecError::ArchiveEncode)?;
        let length = u32::try_from(archive.len()).map_err(|_| FrameCodecError::LengthTooLarge)?;
        let mut frame = Vec::with_capacity(4 + archive.len());
        frame.extend_from_slice(&length.to_le_bytes());
        frame.extend_from_slice(&archive);
        Ok(frame)
    }

    fn decode_length_prefixed(bytes: &[u8]) -> Result<Self, FrameCodecError> {
        let Some(prefix) = bytes.get(..4) else {
            return Err(FrameCodecError::LengthPrefixMissing);
        };
        let expected = u32::from_le_bytes([prefix[0], prefix[1], prefix[2], prefix[3]]) as usize;
        let payload = &bytes[4..];
        if payload.len() != expected {
            return Err(FrameCodecError::LengthMismatch {
                expected,
                found: payload.len(),
            });
        }
        let frame = rkyv::from_bytes::<Self, rkyv::rancor::Error>(payload)
            .map_err(|_| FrameCodecError::ArchiveDecode)?;
        if frame.channel_contract_id != CHANNEL_CONTRACT_ID {
            return Err(FrameCodecError::WrongChannelContract {
                expected: CHANNEL_CONTRACT_ID,
                found: frame.channel_contract_id,
            });
        }
        if frame.channel_wire_revision != CHANNEL_WIRE_REVISION {
            return Err(FrameCodecError::WrongChannelWireRevision {
                expected: CHANNEL_WIRE_REVISION,
                found: frame.channel_wire_revision,
            });
        }
        if frame.protocol_version != PROTOCOL_VERSION {
            return Err(FrameCodecError::UnsupportedProtocol {
                expected: PROTOCOL_VERSION,
                found: frame.protocol_version,
            });
        }
        Ok(frame)
    }
}
