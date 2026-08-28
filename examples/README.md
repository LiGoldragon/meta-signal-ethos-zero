# Positional Datomic examples

Each line in `round-trip.datom` is a concrete positional Datomic form of a value
covered by `tests/contract.rs`. These lines are the text-side round-trip
specification for a future CLI. This wire crate intentionally does not parse or
emit them: its only transport is the validated rkyv Signal frame.
