# Datom examples

Each line in `round-trip.datom` is the datom text of a query or a response of
this contract, as a CLI would read it. `tests/generated_contract.rs`, under the
`datom` feature, reads every line as one of the two roots and reads its
printed form back equal. The contract crate speaks rkyv on the wire; datom is
the CLI's text boundary.
