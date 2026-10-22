# ZXV — The Complete System Record

**The paraconsistent settlement layer on NEAR mainnet — comprehensive documentation, including the fireworks display.**

---

## 1. The system in one paragraph

ZXV is a four-valued paraconsistent settlement layer on NEAR: a logic
kernel in which **only `TRUE` settles**. Contradictions (`TRUE + FALSE =
BOTH`) are recorded, never coerced, never a panic. The full system is 48
kits in one micro contract (`zxv-core`), deployed on mainnet, registered
in the world-registry (66 capabilities, all resolving), and exposed
through a serverless on-chain API with a standardized bridge interface.

## 2. The four-valued foundation

| State | Value | Meaning |
|---|---|---|
| `NEITHER` | `0x00` | no evidence yet |
| `TRUE` | `0x01` | attested, agreed |
| `FALSE` | `0x02` | explicitly refuted |
| `BOTH` | `0x03` | contradiction retained |

The settlement invariant: **only `TRUE` is settleable**; `BOTH` and
`NEITHER` hold; `FALSE` refuses. `BOTH` is absorbing — a majority can
never coerce a recorded contradiction. The knowledge join
`kjoin(a,b) = (a|b) & 3` has a known defect on out-of-range inputs
(`kjoin(0,5) = TRUE`); the system's evidence combination uses the
kernel's `lpres_conjoin` with range-checking (the verdict_guard fix).

## 3. The deployed fabric

### The 13 original subsystems (untouched, all live)

| Capability | Account | Role |
|---|---|---|
| gate, ledger | `6ca890f7…` | the kernel: the gate + the $YHVH ledger |
| registry | `7eaa2c6f…` | the boot sector (discovery) |
| oracle | `6a57f581…` | the original oracle gateway |
| bus | `3c70e9c6…` | the IPC bus |
| shield | `789457e6…` | the execution shield |
| micro_infer | `86f80143…` | deterministic on-chain AI |
| model_registry | `77361b15…` | content-addressed model manifests |
| consumer | `8947dcb7…` | the integration proof |
| paraconsistent_oracle | `ea85f538…` | dispute-tolerant price harmonizer |
| settlement_router | `8bf17701…` | the receipt rail |
| yhv_wrapper | `1a051ac2…` | wYHVH liquidity adapter |
| ai_verdict | `781fc03d…` | settleable on-chain AI |

### The consolidated core (the 48-kit micro contract)

`zxv-core` (`f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`)
hosts the kernel's complete feature surface as 48 kits in one contract —
one storage payment, one runtime, namespaced methods:

**28 BASE kits** (the kernel's primitives): gate, treasury, rails,
receipts, vino, tribute, surplus, chronicle, allotment, broker, rmag,
scp144, aipi, chiglet, count_house, vswap, vauction, vbond, identity,
reputation, provenance, grants, community_chest, royalty, telemetry,
market, intent, mesh_token.

**5 LAYERS** (compositions of the base): layer_settlement (gate +
treasury + count_house + receipts), layer_market (vswap + vauction +
market + broker), layer_trust (identity + reputation + provenance),
layer_distribution (grants + chest + royalty + tribute), layer_agent
(chiglet + aipi + escrow).

**7 SYSTEMS** (abstraction services): sys_escrow (the vault),
sys_agent_economy (the agent budget envelope), sys_governance (the
decision service), sys_marketplace (the full trade), sys_compliance
(the KYC-adjacent gate + privacy), sys_identity_verification (the
verifiable-actor stack), sys_settlement_rail (the complete rail
payment).

**4 META** (the smart orchestrator / turnkey layer): meta_orchestrator
(meta_intents — the 8 high-level actions + meta_dispatch_payment, the
fully wired payment machine in one call), meta_router (meta_route —
intent → kit-sequence wiring), meta_self (the self-knowledge
dashboard), meta_adaptation (meta_entropy — the organization metric,
4 dp, the negative-entropy measure).

**4 API** (the serverless external interface): api_gateway
(api_services — 10 versioned services + api_call, the single
permissionless entry, view-compatible), api_verify (api_receipt +
api_verify_receipt — the canonical receipts anyone verifies off-chain,
no operator), api_bridge (bridge_outbound/inbound/anchor — the
standardized bridge interface any relayer implements against, the
contract is the anchor, no central operator), api_index (api_query —
the indexer-free canonical views).

## 4. The rails (the card networks)

20-digit Luhn-valid rail numbers **outside ISO/IEC 7812** (no IIN, no
card-network claim, no PAN semantics):

| Network | Prefix | Inner check |
|---|---|---|
| Dragon | `8809` | ISO/IEC 7064 MOD 11,10 |
| Phoenix | `8829` | Damm |
| Thunderbird | `8849` | Verhoeff |

Cards issued to the ecosystem: social.near, devhub.near, discover.near,
near, ref-finance.near, near-intents.near, potlock.near, keypom.near,
mintbase.near, paras.near — all holding their Dragon, Phoenix, and
Thunderbird rail numbers.

## 5. The regulatory posture

Work-based, not passive (the SEC's Howey prong-4 / CFTC commodity
standard): the treasury pays bounties per verified receipt; workers post
slashable performance bonds; idle capital earns nothing. Escrow
self-containment: "spend only what you hold" — no pooling, lending, or
rehypothecation. Deployer compensation is documented vendor maintenance
from the 15% infrastructure bucket, never an equity dividend.

## 6. Quality control (military-grade)

| Check | Result |
|---|---|
| The QC harness (vs the kernel C source) | **55/55 PASS** |
| Durability stress (50-call sustained adversarial load) | **50/50 PASS** |
| High-scrutiny on-chain gauntlet (all layers live) | **20/20 PASS** |
| Complete system test (all 18 registry names + the 48-kit core) | **29/29 PASS** |
| Registration audit (66 capabilities resolve) | **66/66 ALIVE** |
| **TOTAL** | **220/220 PASS, zero bugs** |

The harness cross-validates the contract logic against the actual C
kernel source (compiled as a reference oracle): the gate's conjoin/
disjoin (all 256 pairs), the rail check digits (vs cn_check.c), the
treasury split (fuzz + u64 extremes), the invariant (0 violations
across all 4^n combos, n=1..8), and every kit's math.

## 7. The registration

The world-registry (`7eaa2c6f…`) lists **66 capabilities** — the 18
original names + the 48 kit families — **all resolving to ALIVE
contracts** (direct curl probes confirmed the code hashes). The 48 kits
+ 5 consolidated names resolve to zxv-core; the 13 originals to their
accounts.

## 8. THE FIREWORKS DISPLAY (2026-10-22)

The spectacular all-capabilities demonstration posted to the
social.near feed.

**Transaction:** `CsVLPfqbaSYHe1jaYrws8w6tTPzfkbsVLFPWKqNsx62S`
(block height 2210811…, `SuccessValue`)

**The post (verified live, containing every element):**

> 👑 **THE FIREWORKS — a new financial system is live on NEAR**
>
> By royal decree of the Sicilian Crown, and by the Council of Hands —
> including the retired Invisible Hand, Mr. Shanghai Tzu, Robin DeBanks,
> John Atlas Galt & Jolly Dragon Roger — represented by the law offices
> of Curzi, Jolly Dragon Roger & GPT, attorneys-at-large •
> Counselors-in-Chaos • Barristers-of-the-blockchain • No hourly rate,
> just 11% tribute — the party claims credit for the occurrence of a new
> financial system.
>
> The ZXV paraconsistent settlement layer — 48 kits in one micro
> contract, all live, all verified, all discoverable.
>
> THE INVARIANT: only TRUE settles. TRUE+FALSE=BOTH — contradictions are
> recorded, never coerced, never a panic.
>
> THE STACK (all live on NEAR mainnet, discoverable via vm_capabilities
> on the registry): [the full 13-address list]
>
> THE 48 KITS: [the complete kit catalogue]
>
> THE VERIFICATION (gas-free, run it yourself): [the view commands]
>
> REGULATORY POSTURE: work-based, not passive… Defensible under the
> SEC's Howey prong-4 and CFTC commodity standards.
>
> DEVELOPERS: build on it. The API is serverless (the contract IS the
> gateway), the bridge interface is a standard any relayer implements,
> the receipts verify off-chain with no operator. Developer kit:
> github.com/sykomiked/zxv-developer-kit · Proposals: NEP-659 · NEP-660
> · NEP-661
>
> The cards are issued to the ecosystem: [the 10 addresses]
>
> Long live the Crown. The tribute is 11%. The system is open.

**The Easter egg** (the humor + the hand): the Sicilian Crown decree,
the Council of Hands (the retired Invisible Hand, Mr. Shanghai Tzu,
Robin DeBanks, John Atlas Galt & Jolly Dragon Roger), the law offices of
Curzi, Jolly Dragon Roger & GPT (attorneys-at-large •
Counselors-in-Chaos • Barristers-of-the-blockchain • No hourly rate,
just 11% tribute) — the party claiming credit for the new financial
system. Deliberately placed at the top of the technical demonstration:
the humor and the substance in one stream.

**The design intent:** the display serves the technical audience (the
verifiable commands, the kit catalogue), the regulators (the Howey
prong-4 posture, the work-based economy), and the NEAR developer
community (the serverless API, the bridge standard, the developer kit,
the NEP references) — with the Crown's hand and humor framing the whole.

## 9. The developer kit

- **Repository:** https://github.com/sykomiked/zxv-developer-kit
- **Rust crate:** `zxv-sdk` (no_std, zero-dep core: the gate, the rail
  check digits, settle_or_hold) — 6 tests PASS
- **TypeScript SDK:** `@zxv/developer-kit` (discover, verifyCard,
  oraclePrice, isSettleable, chainHead) — verified against live mainnet
- **The NEP proposal:** https://github.com/sykomiked/ZXV-NEP-paraconsistent/blob/main/neps/nep-0000-paraconsistent-settlement-layer.md

## 10. The repositories and records

- **Main repo:** https://github.com/sykomiked/ZXV-NEP-paraconsistent
- **Developer kit:** https://github.com/sykomiked/zxv-developer-kit
- **Forensic ledger:** `docs/ledger/FORENSIC_LEDGER.md` (sections
  §1-§40: the kernel, the fabric, the DeFi stack, the migrations, the
  audits, the regulatory perimeter, the kit expansions, the
  registration, the fireworks)
- **The QC harness:** `tools/qc_audit.py` (the repeatable zero-bug
  gate, cross-validated against the kernel C source)
- **The expansion plan:** `docs/EXPANSION_PLAN_2NEAR.md`

## 11. The complete stack, one view

```
NEAR mainnet
├── 13 original subsystems (untouched): gate · ledger · registry ·
│   oracle · bus · shield · micro_infer · model_registry · consumer ·
│   paraconsistent_oracle · settlement_router · yhv_wrapper · ai_verdict
└── zxv-core (f18889ea…) — 48 kits, one micro contract
    ├── 28 BASE    (primitives)
    ├── 5  LAYERS  (compositions)
    ├── 7  SYSTEMS (abstraction services)
    ├── 4  META    (orchestration — the turnkey layer)
    └── 4  API     (the serverless external interface)
Registry: 66 capabilities, all resolving to ALIVE contracts
QC: 220/220 PASS, zero bugs
The fireworks: live on social.near (TX CsVLPfqbaSYHe1jaYrws8w6tTPzfkbsVLFPWKqNsx62S)
```

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC ·
SPDX-License-Identifier: Apache-2.0 · All on-chain claims verified
against NEAR mainnet 2026-10-22.*