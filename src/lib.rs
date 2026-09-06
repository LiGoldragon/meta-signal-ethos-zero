//! Privileged Ethos-zero Signal generated from current Ethos.
//!
//! The public roots are typed Datom; socket bytes are structural rkyv values
//! bound to contract seat 2, revision 4.

#[rustfmt::skip]
#[path = "generated/signal.rs"]
mod generated;
pub use generated::*;
pub const META_ETHOS_ZERO_SIGNAL_SOURCE: &str = include_str!("../ethos/signal.ethos");

impl WireConversion for protos::Text {
    type Wire = String;
    fn into_wire(self) -> Self::Wire {
        self.to_string()
    }
    fn try_from_wire(wire: Self::Wire) -> Result<Self, WireFault> {
        protos::Text::try_from(wire).map_err(|_| WireFault::Text)
    }
}

/// The allocated privileged Ethos-zero wire contract: stable seat 2, structural revision 4.
pub enum MetaEthosZeroWire {}
impl signal_frame::WireContract for MetaEthosZeroWire {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        signal_frame::ContractId::new(
            core::num::NonZeroU32::new(2).expect("meta Ethos-zero wire seat is nonzero"),
        ),
        signal_frame::WireRevision::new(
            core::num::NonZeroU16::new(4).expect("structural wire revision is nonzero"),
        ),
    );
}
pub type MetaEthosZeroFrame =
    signal_frame::BoundExchangeFrame<MetaEthosZeroWire, RequestWire, ResponseWire>;

#[derive(Debug)]
pub enum ExchangeDecodeFault {
    Frame(signal_frame::FrameError),
    UnexpectedFrameBody,
    MultiplePayloads {
        count: usize,
    },
    RouteMismatch {
        expected: signal_frame::WireRoute,
        actual: signal_frame::WireRoute,
    },
    Wire(WireFault),
}
impl From<signal_frame::FrameError> for ExchangeDecodeFault {
    fn from(fault: signal_frame::FrameError) -> Self {
        Self::Frame(fault)
    }
}
fn request_route(request: &Request) -> signal_frame::WireRoute {
    let variant = match request {
        Request::Configure(_) => 0,
        Request::Observe(_) => 1,
        Request::Subscribe(_) => 2,
        Request::Unsubscribe(_) => 3,
    };
    signal_frame::WireRoute::new(
        signal_frame::RootCode::new(0),
        signal_frame::VariantCode::new(variant),
    )
}
fn response_route(response: &Response) -> signal_frame::WireRoute {
    let variant = match response {
        Response::Configured(_) => 0,
        Response::Observed(_) => 1,
        Response::ConfigurationRejected(_) => 2,
        Response::ConfigurationChanged(_) => 3,
        Response::SourcesChanged(_) => 4,
    };
    signal_frame::WireRoute::new(
        signal_frame::RootCode::new(1),
        signal_frame::VariantCode::new(variant),
    )
}
pub fn encode_request(
    exchange: signal_frame::ExchangeIdentifier,
    request: Request,
) -> Result<Vec<u8>, signal_frame::FrameError> {
    let route = request_route(&request);
    MetaEthosZeroFrame::new(
        route,
        signal_frame::ExchangeFrameBody::Request {
            exchange,
            request: signal_frame::Request::from_payload(request.into_wire()),
        },
    )
    .encode_length_prefixed()
}
pub fn encode_response(
    exchange: signal_frame::ExchangeIdentifier,
    response: Response,
) -> Result<Vec<u8>, signal_frame::FrameError> {
    let route = response_route(&response);
    MetaEthosZeroFrame::new(
        route,
        signal_frame::ExchangeFrameBody::Reply {
            exchange,
            reply: signal_frame::Reply::committed(signal_frame::NonEmpty::single(
                signal_frame::SubReply::Ok(response.into_wire()),
            )),
        },
    )
    .encode_length_prefixed()
}
pub fn decode_request(
    bytes: &[u8],
) -> Result<(signal_frame::ExchangeIdentifier, Request), ExchangeDecodeFault> {
    let frame = MetaEthosZeroFrame::decode_length_prefixed(bytes)?;
    let actual = frame.short_header().route();
    let signal_frame::ExchangeFrameBody::Request { exchange, request } = frame.into_body() else {
        return Err(ExchangeDecodeFault::UnexpectedFrameBody);
    };
    if request.payloads().len() != 1 {
        return Err(ExchangeDecodeFault::MultiplePayloads {
            count: request.payloads().len(),
        });
    }
    let request = Request::try_from_wire(request.payloads().clone().into_head())
        .map_err(ExchangeDecodeFault::Wire)?;
    let expected = request_route(&request);
    if actual != expected {
        return Err(ExchangeDecodeFault::RouteMismatch { expected, actual });
    }
    Ok((exchange, request))
}
pub fn decode_response(
    bytes: &[u8],
) -> Result<(signal_frame::ExchangeIdentifier, Response), ExchangeDecodeFault> {
    let frame = MetaEthosZeroFrame::decode_length_prefixed(bytes)?;
    let actual = frame.short_header().route();
    let signal_frame::ExchangeFrameBody::Reply { exchange, reply } = frame.into_body() else {
        return Err(ExchangeDecodeFault::UnexpectedFrameBody);
    };
    let signal_frame::Reply::Accepted { per_operation, .. } = reply else {
        return Err(ExchangeDecodeFault::UnexpectedFrameBody);
    };
    if per_operation.len() != 1 {
        return Err(ExchangeDecodeFault::MultiplePayloads {
            count: per_operation.len(),
        });
    }
    let signal_frame::SubReply::Ok(response) = per_operation.into_head() else {
        return Err(ExchangeDecodeFault::UnexpectedFrameBody);
    };
    let response = Response::try_from_wire(response).map_err(ExchangeDecodeFault::Wire)?;
    let expected = response_route(&response);
    if actual != expected {
        return Err(ExchangeDecodeFault::RouteMismatch { expected, actual });
    }
    Ok((exchange, response))
}
