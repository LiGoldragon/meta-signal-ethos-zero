# Upgrades

## 0.5.0 — distinct global structural binding

This breaking wire change moves the privileged Ethos-zero contract from the
colliding historical Orchestrate identity to ContractId 8 while retaining its
structural wire revision 4. Contract IDs are global identities in the shared
signal-frame header; a revision is contract-local and cannot distinguish a
different component. Upgrade every Ethos-zero privileged peer together. Frames
carrying Orchestrate's 2/3 binding are rejected as foreign before archive decoding.

## 0.3.0 — generated lifecycle contract

This is a breaking wire change. Deploy every MetaEthosZero peer with
`meta-signal-ethos-zero` 0.3.0 in one coordinated rollout. The contract is now
generated from `ethos/signal.ethos`, carries interface 0.3.0 and wire revision
3, and adds Subscribe/Unsubscribe lifecycle requests, typed
`InvalidRelativePath` refusals, and configuration/source stream events.

Do not mix 0.2.0 and 0.3.0 peers on one socket. Drain or stop the old peer,
deploy the new peer and its generated contract revision, then reopen the socket.

## 0.2.0 — framed contract identity

This is a breaking wire change. Deploy every MetaEthosZero peer with
`meta-signal-ethos-zero` 0.2.0 in one coordinated rollout. New frames carry
contract identity 2 and wire revision 2 inside the rkyv envelope; 0.2.0 rejects
frames with the former revision or another contract identity before accepting their
bodies.

Do not mix 0.1.0 and 0.2.0 peers on one socket. Drain or stop the old peer, deploy
the new peer and its pinned contract revision, then reopen the socket.
