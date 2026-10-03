//! The generated meta vocabulary as values: portable rkyv frames always, and
//! datom text under the `datom` feature, which the CLI enables.

use meta_signal_ethos_zero::{
    ConfigurationRejected_Data, EthosNexusConfiguration, ObservationSelection, Query, Response,
};
use signal::{ByteViewable, Restorable, Signal, Signalizable};

#[test]
fn query_and_response_round_trip_as_portable_frames() {
    let configure = Query::Configure(EthosNexusConfiguration {
        ordinary_socket_path: "/run/ethos-zero.sock".to_owned(),
        meta_socket_path: "/run/ethos-zero-meta.sock".to_owned(),
        source_manifest_path: "/etc/ethos-zero/sources.datom".to_owned(),
    });
    for query in [
        configure,
        Query::Observe(ObservationSelection::Configuration),
    ] {
        let frame = query.signalize().expect("signalize");
        assert!(!frame.bytes().is_empty());
        let received = Signal::<Query>::from(frame.bytes().to_vec());
        assert_eq!(received.restore().expect("restore query"), query);
    }
    let response = Response::ConfigurationRejected(
        ConfigurationRejected_Data::InvalidRelativePath("../outside".to_owned()),
    );
    let frame = response.signalize().expect("signalize");
    assert_eq!(frame.restore().expect("restore response"), response);
}

#[cfg(feature = "datom")]
#[test]
fn every_example_line_reads_as_a_query_or_a_response_and_reads_back_equal() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    fn budget() -> Budget {
        Budget {
            remaining: 1_024,
            reader: ReaderBudget { remaining: 1_024 },
            depth: 0,
            maximum_depth: 1_024,
        }
    }
    let examples = include_str!("../examples/round-trip.datom");
    let mut queries = 0;
    let mut responses = 0;
    for line in examples.lines().filter(|line| !line.trim().is_empty()) {
        if let Ok(query) = Potential::<Query>::from(line.to_owned()).actualize(&mut budget()) {
            let text = query.clone().datomize(vec![]).protosize().textualize();
            let again = Potential::<Query>::from(text)
                .actualize(&mut budget())
                .expect("a printed query reads back");
            assert_eq!(again, query);
            queries += 1;
        } else {
            let response = Potential::<Response>::from(line.to_owned())
                .actualize(&mut budget())
                .unwrap_or_else(|error| panic!("{line} reads as neither root: {error:?}"));
            let text = response.clone().datomize(vec![]).protosize().textualize();
            let again = Potential::<Response>::from(text)
                .actualize(&mut budget())
                .expect("a printed response reads back");
            assert_eq!(again, response);
            responses += 1;
        }
    }
    assert_eq!((queries, responses), (4, 10));
}

/// The two shapes 2.0.0 changed (UPGRADES.md): their 1.0.0 text, one brace
/// level deeper, no longer reads as either root.
#[cfg(feature = "datom")]
#[test]
fn the_text_of_a_removed_holder_no_longer_reads() {
    use datom_codec::{Actualizing, Budget, Potential};
    use protos::ReaderBudget;

    fn budget() -> Budget {
        Budget {
            remaining: 1_024,
            reader: ReaderBudget { remaining: 1_024 },
            depth: 0,
            maximum_depth: 1_024,
        }
    }
    for line in [
        "Subscribe.{ Sources }",
        "Observed.Sources.{ [ { workspace ethos/signal.ethos } ] }",
        "SourcesChanged.{ [ { workspace ethos/signal.ethos } ] }",
    ] {
        assert!(
            Potential::<Query>::from(line.to_owned())
                .actualize(&mut budget())
                .is_err()
        );
        assert!(
            Potential::<Response>::from(line.to_owned())
                .actualize(&mut budget())
                .is_err()
        );
    }
}
