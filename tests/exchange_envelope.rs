//! The meta contract inside signal's exchange envelope, exercised as a wire.
//!
//! Every value is framed, read back off a byte stream and attributed to its
//! exchange. The digest oracle is computed outside the crate, by the published
//! FNV-1a algorithm over signal-ethos-zero's `ethos/library.ethos` then this
//! crate's `ethos/signal.ethos` in Python, not through the path under test.

use std::io::Cursor;

use meta_signal_ethos_zero::{
    CONTRACT, ETHOS, EthosNexusConfiguration, FileLocation, ObservationSelection, Observed_Data,
    Query, Response, SourceIndex,
};
use signal::{
    Answer, ByteViewable, ContractDigest, Contracted, Delivery, Dispatch, Ending, ExchangeLedger,
    ExchangeMinting, Exchanged, FrameCapacity, FrameReading, FrameWriting, Greeted, Handshake,
    HandshakeReceipt, HandshakeRejection, Opening, Restorable, Signal, Signalizable,
};

/// FNV-1a over exactly the bytes of signal-ethos-zero's `ethos/library.ethos`
/// followed by those of `ethos/signal.ethos`, as a signed 64-bit integer.
const META_DIGEST: ContractDigest = -1_340_365_494_544_471_030;

/// Put a value on a byte stream the way a socket carries it, and read it back.
trait CrossesTheWire: Sized {
    fn across(&self) -> Self;
}

impl<T> CrossesTheWire for T
where
    T: Signalizable,
    Signal<T>: Restorable<T>,
{
    fn across(&self) -> Self {
        let capacity = FrameCapacity::default();
        let mut wire = Vec::new();
        wire.write_frame(&self.signalize().expect("signalize"), capacity)
            .expect("write the frame");
        let body = Cursor::new(wire)
            .read_frame(capacity)
            .expect("read the frame");
        Signal::<T>::from(body.bytes().to_vec())
            .restore()
            .expect("restore the value")
    }
}

trait Configures {
    fn temporary() -> Self;
}

impl Configures for EthosNexusConfiguration {
    fn temporary() -> Self {
        Self {
            ordinary_socket_path: "/tmp/ethos.sock".to_owned(),
            meta_socket_path: "/tmp/ethos-meta.sock".to_owned(),
            source_manifest_path: "/tmp/ethos-sources.datom".to_owned(),
        }
    }
}

/// A greeted ledger of the querying side, where exchange identifiers come
/// from.
trait Opens {
    fn greeted() -> Self;
}

impl Opens for ExchangeLedger {
    fn greeted() -> Self {
        let mut ledger = Self::default();
        ledger.greet().expect("the first greeting");
        ledger
    }
}

#[test]
fn the_meta_contract_is_identified_by_the_digest_of_its_own_source() {
    assert_eq!(<Query as Contracted>::contract_digest(), META_DIGEST);
    assert_eq!(
        <Query as Contracted>::greeting(),
        Handshake {
            contract_digest: META_DIGEST
        }
    );
    assert_eq!(<Query as Contracted>::CONTRACT_SOURCE, CONTRACT);
    assert!(CONTRACT.ends_with(ETHOS));
}

#[test]
fn an_ordinary_peer_greeting_the_meta_socket_is_refused() {
    assert_eq!(
        <Query as Contracted>::receipt(&<signal_ethos_zero::Query as Contracted>::greeting()),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(META_DIGEST))
    );
    assert_eq!(
        <Query as Contracted>::receipt(&<Query as Contracted>::greeting()),
        HandshakeReceipt::Greeted(META_DIGEST)
    );
}

#[test]
fn a_configure_is_one_exchange_answered_once_and_then_ended() {
    let mut ledger = ExchangeLedger::greeted();
    let exchange = ledger.open().expect("open the exchange");

    let opening = Dispatch::Open(Opening {
        exchange,
        query: Query::Configure(EthosNexusConfiguration::temporary()),
    });
    assert_eq!(opening.across(), opening);

    let answer: Delivery<Response> = Delivery::Answer(Answer {
        exchange,
        response: Response::Configured(EthosNexusConfiguration::temporary()),
    });
    assert_eq!(answer.across(), answer);

    let ending: Delivery<Response> = Delivery::End(Ending::completed(exchange));
    let Delivery::End(received) = ending.across() else {
        panic!("a one-answer exchange ends after its answer");
    };
    assert_eq!(received.exchange(), exchange);
}

#[test]
fn a_source_subscription_and_a_configure_are_told_apart_by_exchange_alone() {
    let capacity = FrameCapacity::default();
    let mut ledger = ExchangeLedger::greeted();
    let watching = ledger.open().expect("open the subscription");
    let configuring = ledger.open().expect("open the Configure exchange");
    let subscribe: Dispatch<Query> = Dispatch::Open(Opening {
        exchange: watching,
        query: Query::Subscribe(ObservationSelection::Sources),
    });
    assert_eq!(subscribe.across(), subscribe);
    let index: SourceIndex = vec![FileLocation {
        source_name: "workspace".to_owned(),
        relative_path: "ethos/signal.ethos".to_owned(),
    }];

    let written: Vec<Delivery<Response>> = vec![
        Delivery::Greeted(HandshakeReceipt::Greeted(META_DIGEST)),
        Delivery::Answer(Answer {
            exchange: watching,
            response: Response::Observed(Observed_Data::Sources(Vec::new())),
        }),
        Delivery::Answer(Answer {
            exchange: configuring,
            response: Response::Configured(EthosNexusConfiguration::temporary()),
        }),
        Delivery::End(Ending::completed(configuring)),
        Delivery::Answer(Answer {
            exchange: watching,
            response: Response::SourcesChanged(index.clone()),
        }),
    ];
    let mut wire = Vec::new();
    for delivery in &written {
        wire.write_frame(&delivery.signalize().expect("signalize"), capacity)
            .expect("write one frame");
    }
    let mut stream = Cursor::new(wire);
    let read = written
        .iter()
        .map(|_| {
            let body = stream.read_frame(capacity).expect("read one frame");
            Signal::<Delivery<Response>>::from(body.bytes().to_vec())
                .restore()
                .expect("restore one delivery")
        })
        .collect::<Vec<_>>();
    assert_eq!(read, written);

    let watched = read
        .iter()
        .filter_map(|delivery| match delivery {
            Delivery::Answer(answer) if answer.exchange() == watching => Some(&answer.response),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        watched,
        vec![
            &Response::Observed(Observed_Data::Sources(Vec::new())),
            &Response::SourcesChanged(index),
        ]
    );
    let abandon: Dispatch<Query> = Dispatch::Abandon(watching);
    assert_eq!(abandon.across(), Dispatch::Abandon(watching));
}
