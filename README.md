# meta-signal-ethos-zero

Generation-zero owner MetaSignal vocabulary for Ethos, version 0.2.0.

The public API is the re-exported `src/generated/signal.rs` vocabulary, `Frame`, and
`SignalFrameCodec`. The frame is a four-byte little-endian payload length followed by
a validated rkyv `Frame`; its envelope carries MetaEthosZero contract identity 2,
wire revision 2, and protocol version 0.2.0.
