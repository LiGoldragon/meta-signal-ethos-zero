# meta-signal-ethos-zero

Generation-zero owner MetaSignal vocabulary for Ethos, version 0.1.0.

The public API is the re-exported `src/generated/signal.rs` vocabulary, `Frame`, and
`SignalFrameCodec`. The frame is a four-byte little-endian payload length followed by
a validated rkyv `Frame`; its protocol version is exactly 0.1.0.
