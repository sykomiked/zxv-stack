# ZXV — The Paraconsistent Settlement Stack

**The complete 48-kit settlement layer on NEAR mainnet. One micro
contract. Zero bugs. Open to build on.**

```
keywords: near · paraconsistent · settlement · rails · cards · oracle ·
ai · web4 · decentralized · blockchain · sdk · rust · typescript ·
crypto · nearcore · dapp · smart-contract · defi · sovereign · zxv
```

---

## What this is

The ZXV paraconsistent settlement layer: a four-valued logic kernel in
which **only `TRUE` settles**. Contradictions (`TRUE + FALSE = BOTH`) are
recorded, never coerced, never a panic. The full system — 84 kits in one
micro contract — is **live on NEAR mainnet**, registered (66 capabilities,
all resolving), and exposed through a **serverless on-chain API** with a
standardized bridge interface.

```
┌──────────────────────────────────────────────────────────┐
│  YOUR APP / AGENT / PROTOCOL                              │
│        │                                                  │
│        ▼                                                  │
│  zxv-sdk (Rust) · @zxv/developer-kit (TypeScript)         │
│        │                                                  │
│        ▼                                                  │
│  zxv-core (f18889ea…) — the 84-kit micro contract         │
│   28 BASE · 5 LAYERS · 7 SYSTEMS · 4 META · 4 API · 4 UX
│    + REWARDS (11) · PLAYER (10) · SYSTEM (10) · GAS (1)         │
│        │                                                  │
│        ▼                                                  │
│  NEAR mainnet — 103 registered capabilities                │
└──────────────────────────────────────────────────────────┘
```

## The 48 kits

| Family | Kits |
|---|---|
| **28 BASE** (primitives) | gate, treasury, rails, receipts, vino, tribute, surplus, chronicle, allotment, broker, rmag, scp144, aipi, chiglet, count_house, vswap, vauction, vbond, identity, reputation, provenance, grants, community_chest, royalty, telemetry, market, intent, mesh_token |
| **5 LAYERS** (compositions) | settlement, market, trust, distribution, agent |
| **7 SYSTEMS** (services) | escrow, agent_economy, governance, marketplace, compliance, identity_verification, settlement_rail |
| **4 META** (orchestration) | orchestrator, router, self, adaptation |
| **4 API** (serverless interface) | gateway, verify, bridge, index |

## The repository layout

```
contracts/zxv-core/   the 48-kit micro contract (near-sdk, wasm)
crates/zxv-sdk/       the Rust developer crate (no_std, crates.io-ready)
typescript/           @zxv/developer-kit (TypeScript SDK)
tools/qc_audit.py     the high-assurance QC harness (55 checks vs the kernel C source)
docs/                 the complete system record (incl. the fireworks display)
```

## Quick start

### Rust

```toml
[dependencies]
zxv-sdk = "0.1"
```

```rust
use zxv_sdk::{Gate, LogicState, settle_or_hold};

let verdict = Gate::conjoin(LogicState::True, LogicState::False);
assert!(!verdict.is_settleable());   // contradiction held, no panic
```

### TypeScript

```bash
npm install @zxv/developer-kit
```

```ts
import { Zxv, LogicState } from "@zxv/developer-kit";
const zxv = new Zxv();
const caps = await zxv.discover();               // 66 capabilities
await zxv.verifyCard("88090018017311574164");    // "dragon"
await zxv.isSettleable(LogicState.BOTH);         // false
```

### Verify the live system (gas-free)

```bash
near view f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf kits '{}'
near view f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf conjoin '{"inputs":[1,2]}'
near view 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a vm_capabilities '{}'
```

## The settlement invariant

| Evidence | Verdict | Settlement |
|---|---|---|
| all sources agree | `TRUE` | ✅ allowed |
| sources diverge | `BOTH` | ❌ held — recorded, never coerced |
| no evidence yet | `NEITHER` | ❌ waits |
| explicit refutation | `FALSE` | ❌ refused |

## The rail card networks

20-digit Luhn-valid rail numbers **outside ISO/IEC 7812**:

| Network | Prefix | Inner check |
|---|---|---|
| Dragon | `8809` | MOD 11,10 |
| Phoenix | `8829` | Damm |
| Thunderbird | `8849` | Verhoeff |

## Quality control

The QC harness cross-validates the contract logic against the **actual
C kernel source** (compiled as a reference oracle):

```
55/55 harness · 50/50 durability · 20/20 on-chain gauntlet ·
29/29 system test · 66/66 registration = 220/220 PASS, zero bugs
```

## Regulatory posture

Work-based, not passive (SEC Howey prong-4 / CFTC commodity standards):
bounties per verified receipt, slashable worker bonds, escrow
self-containment ("spend only what you hold"). No pooling, no lending,
no passive yield.

## Documentation

- **The complete system record** (incl. the fireworks display):
  `docs/FIREWORKS_COMPLETE_SYSTEM.md`
- **The NEP proposal:** https://github.com/sykomiked/ZXV-NEP-paraconsistent/blob/main/neps/nep-0000-paraconsistent-settlement-layer.md
- **The forensic ledger:** https://github.com/sykomiked/ZXV-NEP-paraconsistent/blob/main/docs/ledger/FORENSIC_LEDGER.md

## Related repositories

- **ZXV-NEP-paraconsistent** (the NEP repo + the source):
  https://github.com/sykomiked/ZXV-NEP-paraconsistent
- **zxv-developer-kit** (the SDK repo):
  https://github.com/sykomiked/zxv-developer-kit

## License

Apache-2.0 — the same license as the ZXV kernel.

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC*