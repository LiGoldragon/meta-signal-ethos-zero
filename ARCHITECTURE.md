# meta-signal-ethos-zero

This repository is the generation-zero owner MetaSignal contract for Ethos,
spoken on the meta socket. It owns the authored `ethos/signal.ethos` and the
committed Rust that ethos-zero generates from it, and nothing else: no runtime, CLI, parser, generator,
storage or framing.

Framing, the greeting and the exchange envelope are signal's. The contract's
wire identity is the digest of signal-ethos-zero's Library, which the
source imports, followed by the source, so a peer built from any
other source is refused at the greeting rather than negotiated with; there is
no allocated contract number and no revision. Every generated type archives
with rkyv; its datom derives sit behind the `datom` feature, which a CLI
enables and a Nexus does not.

`examples/round-trip.datom` holds the datom text of concrete values; the
`datom` feature's test reads every line and reads it back equal. Production
Rust carries no free function and no inherent method, held by the
`no-free-functions` and `no-inherent-methods` flake checks.

The names the meta and ordinary contracts share (`SourceName`,
`RelativePath`, `FileLocation`) live in signal-ethos-zero's Library; this
crate depends on signal-ethos-zero, a wire repository, and on no Nexus.
