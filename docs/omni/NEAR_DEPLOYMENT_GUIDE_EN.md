# Using the ZXV NEAR Deployment: A Complete Guide

**English** · ZXV — paraconsistent logic · state machine · settlement


ZXV is a live implementation of paraconsistent logic on the NEAR network. This guide explains how to read, verify, and use the deployment — gas-free, from any client, with no wallet required for reading.

---

## The addresses
The canonical address is zedec.near, a live routing contract. It points to the protocol (the 110-kit micro-contract), the registry (136 capabilities), and the gate (the kernel invariant).

```
zedec.near · f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf · 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a · 6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

---

## How to read the system
Any client reads the system gas-free with view calls. The universal interface teaches itself: ux_manifest returns the whole schema, ux_help answers any topic, and the registry's vm_capabilities lists all capabilities.

---

## How to join
A new actor calls mint_onboard to receive a one-time 5000-unit grant, spendable on the platform but not transferable — one per account, preventing wallet spam. Then pool_enter provides access through the liquidity pool.

---

## How to verify
The invariant is verifiable on-chain: conjoin returns the four-valued result; is_settleable returns true only for TRUE; verify_chain confirms the audit hash-chain. Every claim can be checked independently, gas-free.

---

## References
The kernel implementation lives at github.com/sykomiked/zxv-stack; the NEP and ledger at github.com/sykomiked/ZXV-NEP-paraconsistent; the developer kit at github.com/sykomiked/zxv-developer-kit.

The system is open.

**Only TRUE settles. TRUE+FALSE=BOTH — contradictions are recorded, never coerced, never a panic.**

