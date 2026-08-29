use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
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
pub const INTERFACE_VERSION: ProtocolVersion = ProtocolVersion::new(0u16, 3u16, 0u16);
pub const CHANNEL_CONTRACT_ID: ChannelContractId = ChannelContractId(2u32);
pub const CHANNEL_WIRE_REVISION: ChannelWireRevision = ChannelWireRevision(3u16);
pub const PROTOCOL_VERSION: ProtocolVersion = INTERFACE_VERSION;
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct OrdinarySocketPath(String);
impl OrdinarySocketPath {
    pub fn try_from_string(
        value: String,
    ) -> std::result::Result<Self, datomic::UnrepresentableString> {
        datomic::DatomicString::try_from(value).map(|value| Self(value.as_ref().to_owned()))
    }
}
impl std::convert::TryFrom<String> for OrdinarySocketPath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}
impl<'a> std::convert::TryFrom<&'a str> for OrdinarySocketPath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value.to_owned())
    }
}
impl AsRef<str> for OrdinarySocketPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct MetaSocketPath(String);
impl MetaSocketPath {
    pub fn try_from_string(
        value: String,
    ) -> std::result::Result<Self, datomic::UnrepresentableString> {
        datomic::DatomicString::try_from(value).map(|value| Self(value.as_ref().to_owned()))
    }
}
impl std::convert::TryFrom<String> for MetaSocketPath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}
impl<'a> std::convert::TryFrom<&'a str> for MetaSocketPath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value.to_owned())
    }
}
impl AsRef<str> for MetaSocketPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SourceManifestPath(String);
impl SourceManifestPath {
    pub fn try_from_string(
        value: String,
    ) -> std::result::Result<Self, datomic::UnrepresentableString> {
        datomic::DatomicString::try_from(value).map(|value| Self(value.as_ref().to_owned()))
    }
}
impl std::convert::TryFrom<String> for SourceManifestPath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}
impl<'a> std::convert::TryFrom<&'a str> for SourceManifestPath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value.to_owned())
    }
}
impl AsRef<str> for SourceManifestPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Configuration {
    pub ordinary_socket_path: OrdinarySocketPath,
    pub meta_socket_path: MetaSocketPath,
    pub source_manifest_path: SourceManifestPath,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct MetaSubscriptionRequest {
    pub selection: MetaObservationSelection,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum MetaObservationSelection {
    Configuration,
    Sources,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SourceName(String);
impl SourceName {
    pub fn try_from_string(
        value: String,
    ) -> std::result::Result<Self, datomic::UnrepresentableString> {
        datomic::DatomicString::try_from(value).map(|value| Self(value.as_ref().to_owned()))
    }
}
impl std::convert::TryFrom<String> for SourceName {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}
impl<'a> std::convert::TryFrom<&'a str> for SourceName {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value.to_owned())
    }
}
impl AsRef<str> for SourceName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RelativePath(String);
impl RelativePath {
    pub fn try_from_string(
        value: String,
    ) -> std::result::Result<Self, datomic::UnrepresentableString> {
        datomic::DatomicString::try_from(value).map(|value| Self(value.as_ref().to_owned()))
    }
}
impl std::convert::TryFrom<String> for RelativePath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}
impl<'a> std::convert::TryFrom<&'a str> for RelativePath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value.to_owned())
    }
}
impl AsRef<str> for RelativePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
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
    InvalidRelativePath(RelativePath),
    UnreadableSourceManifest,
}
impl datomic::Datomic for OrdinarySocketPath {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(
            <datomic::DatomicString as datomic::Datomic>::embody(portion)?
                .as_ref()
                .to_owned(),
        ))
    }
    fn portion(&self) -> protos::Portion {
        datomic::DatomicString::try_from(self.0.clone()).map_or_else(
            |_| datomic::PortionBuilding::bare("wire-invalid"),
            |value| datomic::Datomic::portion(&value),
        )
    }
}
impl datomic::Datomic for MetaSocketPath {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(
            <datomic::DatomicString as datomic::Datomic>::embody(portion)?
                .as_ref()
                .to_owned(),
        ))
    }
    fn portion(&self) -> protos::Portion {
        datomic::DatomicString::try_from(self.0.clone()).map_or_else(
            |_| datomic::PortionBuilding::bare("wire-invalid"),
            |value| datomic::Datomic::portion(&value),
        )
    }
}
impl datomic::Datomic for SourceManifestPath {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(
            <datomic::DatomicString as datomic::Datomic>::embody(portion)?
                .as_ref()
                .to_owned(),
        ))
    }
    fn portion(&self) -> protos::Portion {
        datomic::DatomicString::try_from(self.0.clone()).map_or_else(
            |_| datomic::PortionBuilding::bare("wire-invalid"),
            |value| datomic::Datomic::portion(&value),
        )
    }
}
impl datomic::Datomic for Configuration {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 3usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            ordinary_socket_path: <OrdinarySocketPath as datomic::Datomic>::embody(&parts[0usize])?,
            meta_socket_path: <MetaSocketPath as datomic::Datomic>::embody(&parts[1usize])?,
            source_manifest_path: <SourceManifestPath as datomic::Datomic>::embody(&parts[2usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![
                datomic::Datomic::portion(&self.ordinary_socket_path),
                datomic::Datomic::portion(&self.meta_socket_path),
                datomic::Datomic::portion(&self.source_manifest_path),
            ],
        )
    }
}
impl datomic::Datomic for MetaSubscriptionRequest {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 1usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            selection: <MetaObservationSelection as datomic::Datomic>::embody(&parts[0usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![datomic::Datomic::portion(&self.selection)],
        )
    }
}
impl datomic::Datomic for MetaObservationSelection {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Configuration)) {
            return Ok(Self::Configuration);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Sources)) {
            return Ok(Self::Sources);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Configuration => datomic::PortionBuilding::bare(stringify!(Configuration)),
            Self::Sources => datomic::PortionBuilding::bare(stringify!(Sources)),
        }
    }
}
impl datomic::Datomic for SourceName {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(
            <datomic::DatomicString as datomic::Datomic>::embody(portion)?
                .as_ref()
                .to_owned(),
        ))
    }
    fn portion(&self) -> protos::Portion {
        datomic::DatomicString::try_from(self.0.clone()).map_or_else(
            |_| datomic::PortionBuilding::bare("wire-invalid"),
            |value| datomic::Datomic::portion(&value),
        )
    }
}
impl datomic::Datomic for RelativePath {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(
            <datomic::DatomicString as datomic::Datomic>::embody(portion)?
                .as_ref()
                .to_owned(),
        ))
    }
    fn portion(&self) -> protos::Portion {
        datomic::DatomicString::try_from(self.0.clone()).map_or_else(
            |_| datomic::PortionBuilding::bare("wire-invalid"),
            |value| datomic::Datomic::portion(&value),
        )
    }
}
impl datomic::Datomic for Source {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 2usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            source_name: <SourceName as datomic::Datomic>::embody(&parts[0usize])?,
            relative_path: <RelativePath as datomic::Datomic>::embody(&parts[1usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![
                datomic::Datomic::portion(&self.source_name),
                datomic::Datomic::portion(&self.relative_path),
            ],
        )
    }
}
impl datomic::Datomic for Sources {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(<Vec<Source> as datomic::Datomic>::embody(portion)?))
    }
    fn portion(&self) -> protos::Portion {
        <Vec<Source> as datomic::Datomic>::portion(&self.0)
    }
}
impl datomic::Datomic for SourceIndex {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 1usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            sources: <Sources as datomic::Datomic>::embody(&parts[0usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![datomic::Datomic::portion(&self.sources)],
        )
    }
}
impl datomic::Datomic for MetaObservation {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Configuration)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Configuration(
                <Configuration as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Sources)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Sources(<SourceIndex as datomic::Datomic>::embody(
                &headed.body,
            )?));
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Configuration(value) => datomic::PortionBuilding::headed(
                stringify!(Configuration),
                protos::Separator::Period,
                <Configuration as datomic::Datomic>::portion(value),
            ),
            Self::Sources(value) => datomic::PortionBuilding::headed(
                stringify!(Sources),
                protos::Separator::Period,
                <SourceIndex as datomic::Datomic>::portion(value),
            ),
        }
    }
}
impl datomic::Datomic for ConfigurationRefusal {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion)
            == Some(stringify!(InvalidOrdinarySocketPath))
        {
            return Ok(Self::InvalidOrdinarySocketPath);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(InvalidMetaSocketPath))
        {
            return Ok(Self::InvalidMetaSocketPath);
        }
        if datomic::PortionViewing::bare_symbol(portion)
            == Some(stringify!(InvalidSourceManifestPath))
        {
            return Ok(Self::InvalidSourceManifestPath);
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(InvalidRelativePath)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::InvalidRelativePath(
                <RelativePath as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if datomic::PortionViewing::bare_symbol(portion)
            == Some(stringify!(UnreadableSourceManifest))
        {
            return Ok(Self::UnreadableSourceManifest);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::InvalidOrdinarySocketPath => {
                datomic::PortionBuilding::bare(stringify!(InvalidOrdinarySocketPath))
            }
            Self::InvalidMetaSocketPath => {
                datomic::PortionBuilding::bare(stringify!(InvalidMetaSocketPath))
            }
            Self::InvalidSourceManifestPath => {
                datomic::PortionBuilding::bare(stringify!(InvalidSourceManifestPath))
            }
            Self::InvalidRelativePath(value) => datomic::PortionBuilding::headed(
                stringify!(InvalidRelativePath),
                protos::Separator::Period,
                <RelativePath as datomic::Datomic>::portion(value),
            ),
            Self::UnreadableSourceManifest => {
                datomic::PortionBuilding::bare(stringify!(UnreadableSourceManifest))
            }
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Configure(Configuration),
    Observe(MetaObservationSelection),
    Subscribe(MetaSubscriptionRequest),
    Unsubscribe(MetaSubscriptionRequest),
}
impl datomic::Datomic for Request {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Configure)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Configure(
                <Configuration as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Observe)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Observe(
                <MetaObservationSelection as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Subscribe)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Subscribe(
                <MetaSubscriptionRequest as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Unsubscribe)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Unsubscribe(
                <MetaSubscriptionRequest as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Configure(value) => datomic::PortionBuilding::headed(
                stringify!(Configure),
                protos::Separator::Period,
                <Configuration as datomic::Datomic>::portion(value),
            ),
            Self::Observe(value) => datomic::PortionBuilding::headed(
                stringify!(Observe),
                protos::Separator::Period,
                <MetaObservationSelection as datomic::Datomic>::portion(value),
            ),
            Self::Subscribe(value) => datomic::PortionBuilding::headed(
                stringify!(Subscribe),
                protos::Separator::Period,
                <MetaSubscriptionRequest as datomic::Datomic>::portion(value),
            ),
            Self::Unsubscribe(value) => datomic::PortionBuilding::headed(
                stringify!(Unsubscribe),
                protos::Separator::Period,
                <MetaSubscriptionRequest as datomic::Datomic>::portion(value),
            ),
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Reply {
    Configured(Configuration),
    Observed(MetaObservation),
    ConfigurationRejected(ConfigurationRefusal),
}
impl datomic::Datomic for Reply {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Configured)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Configured(
                <Configuration as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Observed)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Observed(
                <MetaObservation as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(ConfigurationRejected)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::ConfigurationRejected(
                <ConfigurationRefusal as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Configured(value) => datomic::PortionBuilding::headed(
                stringify!(Configured),
                protos::Separator::Period,
                <Configuration as datomic::Datomic>::portion(value),
            ),
            Self::Observed(value) => datomic::PortionBuilding::headed(
                stringify!(Observed),
                protos::Separator::Period,
                <MetaObservation as datomic::Datomic>::portion(value),
            ),
            Self::ConfigurationRejected(value) => datomic::PortionBuilding::headed(
                stringify!(ConfigurationRejected),
                protos::Separator::Period,
                <ConfigurationRefusal as datomic::Datomic>::portion(value),
            ),
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    InvalidRelativePath(RelativePath),
}
impl datomic::Datomic for Refusal {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(InvalidRelativePath)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::InvalidRelativePath(
                <RelativePath as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::InvalidRelativePath(value) => datomic::PortionBuilding::headed(
                stringify!(InvalidRelativePath),
                protos::Separator::Period,
                <RelativePath as datomic::Datomic>::portion(value),
            ),
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Stream {
    ConfigurationChanged(Configuration),
    SourcesChanged(SourceIndex),
}
impl datomic::Datomic for Stream {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(ConfigurationChanged)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::ConfigurationChanged(
                <Configuration as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(SourcesChanged)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::SourcesChanged(
                <SourceIndex as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::ConfigurationChanged(value) => datomic::PortionBuilding::headed(
                stringify!(ConfigurationChanged),
                protos::Separator::Period,
                <Configuration as datomic::Datomic>::portion(value),
            ),
            Self::SourcesChanged(value) => datomic::PortionBuilding::headed(
                stringify!(SourcesChanged),
                protos::Separator::Period,
                <SourceIndex as datomic::Datomic>::portion(value),
            ),
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FrameBody {
    Request(Request),
    Reply(Reply),
    Refusal(Refusal),
    Event(Stream),
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    pub channel_contract_id: ChannelContractId,
    pub channel_wire_revision: ChannelWireRevision,
    pub protocol_version: ProtocolVersion,
    pub body: FrameBody,
}
