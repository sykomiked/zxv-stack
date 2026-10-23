# Paraconsistent Logic: A Complete Introduction

**English** · ZXV — paraconsistent logic · state machine · settlement


Every distributed system eventually faces the same wall: two sources of information disagree about the truth, and the system must decide what to do. Classical two-valued logic triggers the principle of explosion when contradictory evidence arrives — from a contradiction, anything follows, so nothing is reliable. Paraconsistent logic offers a different answer: treat the contradiction itself as information.

---

## 1. The problem classical logic cannot solve
The law of excluded middle (P or not-P) assumes every proposition is either true or false. But in a distributed environment, network partitions and replication lag mean both P and not-P can arrive at once. Classical systems respond with panic, last-writer-wins, deadlock, or majority coercion — each losing information.

---

## 2. The four values
A four-valued state machine holds TRUE, FALSE, NEITHER, and BOTH as first-class states. BOTH means a contradiction is recorded and held — never coerced, never erased. This is Belnap's four-valued logic, adapted as a systems primitive.

---

## 3. The chain-meet algebra
Order the states on a total chain: FALSE < NEITHER < BOTH < TRUE. The meet (conjoin) returns the smaller rank; the join (disjoin) returns the larger. Because TRUE is the top, no combination of non-TRUE values can ever yield TRUE — the invariant is structural, not checked.

---

## 4. The invariant
The settlement rule falls out of the rank structure: only TRUE settles. A contradiction (BOTH) is held and journaled; agreement (TRUE) alone authorizes action. The algebra is idempotent, commutative, associative, and absorbing — a bounded lattice verified over all 16 and 64 combinations.

**Only TRUE settles. TRUE+FALSE=BOTH — contradictions are recorded, never coerced, never a panic.**

---

## 5. Conclusion
Paraconsistent logic solves what classical systems refuse: what to do when the truth is contested. The answer is to hold the contradiction as evidence, journal it, and settle only on genuine agreement.

The system is open.
