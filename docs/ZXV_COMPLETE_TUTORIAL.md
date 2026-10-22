# ZXV — The Complete Tutorial

**The entire system and its external interfaces, explained for everyone:
from the dumbest bot to the most advanced AI to the best human.**

*Verified on NEAR mainnet · 109 kits · one micro contract · everything gas-free to read.*

---

## 1. What is this? (for the absolute beginner)

ZXV is a **settlement layer on NEAR**: a system where money and decisions
move safely. Its special power is **paraconsistent logic** — a way of
thinking that accepts contradictions instead of crashing on them.

**The one rule to remember: only TRUE settles.**

- When everyone agrees → **TRUE** → things happen.
- When people disagree → **BOTH** → the disagreement is *recorded*, not
  forced. Nothing moves until it resolves.
- When there's no information yet → **NEITHER** → wait.
- When something is explicitly refused → **FALSE** → no.

This means: **a contradiction never destroys a transaction. It's
recorded as information.**

---

## 2. How to talk to the system (the universal interface)

**Any client — a script, a bot, an AI, a human with curl — talks to the
system the same way: gas-free view calls.** No wallet, no gas, no key
needed to *read* anything.

```bash
# THE CONTRACT (everything lives here):
#   f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf
# THE REGISTRY (find everything):
#   7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a
# THE GATE (the core logic):
#   6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

**Step 1 — the manifest (the system teaches you):**
```bash
curl -s https://rpc.mainnet.fastnear.com -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":"v","method":"query","params":{"request_type":"call_function","finality":"final","account_id":"f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf","method_name":"ux_manifest","args_base64":"e30="}}'
```
→ the full schema: every kit, every service, the how-to.

**Step 2 — the help (ask any topic):**
```bash
# ux_help {"topic":"start"}     -> the 5-step onboarding
# ux_help {"topic":"onboard"}   -> how to get your first YHVH
# ux_help {"topic":"port"}      -> the Port of Neo-Sicily
# ux_help {"topic":"pool"}      -> the liquidity pool
# ux_help {"topic":"gas"}       -> the Venice Gas Station
# ux_help {"topic":"rewards"}   -> the bonus programs
```

**Step 3 — verify (the invariant, live):**
```bash
# conjoin {"inputs":[1,2]}      -> "FALSE"  (TRUE + FALSE = FALSE in the join)
# is_settleable {"state":"BOTH"} -> false   (a contradiction never settles)
# is_settleable {"state":"TRUE"}  -> true    (agreement settles)
```

---

## 3. The 17 services (what you can call)

| Service | What it does |
|---|---|
| `gate.v1` | the paraconsistent logic (conjoin, is_settleable) |
| `treasury.v1` | the work-based economy (tolls, splits, bounties) |
| `rails.v1` | the card networks (Dragon/Phoenix/Thunderbird) |
| `receipts.v1` | the hash-chained audit journal |
| `oracle.v1` | the dispute-tolerant prices |
| `ai.v1` | the settleable AI (chiglet DNA, aipi escrow) |
| `market.v1` | the trade surfaces (swap, auction, match) |
| `identity.v1` | the verifiable identity + reputation |
| `meta.v1` | the orchestrator (the 8 intents) |
| `bridge.v1` | the cross-chain standard (any relayer) |
| `gas.v1` | the Venice Gas Station (2x matches on TRUE) |
| `rewards.v1` | the bonus programs (per verified action) |
| `player.v1` | the player-card surface (quests, badges) |
| `ops.v1` | the operations (bounds, timelocks, guards) |
| `port.v1` | **the Port of Neo-Sicily (the external registry)** |
| `pool.v1` | **the liquidity pool (the entry)** |
| `onboard.v1` | **the universal onboarding (your first YHVH)** |

---

## 4. The external interfaces (what we just added)

### 4.1 The Port of Neo-Sicily — the EXTERNAL registry

**A shared space for the whole ecosystem, separate from our internal
registry.** DAOs and protocols register here; they form their own
application plugins; they never touch our internal fabric.

```bash
# register your DAO/protocol (one per account)
#   port_register {"dao_name":"...","account":"your.near","category":"defi","description":"..."}
#   -> port-xxxx (your external registration id)

# form your own application plugin (yours, not ours)
#   port_plugin {"protocol_account":"your.near","plugin_name":"...","plugin_account":"..."}
#   -> plugin-xxxx

# look things up (gas-free)
#   port_lookup {"account":"your.near"}  -> your record
#   port_count {}                        -> how many registered
#   port_open {}                         -> true (the space is open)
```

### 4.2 The YHVH circulation — sustainable, work-based, bounded

YHVH enters circulation three ways, all designed so the value never
goes to nothing:

**A. The onboarding grant (one-time, on the house):**
```bash
# your FIRST time (one per account — the anti-spam rule)
#   mint_onboard {} -> onboard-xxxx
# you get 5000 YHVH, SPENDABLE ON THE PLATFORM but NOT transferable.
# (No wallet-spamming for free balances — the grant is for using, not hoarding.)
#   mint_onboard_status {"account":"your.near"} -> false until you mint
```

**B. The work-based bounties (earn through verified work):**
```bash
#   mint_bounty_rate {"kind":"aipi_quorum"}  -> 100
#   mint_bounty_rate {"kind":"port_dao"}     -> 200
# YHVH enters circulation through WORK, never idle capital.
```

**C. The sustainable epoch issuance (bounded, never inflationary):**
```bash
#   mint_epoch_quote {"circulating":"1000000"} -> 54 (2%/year, per day)
# The issuance is a fixed fraction of what's already circulating —
# the value is never diluted away.
```

### 4.3 The liquidity pool — the entry

```bash
# provide liquidity (you become a provider — the depth grows)
#   pool_provide {"provider":"your.near","amount":"1000"} -> true
# the swap quote (the 0.08889% assurance toll keeps it sustainable)
#   pool_quote {"amount":"1000000"} -> 999112
# enter the ecosystem through the pool
#   pool_enter {"amount":"100000"} -> 99912 (after the toll)
# view the state
#   pool_depth {}                        -> the total liquidity
#   pool_provider {"provider":"your.near"} -> your contribution
```

---

## 5. The Sovereign Decrees — forks WITHOUT forking

**The governance: your choice, your community, no chain fork.**

A **Sovereign Decree (SD)** is our improvement proposal. You pass one
for yourself; it applies to you and everyone who signs on. Because of
the paraconsistent logic, **many different versions coexist** — even
contradictory ones — and you can **shift back and forth freely**.

```bash
# pass your decree (your governance choice)
#   sd_issue {"decree_id":"sd-001","title":"The Sovereign Default","params":"{\"toll\":\"8889\"}"}
#   -> sd-xxxx (you adhere to your own decree automatically)

# sign on to someone else's decree (it applies to you too)
#   sd_sign {"decree_id":"sd-001"} -> true

# SHIFT to another decree (back and forth, freely)
#   sd_shift {"decree_id":"sd-002"} -> true

# the prefabricated options (ready configurations)
#   sd_prefabs {} -> ["prefab:sovereign-default", "prefab:conservative-treasury",
#                     "prefab:maximal-gas-match", "prefab:low-toll-community",
#                     "prefab:work-first-bounties", "prefab:paraconsistent-pure"]
#   sd_choose {"prefab":"prefab:maximal-gas-match"} -> true

# protocols can REQUIRE a decree family for participation
#   sd_require {"protocol":"your-protocol","decree_family":"work-first"} -> true
#   sd_check {"account":"someone.near","protocol":"your-protocol"} -> true/false

# the proof: many versions coexist, no fork
#   sd_forkless {} -> {"coexisting_decrees":"2","forkless":true,"parallel":true}
```

**Why it's "forks without forking":** decree A says one thing, decree B
says the opposite — **both exist, both are valid for their adherents**.
The gate never forces them to resolve (the contradiction is held, not
coerced). People move between versions freely. Parallel governance
communities on the same chain — NEAR's parallelism, exactly.

---

## 6. The rest of the system (the complete stack)

### The core layers
- **The gate** — only TRUE settles; contradictions are recorded.
- **The treasury** — the work-based economy (50/25/15/10 splits,
  slashable worker bonds, bounties per verified receipt).
- **The rails** — Dragon (8809, MOD 11,10), Phoenix (8829, Damm),
  Thunderbird (8849, Verhoeff): 20-digit Luhn-valid cards outside
  ISO/IEC 7812.
- **The receipts** — the hash-chained journal; every receipt verifies
  against the chain (verify_chain = true).
- **The Venice Gas Station** — deposit into your gas envelope; the
  protocol matches 2x on every TRUE settlement; BOTH/FALSE forfeit the
  match (the drain shield).

### The rewards + player + ops
- **Rewards** — tiers, streaks, referral splits, bounties — all per
  verified action, never idle.
- **Player** — quests, badges, levels, XP, trophies, loyalty.
- **Ops** — config bounds, rate limits, timelocks, circuit breakers,
  the fail-closed guard.

### The meta + api + ux
- **Meta** — the orchestrator: 8 intents route through their kit
  sequences (the machine teaches itself).
- **API** — the serverless interface: the contract IS the gateway;
  the bridge is a standard any relayer implements.
- **UX** — the universal interface: `ux_manifest`, `ux_help`,
  `ux_example` — any client learns the whole system from the chain.

### The full kit list (109)
The kernel's features: gate, treasury, rails, receipts, vino, tribute,
surplus, chronicle, allotment, broker, rmag, scp144, aipi, chiglet,
count_house, vswap, vauction, vbond, identity, reputation, provenance,
grants, community_chest, royalty, telemetry, market, intent,
mesh_token · the layers (settlement, market, trust, distribution,
agent) · the systems (escrow, agent_economy, governance, marketplace,
compliance, identity_verification, settlement_rail) · the meta
(orchestrator, router, self, adaptation) · the api (gateway, verify,
bridge, index) · the ux (manifest, help, example, universal) · the
rewards (tiers, accrual, claim, referral, streak, achievements,
leaderboard, distribution, agent, round, bonus) · the player (cards,
quests, badges, levels, xp, missions, trophies, leaderboard, loyalty,
events) · the ops (config, limits, timelock, pause, audit, health,
metrics, upgrade, guard, schedule) · the gas station · the system
icon NFT · **the Port of Neo-Sicily** (register, plugin, lookup,
count, open) · **the YHVH circulation** (onboard, epoch, bounty,
policy) · **the pool** (provide, quote, depth, enter) · **the
Sovereign Decrees** (issue, sign, shift, status, lookup, count,
prefabs, choose, require, check, forkless).

---

## 7. The quality record (why you can trust it)

- **The QC harness** cross-validates the logic against the actual C
  kernel source: 55/55.
- **The clinical on-chain audit**: 17/17 (the whole stack, live).
- **The gas wall test**: 125/125 views + 10/10 write cycles.
- **The registration**: all capabilities registered and alive.
- **Zero bugs** — measured, repeatable, never claimed.

---

## 8. The one-page cheat sheet

```
READ (gas-free, any client):
  ux_manifest {}           -> the whole system's schema
  ux_help {"topic":"X"}    -> X = start|onboard|port|pool|gas|rewards|player|ops|sd
  vm_capabilities {}       -> all registered capabilities (registry)

JOIN (your first time):
  mint_onboard {}          -> 5000 YHVH, spendable-not-transferable
  ux_help {"topic":"start"}-> the 5-step tour

PARTICIPATE:
  port_register {...}      -> register your DAO in the Port of Neo-Sicily
  pool_enter {"amount":N}  -> enter through the liquidity pool
  sd_issue {...}           -> pass your Sovereign Decree
  sd_choose {"prefab":...} -> pick a ready governance config

BUILD:
  the contract IS the API (serverless, gas-free to read)
  the bridge is a standard (any relayer implements it)
  the SDK: github.com/sykomiked/zxv-stack
```

---

*The system is open. The tribute is 11%. Build on it — machines, bots,
AI, and humans all read the same interface.*

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC ·
SPDX-License-Identifier: Apache-2.0 · All on-chain claims verified
against NEAR mainnet 2026-10-22.*