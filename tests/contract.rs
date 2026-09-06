use core::num::{NonZeroU16, NonZeroU32};
use datom_codec::{Actualizable, IncorporationBudget, Potential, Textualizable};
use meta_signal_ethos_zero::{
    Configuration, ExchangeDecodeFault, MetaEthosZeroWire, Request, RequestWire, ResponseWire,
    WireConversion, decode_request, encode_request,
};
use signal_frame::{
    BoundExchangeFrame, ContractBinding, ContractId, ExchangeFrameBody, ExchangeIdentifier,
    ExchangeLane, LaneSequence, RootCode, SessionEpoch, VariantCode, WireContract, WireRevision,
    WireRoute,
};
fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::new(1),
    )
}
fn request() -> Request {
    Request::Configure(Configuration(
        protos::Text::try_from("/run/ordinary.sock").unwrap(),
        protos::Text::try_from("/run/meta.sock").unwrap(),
        protos::Text::try_from("/etc/ethos/sources.datom").unwrap(),
    ))
}
#[test]
fn typed_datom_request_round_trips() {
    let expected = request();
    let text = <Request as Textualizable<datom_codec::Datom>>::textualize(&expected);
    let found = Potential::<Request>::from(text.as_str())
        .actualize(IncorporationBudget::try_from(128).unwrap())
        .unwrap();
    assert_eq!(found, expected);
}
#[test]
fn bound_structural_request_round_trips() {
    let expected = request();
    let (found_exchange, found) =
        decode_request(&encode_request(exchange(), expected.clone()).unwrap()).unwrap();
    assert_eq!(found_exchange, exchange());
    assert_eq!(found, expected);
}
struct OrchestrateMetaContract;
impl WireContract for OrchestrateMetaContract {
    const BINDING: ContractBinding = ContractBinding::new(
        ContractId::new(NonZeroU32::new(2).unwrap()),
        WireRevision::new(NonZeroU16::new(3).unwrap()),
    );
}
struct WrongRevision;
impl WireContract for WrongRevision {
    const BINDING: ContractBinding = ContractBinding::new(
        ContractId::new(NonZeroU32::new(8).unwrap()),
        WireRevision::new(NonZeroU16::new(3).unwrap()),
    );
}
fn forged<Contract: WireContract>(route: WireRoute) -> Vec<u8> {
    BoundExchangeFrame::<Contract, RequestWire, ResponseWire>::new(
        route,
        ExchangeFrameBody::Request {
            exchange: exchange(),
            request: signal_frame::Request::from_payload(request().into_wire()),
        },
    )
    .encode_length_prefixed()
    .unwrap()
}
#[test]
fn orchestrate_meta_contract_is_rejected_before_archive_decode() {
    let correct = WireRoute::new(RootCode::new(0), VariantCode::new(0));
    let mut bytes = forged::<OrchestrateMetaContract>(correct);
    bytes[..4].copy_from_slice(&(8_u32).to_be_bytes());
    bytes.truncate(12);
    assert!(matches!(
        decode_request(&bytes),
        Err(ExchangeDecodeFault::Frame(
            signal_frame::FrameError::ContractMismatch { .. }
        ))
    ));
}
#[test]
fn wrong_revision_route_and_archive_fail_closed() {
    let correct = WireRoute::new(RootCode::new(0), VariantCode::new(0));
    assert!(matches!(
        decode_request(&forged::<WrongRevision>(correct)),
        Err(ExchangeDecodeFault::Frame(
            signal_frame::FrameError::UnsupportedWireRevision { .. }
        ))
    ));
    assert!(matches!(
        decode_request(&forged::<MetaEthosZeroWire>(WireRoute::new(
            RootCode::new(1),
            VariantCode::new(0)
        ))),
        Err(ExchangeDecodeFault::RouteMismatch { .. })
    ));
    assert!(matches!(
        decode_request(&[0, 0, 0, 1]),
        Err(ExchangeDecodeFault::Frame(_))
    ));
}
