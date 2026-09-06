#![allow(dead_code)]
#![allow(clippy::redundant_closure)]
pub type OrdinarySocketPath = protos::Text;
pub type MetaSocketPath = protos::Text;
pub type SourceManifestPath = protos::Text;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Configuration(
    pub OrdinarySocketPath,
    pub MetaSocketPath,
    pub SourceManifestPath,
);
impl datom_codec::Datomic for Configuration {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: OrdinarySocketPath = datom_codec::Positional::position(&mut p)?;
        let p1: MetaSocketPath = datom_codec::Positional::position(&mut p)?;
        let p2: SourceManifestPath = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for Configuration {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MetaSubscriptionRequest(pub MetaObservationSelection);
impl datom_codec::Datomic for MetaSubscriptionRequest {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: MetaObservationSelection = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for MetaSubscriptionRequest {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetaObservationSelection {
    Configuration,
    Sources,
}
impl datom_codec::Datomic for MetaObservationSelection {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Configuration" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Configuration)
            }
            "Sources" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Sources)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for MetaObservationSelection {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Configuration => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Configuration")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Sources => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Sources").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
pub type SourceName = protos::Text;
pub type RelativePath = protos::Text;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source(pub SourceName, pub RelativePath);
impl datom_codec::Datomic for Source {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: SourceName = datom_codec::Positional::position(&mut p)?;
        let p1: RelativePath = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for Source {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type Sources = std::vec::Vec<Source>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceIndex(pub Sources);
impl datom_codec::Datomic for SourceIndex {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: Sources = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for SourceIndex {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MetaObservation {
    Configuration(Configuration),
    Sources(SourceIndex),
}
impl datom_codec::Datomic for MetaObservation {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Configuration" => {
                std::result::Result::Ok(
                    Self::Configuration(datom_codec::Carrying::body(v)?),
                )
            }
            "Sources" => {
                std::result::Result::Ok(Self::Sources(datom_codec::Carrying::body(v)?))
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for MetaObservation {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Configuration(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Configuration")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Sources(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Sources").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigurationRefusal {
    InvalidOrdinarySocketPath,
    InvalidMetaSocketPath,
    InvalidSourceManifestPath,
    InvalidRelativePath(RelativePath),
    UnreadableSourceManifest,
}
impl datom_codec::Datomic for ConfigurationRefusal {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "InvalidOrdinarySocketPath" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::InvalidOrdinarySocketPath)
            }
            "InvalidMetaSocketPath" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::InvalidMetaSocketPath)
            }
            "InvalidSourceManifestPath" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::InvalidSourceManifestPath)
            }
            "InvalidRelativePath" => {
                std::result::Result::Ok(
                    Self::InvalidRelativePath(datom_codec::Carrying::body(v)?),
                )
            }
            "UnreadableSourceManifest" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::UnreadableSourceManifest)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for ConfigurationRefusal {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::InvalidOrdinarySocketPath => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("InvalidOrdinarySocketPath")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::InvalidMetaSocketPath => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("InvalidMetaSocketPath")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::InvalidSourceManifestPath => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("InvalidSourceManifestPath")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::InvalidRelativePath(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("InvalidRelativePath")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::UnreadableSourceManifest => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("UnreadableSourceManifest")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Configure(Configuration),
    Observe(MetaObservationSelection),
    Subscribe(MetaSubscriptionRequest),
    Unsubscribe(MetaSubscriptionRequest),
}
impl datom_codec::Datomic for Request {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Configure" => {
                std::result::Result::Ok(Self::Configure(datom_codec::Carrying::body(v)?))
            }
            "Observe" => {
                std::result::Result::Ok(Self::Observe(datom_codec::Carrying::body(v)?))
            }
            "Subscribe" => {
                std::result::Result::Ok(Self::Subscribe(datom_codec::Carrying::body(v)?))
            }
            "Unsubscribe" => {
                std::result::Result::Ok(
                    Self::Unsubscribe(datom_codec::Carrying::body(v)?),
                )
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Request {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Configure(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Configure")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Observe(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Observe").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Subscribe(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Subscribe")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Unsubscribe(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Unsubscribe")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Response {
    Configured(Configuration),
    Observed(MetaObservation),
    ConfigurationRejected(ConfigurationRefusal),
    ConfigurationChanged(Configuration),
    SourcesChanged(SourceIndex),
}
impl datom_codec::Datomic for Response {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Configured" => {
                std::result::Result::Ok(
                    Self::Configured(datom_codec::Carrying::body(v)?),
                )
            }
            "Observed" => {
                std::result::Result::Ok(Self::Observed(datom_codec::Carrying::body(v)?))
            }
            "ConfigurationRejected" => {
                std::result::Result::Ok(
                    Self::ConfigurationRejected(datom_codec::Carrying::body(v)?),
                )
            }
            "ConfigurationChanged" => {
                std::result::Result::Ok(
                    Self::ConfigurationChanged(datom_codec::Carrying::body(v)?),
                )
            }
            "SourcesChanged" => {
                std::result::Result::Ok(
                    Self::SourcesChanged(datom_codec::Carrying::body(v)?),
                )
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Response {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Configured(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Configured")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Observed(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Observed")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::ConfigurationRejected(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("ConfigurationRejected")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::ConfigurationChanged(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("ConfigurationChanged")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::SourcesChanged(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("SourcesChanged")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
pub trait WireConversion: Sized {
    type Wire;
    fn into_wire(self) -> Self::Wire;
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault>;
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WireFault {
    Text,
}
pub type OrdinarySocketPathWire = std::string::String;
pub type MetaSocketPathWire = std::string::String;
pub type SourceManifestPathWire = std::string::String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ConfigurationWire(
    pub OrdinarySocketPathWire,
    pub MetaSocketPathWire,
    pub SourceManifestPathWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MetaSubscriptionRequestWire(pub MetaObservationSelectionWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum MetaObservationSelectionWire {
    Configuration,
    Sources,
}
pub type SourceNameWire = std::string::String;
pub type RelativePathWire = std::string::String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SourceWire(pub SourceNameWire, pub RelativePathWire);
pub type SourcesWire = std::vec::Vec<SourceWire>;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SourceIndexWire(pub SourcesWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum MetaObservationWire {
    Configuration(ConfigurationWire),
    Sources(SourceIndexWire),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ConfigurationRefusalWire {
    InvalidOrdinarySocketPath,
    InvalidMetaSocketPath,
    InvalidSourceManifestPath,
    InvalidRelativePath(RelativePathWire),
    UnreadableSourceManifest,
}
impl WireConversion for Configuration {
    type Wire = ConfigurationWire;
    fn into_wire(self) -> Self::Wire {
        let Configuration(p0, p1, p2) = self;
        ConfigurationWire(p0.to_string(), p1.to_string(), p2.to_string())
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let ConfigurationWire(p0, p1, p2) = wire;
        Ok(
            Configuration(
                protos::Text::try_from(p0).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p2).map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for MetaSubscriptionRequest {
    type Wire = MetaSubscriptionRequestWire;
    fn into_wire(self) -> Self::Wire {
        let MetaSubscriptionRequest(p0) = self;
        MetaSubscriptionRequestWire(
            <MetaObservationSelection as WireConversion>::into_wire(p0),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let MetaSubscriptionRequestWire(p0) = wire;
        Ok(
            MetaSubscriptionRequest(
                <MetaObservationSelection as WireConversion>::try_from_wire(p0)?,
            ),
        )
    }
}
impl WireConversion for MetaObservationSelection {
    type Wire = MetaObservationSelectionWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            MetaObservationSelection::Configuration => {
                MetaObservationSelectionWire::Configuration
            }
            MetaObservationSelection::Sources => MetaObservationSelectionWire::Sources,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            MetaObservationSelectionWire::Configuration => {
                Ok(MetaObservationSelection::Configuration)
            }
            MetaObservationSelectionWire::Sources => {
                Ok(MetaObservationSelection::Sources)
            }
        }
    }
}
impl WireConversion for Source {
    type Wire = SourceWire;
    fn into_wire(self) -> Self::Wire {
        let Source(p0, p1) = self;
        SourceWire(p0.to_string(), p1.to_string())
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let SourceWire(p0, p1) = wire;
        Ok(
            Source(
                protos::Text::try_from(p0).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for SourceIndex {
    type Wire = SourceIndexWire;
    fn into_wire(self) -> Self::Wire {
        let SourceIndex(p0) = self;
        SourceIndexWire(
            p0
                .into_iter()
                .map(|value| <Source as WireConversion>::into_wire(value))
                .collect(),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let SourceIndexWire(p0) = wire;
        Ok(
            SourceIndex(
                p0
                    .into_iter()
                    .map(|value| <Source as WireConversion>::try_from_wire(value))
                    .collect::<std::result::Result<std::vec::Vec<_>, WireFault>>()?,
            ),
        )
    }
}
impl WireConversion for MetaObservation {
    type Wire = MetaObservationWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            MetaObservation::Configuration(value) => {
                MetaObservationWire::Configuration(
                    <Configuration as WireConversion>::into_wire(value),
                )
            }
            MetaObservation::Sources(value) => {
                MetaObservationWire::Sources(
                    <SourceIndex as WireConversion>::into_wire(value),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            MetaObservationWire::Configuration(value) => {
                Ok(
                    MetaObservation::Configuration(
                        <Configuration as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            MetaObservationWire::Sources(value) => {
                Ok(
                    MetaObservation::Sources(
                        <SourceIndex as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
        }
    }
}
impl WireConversion for ConfigurationRefusal {
    type Wire = ConfigurationRefusalWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            ConfigurationRefusal::InvalidOrdinarySocketPath => {
                ConfigurationRefusalWire::InvalidOrdinarySocketPath
            }
            ConfigurationRefusal::InvalidMetaSocketPath => {
                ConfigurationRefusalWire::InvalidMetaSocketPath
            }
            ConfigurationRefusal::InvalidSourceManifestPath => {
                ConfigurationRefusalWire::InvalidSourceManifestPath
            }
            ConfigurationRefusal::InvalidRelativePath(value) => {
                ConfigurationRefusalWire::InvalidRelativePath(value.to_string())
            }
            ConfigurationRefusal::UnreadableSourceManifest => {
                ConfigurationRefusalWire::UnreadableSourceManifest
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            ConfigurationRefusalWire::InvalidOrdinarySocketPath => {
                Ok(ConfigurationRefusal::InvalidOrdinarySocketPath)
            }
            ConfigurationRefusalWire::InvalidMetaSocketPath => {
                Ok(ConfigurationRefusal::InvalidMetaSocketPath)
            }
            ConfigurationRefusalWire::InvalidSourceManifestPath => {
                Ok(ConfigurationRefusal::InvalidSourceManifestPath)
            }
            ConfigurationRefusalWire::InvalidRelativePath(value) => {
                Ok(
                    ConfigurationRefusal::InvalidRelativePath(
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)?,
                    ),
                )
            }
            ConfigurationRefusalWire::UnreadableSourceManifest => {
                Ok(ConfigurationRefusal::UnreadableSourceManifest)
            }
        }
    }
}
impl WireConversion for Request {
    type Wire = RequestWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            Request::Configure(value) => {
                RequestWire::Configure(
                    <Configuration as WireConversion>::into_wire(value),
                )
            }
            Request::Observe(value) => {
                RequestWire::Observe(
                    <MetaObservationSelection as WireConversion>::into_wire(value),
                )
            }
            Request::Subscribe(value) => {
                RequestWire::Subscribe(
                    <MetaSubscriptionRequest as WireConversion>::into_wire(value),
                )
            }
            Request::Unsubscribe(value) => {
                RequestWire::Unsubscribe(
                    <MetaSubscriptionRequest as WireConversion>::into_wire(value),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            RequestWire::Configure(value) => {
                Ok(
                    Request::Configure(
                        <Configuration as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            RequestWire::Observe(value) => {
                Ok(
                    Request::Observe(
                        <MetaObservationSelection as WireConversion>::try_from_wire(
                            value,
                        )?,
                    ),
                )
            }
            RequestWire::Subscribe(value) => {
                Ok(
                    Request::Subscribe(
                        <MetaSubscriptionRequest as WireConversion>::try_from_wire(
                            value,
                        )?,
                    ),
                )
            }
            RequestWire::Unsubscribe(value) => {
                Ok(
                    Request::Unsubscribe(
                        <MetaSubscriptionRequest as WireConversion>::try_from_wire(
                            value,
                        )?,
                    ),
                )
            }
        }
    }
}
impl WireConversion for Response {
    type Wire = ResponseWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            Response::Configured(value) => {
                ResponseWire::Configured(
                    <Configuration as WireConversion>::into_wire(value),
                )
            }
            Response::Observed(value) => {
                ResponseWire::Observed(
                    <MetaObservation as WireConversion>::into_wire(value),
                )
            }
            Response::ConfigurationRejected(value) => {
                ResponseWire::ConfigurationRejected(
                    <ConfigurationRefusal as WireConversion>::into_wire(value),
                )
            }
            Response::ConfigurationChanged(value) => {
                ResponseWire::ConfigurationChanged(
                    <Configuration as WireConversion>::into_wire(value),
                )
            }
            Response::SourcesChanged(value) => {
                ResponseWire::SourcesChanged(
                    <SourceIndex as WireConversion>::into_wire(value),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            ResponseWire::Configured(value) => {
                Ok(
                    Response::Configured(
                        <Configuration as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::Observed(value) => {
                Ok(
                    Response::Observed(
                        <MetaObservation as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::ConfigurationRejected(value) => {
                Ok(
                    Response::ConfigurationRejected(
                        <ConfigurationRefusal as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::ConfigurationChanged(value) => {
                Ok(
                    Response::ConfigurationChanged(
                        <Configuration as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::SourcesChanged(value) => {
                Ok(
                    Response::SourcesChanged(
                        <SourceIndex as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
        }
    }
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RequestWire {
    Configure(ConfigurationWire),
    Observe(MetaObservationSelectionWire),
    Subscribe(MetaSubscriptionRequestWire),
    Unsubscribe(MetaSubscriptionRequestWire),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ResponseWire {
    Configured(ConfigurationWire),
    Observed(MetaObservationWire),
    ConfigurationRejected(ConfigurationRefusalWire),
    ConfigurationChanged(ConfigurationWire),
    SourcesChanged(SourceIndexWire),
}
