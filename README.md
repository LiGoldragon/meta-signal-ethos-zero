# meta-signal-ethos-zero

Generation-zero owner MetaSignal vocabulary for Ethos, version 2.0.0: the
meta-socket wire of the Ethos Nexus that follows ethos-zero, which nothing
serves yet.

`ethos/signal.ethos` is the authored source, written in ethos-zero's own
vertical print; it imports `RelativePath` and `FileLocation` from
signal-ethos-zero's Library. `src/generated/signal.rs` is its ethos-zero
16.0.0 projection, held byte-identical, and the source held in that print, by
`build.rs`. Regenerate it with
`ethos-zero 'Generate.{ /abs/ethos/signal.ethos /abs/src/generated }'`.

The contract rides signal 8.0.0's exchange layer: a meta connection is greeted
once with the digest of signal-ethos-zero's Library followed by
`ethos/signal.ethos` (`CONTRACT`, `signal::Contracted for Query`), which
differs from the ordinary contract's, so an ordinary peer is refused at the
greeting. Each query travels as `Dispatch::Open` on an exchange the peer
names, and each response comes back as `Delivery::Answer` on that exchange
until `Delivery::End`. Datom text is behind the `datom` feature, for the meta
CLI only.
