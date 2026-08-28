# meta-signal-ethos-zero

This repository is the generation-zero owner MetaSignal contract for Ethos. It owns
the authored `ethos/signal.ethos`, committed hand-written Rust projection, and a
validated length-prefixed rkyv frame codec. Every frame carries and validates the
MetaEthosZero contract identity and wire revision before its body is accepted. It owns no runtime, CLI, parser, string
codec, generator, storage, actors, or streams.

`examples/round-trip.datom` gives concrete positional Datomic values as contract
examples only. A later meta CLI performs any text-to-Signal conversion; this crate
speaks rkyv Signal only.
