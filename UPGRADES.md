# Upgrades

## 0.2.0 — framed contract identity

This is a breaking wire change. Deploy every MetaEthosZero peer with
`meta-signal-ethos-zero` 0.2.0 in one coordinated rollout. New frames carry
contract identity 2 and wire revision 2 inside the rkyv envelope; 0.2.0 rejects
frames with the former revision or another contract identity before accepting their
bodies.

Do not mix 0.1.0 and 0.2.0 peers on one socket. Drain or stop the old peer, deploy
the new peer and its pinned contract revision, then reopen the socket.
