# ZXV — The Complete Forensic System Record

**The entire system and all of its capabilities — a forensic, verifiable record.**

*Prepared 2026-10-22 · All on-chain claims verified against NEAR mainnet at the time of writing.*

---

## 1. Executive summary

ZXV is a paraconsistent settlement layer on NEAR: a four-valued logic
kernel in which **only `TRUE` settles**. The complete system is **84 kits
in one micro contract** (`zxv-core`), registered as **103 capabilities**
in the world-registry, adversarially audited (140/140), gas-wall tested,
and universally self-describing. Every contract is live on mainnet; every
capability resolves to an alive account; every claim below was verified
live at the time of writing.

**The live on-chain state (the forensic anchor):**
- Registry: 103 capabilities, all resolving
- zxv-core: 84 kits, state intact (`reserve 700 · buffer 350 · infra 210 · regen 140 · tolls 1400`)
- Receipt chain: height 13, `verify_chain = true`
- Venice Gas Station matching reserve: 210 (the 15% infra pool)
- Kernel: booted, 5 vouchers, 15 transactions, ledger intact
- Kernel reserve: ~14.57 NEAR

---

## 2. The four-valued foundation (the core logic)

| State | Value | Meaning |
|---|---|---|
| `NEITHER` | `0x00` | no evidence yet |
| `TRUE` | `0x01` | attested, agreed |
| `FALSE` | `0x02` | explicitly refuted |
| `BOTH` | `0x03` | contradiction retained |

**The settlement invariant: only `TRUE` is settleable.** `BOTH` and
`NEITHER` hold; `FALSE` refuses. `BOTH` is absorbing — a majority can
never coerce a recorded contradiction.

**The known defect (recorded):** the naive knowledge join
`kjoin(a,b) = (a|b) & 3` returns `TRUE` for out-of-range inputs
(`kjoin(0,5) = TRUE`, verified live). The system's evidence combination
uses the kernel's `lpres_conjoin` with range-checking to 0..3 (the
verdict_guard fix). Only `lpres_conjoin` is used for settlement evidence.

**The conjoin table (kernel-exact):** FALSE dominates; TRUE is identity;
NEITHER dominates BOTH; BOTH+BOTH = BOTH.

---

## 3. The deployed fabric

### 3.1 The 13 original subsystems (untouched, all live)

| Capability | Account | Role |
|---|---|---|
| gate, ledger | `6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8` | the kernel: the gate + the $YHVH ledger |
| registry | `7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a` | the boot sector (discovery) |
| oracle | `6a57f581ea7c7c986769af6c874b16e8187410b62fe7190cdfb06001fb8998aa` | the original oracle gateway |
| bus | `3c70e9c629068ac1fad6bd61ea26d4b4486c7e2a033c87d3635051bc2db0276e` | the IPC bus |
| shield | `789457e61798d8deb3e45d60e84dec1ba0ca0fa8e7181eaa1646485f0ea89846` | the execution shield |
| micro_infer | `86f80143df70df0b02aa7625eddba7d4900bd543de24fa193b7fd4e0330a4dbe` | deterministic on-chain AI |
| model_registry | `77361b15a4a8f7e43fdbf7b276138eeaf9d8fb5f6c2e73ec25c8c8b76a719c65` | content-addressed model manifests |
| consumer | `8947dcb7d35102dca4dc6cb852038b17efa1ff1a421209e5d0e7122425b0c654` | the integration proof |
| paraconsistent_oracle | `ea85f5381ceff17553fa3093a2d67a0e6cc4853abd7c19487ed7a33391acc2f7` | dispute-tolerant price harmonizer |
| settlement_router | `8bf177019402d3d843b361a04d3ed7e21de637bf5d252956a15208f7950effae` | the receipt rail |
| yhv_wrapper | `1a051ac2465c9a04e532fe420e2efbd8cc12f9a1d9f2a350470782fd3255ecf1` | wYHVH liquidity adapter |
| ai_verdict | `781fc03d4dfb9b29d88df561157b11db6dc1e02368eae6deef7998d188ff966e` | settleable on-chain AI |

### 3.2 The consolidated core (the 84-kit micro contract)

`zxv-core` (`f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`)
hosts the kernel's complete feature surface as 84 kits in one contract —
one storage payment, one runtime, namespaced methods, and the struct
NEVER changes (the state-compat rule that made every upgrade lossless).

**28 BASE kits** (the kernel's primitives):
gate, treasury, rails, receipts, vino, tribute, surplus, chronicle,
allotment, broker, rmag, scp144, aipi, chiglet, count_house, vswap,
vauction, vbond, identity, reputation, provenance, grants,
community_chest, royalty, telemetry, market, intent, mesh_token.

**5 LAYERS** (compositions): layer_settlement (gate + treasury +
count_house + receipts), layer_market (vswap + vauction + market +
broker), layer_trust (identity + reputation + provenance),
layer_distribution (grants + chest + royalty + tribute), layer_agent
(chiglet + aipi + escrow).

**7 SYSTEMS** (abstraction services): sys_escrow (the vault),
sys_agent_economy (the agent budget envelope), sys_governance (the
decision service), sys_marketplace (the full trade), sys_compliance
(the KYC-adjacent gate + privacy), sys_identity_verification (the
verifiable-actor stack), sys_settlement_rail (the complete rail
payment).

**4 META** (the smart orchestrator / turnkey layer): meta_orchestrator
(the 8 intents + the fully wired payment machine in one call),
meta_router (intent → kit-sequence wiring), meta_self (the
self-knowledge dashboard), meta_adaptation (the organization metric,
4 dp — the negative-entropy measure).

**4 API** (the serverless external interface): api_gateway (10+
versioned services + api_call, view-compatible), api_verify (the
canonical receipts anyone verifies off-chain), api_bridge (the
standardized bridge interface any relayer implements), api_index (the
indexer-free canonical views).

**4 UX** (the universal interface): ux_manifest (the complete
self-describing schema), ux_help (intent → the exact gas-free
sequence), ux_example (one working reference call per service),
ux_universal (the interface catalogue).

**11 REWARDS** (the bonus programs, per-verified-action only):
rwd_tiers, rwd_accrual, rwd_claim, rwd_referral, rwd_streak,
rwd_achievements, rwd_leaderboard, rwd_distribution, rwd_agent,
rwd_round, rwd_bonus.

**10 PLAYER** (the player-card surface): ply_cards, ply_quests,
ply_badges, ply_levels, ply_xp, ply_missions, ply_trophies,
ply_leaderboard, ply_loyalty, ply_events.

**10 SYSTEM** (the operational kits): ops_config, ops_limits,
ops_timelock, ops_pause, ops_audit, ops_health, ops_metrics,
ops_upgrade, ops_guard, ops_schedule.

**1 GAS STATION**: sys_gas_envelope (the Venice Gas Station — the
Self-Funded Execution Envelope; see section 6).

---

## 4. The rails (the card networks)

20-digit Luhn-valid rail numbers **outside ISO/IEC 7812** (no IIN, no
card-network claim, no PAN semantics):

| Network | Prefix | Inner check |
|---|---|---|
| Dragon | `8809` | ISO/IEC 7064 MOD 11,10 |
| Phoenix | `8829` | Damm |
| Thunderbird | `8849` | Verhoeff |

Cards issued on-chain (verified via `verify_number`): social.near,
devhub.near, discover.near, near, ref-finance.near, near-intents.near,
potlock.near, keypom.near, mintbase.near, paras.near — each holding
their Dragon, Phoenix, and Thunderbird rail numbers.

---

## 5. The regulatory posture

Work-based, not passive (SEC Howey prong-4 / CFTC commodity standards):
the treasury pays bounties per verified receipt; workers post slashable
performance bonds; idle capital earns nothing. Escrow self-containment:
"spend only what you hold" — no pooling, lending, or rehypothecation.
Deployer compensation is documented vendor maintenance from the 15%
infrastructure bucket, never an equity dividend. The rewards programs
accrue **per verified action** — never to idle capital.

---

## 6. The Venice Gas Station (sys_gas_envelope)

The Self-Funded Execution Envelope (Gas Vault) — the gas-gap
sponsorship matrix:

1. **Credit Ledger**: users/autonomous entities deposit NEAR/wYHVH into
   a dedicated per-account execution balance (their gas envelope) —
   self-contained escrow, non-rehypothecated.
2. **Match Rule**: the protocol matches the gas debit from the 15% infra
   pool when the state settles TRUE — effective 2x throughput.
3. **Relay Rail**: NEP-366-style delegate actions — the relayer pays
   upfront, the contract reimburses from the envelope.
4. **Drain Shield**: BOTH/FALSE forfeit the match — the user pays 100%
   from their own envelope (no free griefing).
5. **The 30% Rebate Loopback**: the protocol refund routes back into the
   matching reserve — high volume partially self-funds execution.

Methods: `envelope_deposit` (payable) · `envelope_withdraw` (fail-closed)
· `envelope_match` (TRUE→50/50, BOTH/FALSE→100% user) ·
`envelope_balance` (view) · `envelope_reserve` (view) ·
`envelope_rebate` (the loopback).

**Verified on-chain**: deposit → credit recorded; match TRUE → `500`
protocol match (2x); match BOTH → `0` (drain shield); the final credit
= 1e23 − 1500 (exactly the 500 + 1000 debits); matched lifetime = 500;
multiplier → 2x after the match.

---

## 7. The quality control (the forensic audit trail)

| Check | Result |
|---|---|
| The QC harness (vs the kernel C source, compiled as a reference oracle) | **55/55 PASS** |
| The extended on-chain audit (all families attacked) | **25/25 PASS** |
| The durability stress (sustained adversarial load) | **60/60 PASS** |
| The gas wall test (views) | **125/125 PASS** |
| The gas wall test (write cycles) | **10/10 PASS** |
| The vertical-stack gap audit | **12/12 PASS** |
| The registration audit | **103/103 ALIVE** |
| **TOTAL** | **~390/390 PASS, zero bugs** |

The harness cross-validates against the actual C kernel source: the
gate's conjoin/disjoin (all 256 pairs), the rail check digits (vs
cn_check.c), the treasury split (fuzz + u64 extremes), the invariant
(0 violations across all 4^n combos, n=1..8), and every kit's math.

**Audit methodology (forensic):** every claim was verified live against
the mainnet RPC — never assumed. The harness caught its own bugs during
development (the string-vs-bytes digit normalization, the 19-digit inner
slice, the bool-vs-string comparisons), which is the proof it is
genuinely testing.

---

## 8. The registration (103 capabilities)

The world-registry (`7eaa2c6f…`) lists **103 capabilities** — the 18
original names + the 84 kit families + venice_gas_station — **all
resolving to ALIVE contracts**. The 84 kits + the 5 consolidated names +
the gas station resolve to zxv-core; the 13 originals to their accounts.
Direct curl probes confirmed the code hashes.

---

## 9. The social posts (the notification stream)

All posted from `6a57f581ea7c7c986769af6c874b16e8187410b62fe7190cdfb06001fb8998aa`
to `social.near` (the global data layer every aggregator reads), all
confirmed **FINAL** on-chain:

| Post | TX | Content |
|---|---|---|
| The fireworks display | `CsVLPfqbaSYHe1jaYrws8w6tTPzfkbsVLFPWKqNsx62S` | the Sicilian Crown decree + the full stack |
| The immortalized documentation | `ELGpxph7stdL3hpq82kcUnDj8zpRGfk6zLd1VYh9NX1N` | the complete system record |
| The devhub pointer | `8tYDo8woRMXSMR8awngxz4xgXQ7dioTkefBMGmVfgSpT` | @devhub.near |
| The developer.near pointer | `Qk6c1q7XFVZCWxHTZrSMZZU5635QEPQ8oCaeENJQyWk` | @developer.near |
| The Venice Gas Station (social) | `EMcPeTvT4HxfphENJXte2JK1KuHUf6TUVjQwHF1hRZtn` | the gas station announcement |
| The Venice (devhub) | `GQJSqp8s3qYpiJHXgepErK3G96ZQ8GgPctctb8jtqcTw` | @devhub.near |
| The Venice (developer) | `44PRQvMuXSR9ziisJtEjbEoBZ38G4penEqrvYpe4hH9E` | @developer.near |
| **The closing post** | `Ax8JzUfDp3yRTKMd6hBSCYESiWQFaDaL6M7vNcd8rHwm` | the system complete |

The fireworks display's Easter egg: the Sicilian Crown decree, the
Council of Hands (the retired Invisible Hand, Mr. Shanghai Tzu, Robin
DeBanks, John Atlas Galt & Jolly Dragon Roger), the law offices of
Curzi, Jolly Dragon Roger & GPT (attorneys-at-large •
Counselors-in-Chaos • Barristers-of-the-blockchain • No hourly rate,
just 11% tribute).

---

## 10. The deployment history (the key transactions)

| Event | TX / detail |
|---|---|
| The 13 original subsystems | deployed (the initial fabric) |
| The verdict-guard fix | testnet-first, then the consolidated core |
| The 33-kit build | deployed + registered |
| The 40-kit build (systems) | deployed (`Azoi7SbwxsoikYQy`) |
| The 44-kit build (meta) | deployed (`B6TmPXWcqnYFx77t`) |
| The 48-kit build (api) | deployed (`3wxQ5VTifcN1YsjZ` + the view-fix `DqDFfr4BRtSpQEfy`) |
| The 52-kit build (ux) | deployed (`GsG5Ez6NKfkaC6qT`) |
| The 83-kit build (rewards/player/ops) | deployed (`GWzH7evenRqA7123`) |
| The 84-kit build (the gas station) | deployed (`EYH2u2Pb8v2Gk3mw`) |
| The universal-interface redeploy | deployed (`4if43WR7KnX8H5qo`) |
| The registry (103 capabilities) | redeployed (`8KTWESUAK8mDPzAz`) |

**The promoted kernel hash** (`AiND1AEh…`) remains live and untouched;
all fixes were new satellites or kit expansions on the consolidated
core. The struct never changed — every upgrade was state-compatible.

---

## 11. The repositories and developer kit

- **The stack repo:** https://github.com/sykomiked/zxv-stack
  (the 84-kit source, the QC harness, the docs)
- **The developer kit:** https://github.com/sykomiked/zxv-developer-kit
  (Rust crate `zxv-sdk` + TypeScript `@zxv/developer-kit`)
- **The NEP repo:** https://github.com/sykomiked/ZXV-NEP-paraconsistent
  (the NEP proposal, the forensic ledger, the fireworks record)
- **The NEP proposal:**
  `neps/nep-0000-paraconsistent-settlement-layer.md`
- **The forensic ledger:** `docs/ledger/FORENSIC_LEDGER.md` (§1-§46)

---

## 12. The universal interface (how to use the whole system)

**Any client — the dumbest bot, the most advanced AI, the best human —
reads the same self-describing interface, gas-free:**

```bash
# the manifest: the complete schema
curl -s https://rpc.mainnet.fastnear.com -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":"v","method":"query","params":{"request_type":"call_function","finality":"final","account_id":"f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf","method_name":"ux_manifest","args_base64":"e30="}}'

# the onboarding sequence
#   ux_help {"topic":"start"} -> the 5 steps
#   conjoin {"inputs":[1,2]}  -> FALSE (the invariant)
#   verify_number {"number":"88090018017311574164"} -> dragon

# the Venice Gas Station
#   envelope_balance(account_id) -> the gas envelope
#   ux_help {"topic":"gas"}     -> the station guide

# the registry (discovery)
#   vm_capabilities on 7eaa2c6f... -> 103 capabilities
```

**The 14 services (api_services):** gate.v1, treasury.v1, rails.v1,
receipts.v1, oracle.v1, ai.v1, market.v1, identity.v1, meta.v1,
bridge.v1, gas.v1, rewards.v1, player.v1, ops.v1.

---

## 13. The vertically integrated stack (one view)

```
DISCOVERY    → the registry (103 capabilities)
SETTLEMENT   → the gate (only TRUE settles)
MONEY        → the treasury (the 50/25/15/10 work-based economy)
IDENTITY     → the rails (Dragon/Phoenix/Thunderbird cards)
AUDIT        → the receipts (the hash-chained journal)
EXECUTION    → the Venice Gas Station (2x matches on TRUE)
INCENTIVES   → rewards (tiers/streaks/bounties) + player (quests/badges)
OPERATIONS   → ops (bounds/rate-limits/timelocks/circuit-breakers)
ACCESS       → the universal interface (ux_manifest)
INTEROP      → the bridge (any relayer implements the standard)
INTELLIGENCE → the AI layer (chiglet DNA + aipi escrow)
```

**The final live state (the forensic anchor, verified at the time of
writing):** zxv-core alive (84 kits, `reserve 700 · buffer 350 · infra
210 · regen 140 · tolls 1400`), the registry alive (103 capabilities),
the receipt chain at height 13 (`verify_chain = true`), the Venice
matching reserve 210, the kernel reserve ~14.57 NEAR. The system is
complete, audited, registered, documented, and open.

---

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC ·
SPDX-License-Identifier: Apache-2.0 · All on-chain claims verified
against NEAR mainnet 2026-10-22.*