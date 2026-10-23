# ZXV Engineering Parameters: The Kernel and the Architecture

**English** · ZXV — paraconsistent logic · state machine · settlement


This document details the engineering parameters of the ZXV system: the kernel's four-valued core, the state-compatible upgrade discipline, the hash-chain audit ledger, the gas economics, and the measured quality record.

---

## The kernel core
The kernel's lpres module defines the four states as bit patterns in a single byte. The conjoin and disjoin operations are pure integer comparisons over a rank table — zero allocation, a handful of instructions, real-time friendly.

---

## State-compatible upgrades
The serialized top-level struct never changes; new features live in reserved namespaces of the store map. This Borsh forward compatibility let 110 kits accumulate without ever breaking deployed state.

---

## The audit chain
Every settlement links into a hash chain (the receipts). verify_chain confirms the full chain's integrity — the event-sourcing pattern implemented on-chain, giving an immutable audit trail from contradiction to resolution.

---

## The gas economics
A TRUE settlement receives a 2x protocol gas match (the Venice Gas Station); a BOTH forfeits the match (the drain shield). Contradiction is costly, so it cannot be spammed. Reads are entirely gas-free.

---

## The quality record
The QC harness cross-checks against the kernel C source (55/55); the on-chain audit covers all layers (24/24); the durability stress test runs 70/70; the registration audit resolves 136/136; the gas-wall test holds 125/125 reads and 10/10 write cycles.

---

## Conclusion
ZXV is a working proof that paraconsistent logic is engineering, not philosophy: deployed, audited, and verifiable on NEAR.

The system is open.

**Only TRUE settles. TRUE+FALSE=BOTH — contradictions are recorded, never coerced, never a panic.**

