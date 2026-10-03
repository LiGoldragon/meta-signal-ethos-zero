#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type OrdinarySocketPath = String;
#[rustfmt::skip]
pub type MetaSocketPath = String;
#[rustfmt::skip]
pub type SourceManifestPath = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EthosNexusConfiguration {
    pub ordinary_socket_path: OrdinarySocketPath,
    pub meta_socket_path: MetaSocketPath,
    pub source_manifest_path: SourceManifestPath,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ObservationSelection {
    Configuration,
    Sources,
}
#[rustfmt::skip]
pub type SourceIndex = std::vec::Vec<signal_ethos_zero::FileLocation>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Configure(EthosNexusConfiguration),
    Observe(ObservationSelection),
    Subscribe(ObservationSelection),
    Unsubscribe(ObservationSelection),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Observed_Data {
    Configuration(EthosNexusConfiguration),
    Sources(SourceIndex),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ConfigurationRejected_Data {
    InvalidOrdinarySocketPath,
    InvalidMetaSocketPath,
    InvalidSourceManifestPath,
    InvalidRelativePath(signal_ethos_zero::RelativePath),
    UnreadableSourceManifest,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Configured(EthosNexusConfiguration),
    Observed(Observed_Data),
    ConfigurationRejected(ConfigurationRejected_Data),
    ConfigurationChanged(EthosNexusConfiguration),
    SourcesChanged(SourceIndex),
}
