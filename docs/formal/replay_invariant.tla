-------------------------------- MODULE ReplayInvariant --------------------------------
(* Minimal abstract model: Execute assigns capsule id; Replay copies it (left inverse). *)
EXTENDS Naturals, TLC

CONSTANT CapsuleIds

ASSUME CapsuleIds \subseteq Nat

VARIABLES cId, rId

vars == <<cId, rId>>

Init == cId = 0 /\ rId = 0

Execute ==
    /\ cId = 0
    /\ \E x \in CapsuleIds : cId' = x
    /\ rId' = rId

Replay ==
    /\ cId # 0
    /\ rId' = cId
    /\ cId' = cId

Next == Execute \/ Replay

Spec == Init /\ [][Next]_vars

(* After any replay, replayed id matches executed capsule id. *)
ReplayInvariant == (rId = 0) \/ (rId = cId)

TypeOK == cId \in Nat /\ rId \in Nat

DeterminismProfileStub == TRUE

EnvelopeStub == TRUE

=============================================================================
