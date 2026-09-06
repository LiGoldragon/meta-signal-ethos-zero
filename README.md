# meta-signal-ethos-zero

Generation-zero owner MetaSignal vocabulary for Ethos, version 0.5.0.

The public API re-exports the generated request and response vocabulary. Typed Datom
is the text boundary; socket bytes are length-prefixed structural rkyv values in a
`BoundExchangeFrame`. The privileged Ethos-zero contract owns globally allocated
`ContractId` 8 at wire revision 4. It validates that binding before archive decoding
and validates the request or response route before recovering the generated wire value.
