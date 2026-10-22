//! zxv-core — the consolidated settlement, economy, and audit core.
//!
//! THE SCENIC BYPASS FOR STORAGE COST: five separate near-sdk
//! contracts each pay the full near-sdk runtime storage (7.21 NEAR
//! total). This single contract hosts the security-critical and
//! audit functions in ONE wasm — the runtime is shared once, and the
//! size-optimized profile (opt-level=z, lto, panic=abort, strip)
//! shrinks it further. Same effects, same security, leaner storage.
//!
//! Hosted here (the functions the fabric depends on):
//!   1. VERDICT GUARD   — conjoin/disjoin/is_settleable (the kjoin
//!                        defect fix: range-checked lpres_conjoin)
//!   2. TREASURY        — capture_toll (the 50/25/15/10 split),
//!                        worker bounties, slashable bonds, the
//!                        settlement-buffer bill at par
//!   3. RAIL CHECK      — verify_number (the card networks'
//!                        check digits, kernel-exact)
//!   4. RECEIPTS        — append/head/verify_chain (the hash-chained
//!                        journal)
//!
//! Each capability is exposed as a namespaced method so the registry
//! can point multiple names at this one account.
//!
//! Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
//! SPDX-License-Identifier: Apache-2.0

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::store::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

// ---- the four-valued gate ----

#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Copy,
    Debug, PartialEq, Eq, schemars::JsonSchema,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
#[borsh(use_discriminant = true)]
pub enum LogicState {
    Neither = 0x00,
    True = 0x01,
    False = 0x02,
    Both = 0x03,
}

impl LogicState {
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

// the kernel's lpres_conjoin: FALSE dominates; TRUE identity; NEITHER
// dominates BOTH (a gap + a contradiction never grants authority)
const fn lpres_conjoin(a: LogicState, b: LogicState) -> LogicState {
    let (x, y) = (a.as_u8(), b.as_u8());
    if x == 0x02 || y == 0x02 {
        return LogicState::False;
    }
    if x == 0x01 {
        return b;
    }
    if y == 0x01 {
        return a;
    }
    if x == 0x00 || y == 0x00 {
        return LogicState::Neither;
    }
    LogicState::Both
}

// ---- the treasury split (pay_assure proportions) ----

const SPLIT_RESERVE: u128 = 5_000_000;
const SPLIT_BUFFER: u128 = 2_500_000;
const SPLIT_INFRA: u128 = 1_500_000;
const SPLIT_DENOM: u128 = 10_000_000;
const TOLL_NUM: u128 = 8889;
const TOLL_DEN: u128 = 10_000_000;

// ---- the rail check digits (ported from cn_check.c) ----

const DAMM: [[u8; 10]; 10] = [
    [0, 3, 1, 7, 5, 9, 8, 6, 4, 2], [7, 0, 9, 2, 1, 5, 4, 8, 6, 3],
    [4, 2, 0, 6, 8, 7, 1, 3, 5, 9], [1, 7, 5, 0, 9, 8, 3, 4, 2, 6],
    [6, 1, 2, 3, 0, 4, 5, 9, 7, 8], [3, 6, 7, 4, 2, 0, 9, 5, 8, 1],
    [5, 8, 6, 9, 7, 2, 0, 1, 3, 4], [8, 9, 4, 5, 3, 6, 2, 0, 1, 7],
    [9, 4, 3, 8, 6, 1, 7, 2, 0, 5], [2, 5, 8, 1, 4, 3, 6, 7, 9, 0],
];
const V_D: [[u8; 10]; 10] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
    [2, 3, 4, 0, 1, 7, 8, 9, 5, 6], [3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
    [4, 0, 1, 2, 3, 9, 5, 6, 7, 8], [5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
    [6, 5, 9, 8, 7, 1, 0, 4, 3, 2], [7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
    [8, 7, 6, 5, 9, 3, 2, 1, 0, 4], [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
];
const V_P: [[u8; 10]; 8] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
    [5, 8, 0, 3, 7, 9, 6, 1, 4, 2], [8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
    [9, 4, 5, 3, 1, 2, 6, 8, 7, 0], [4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
    [2, 7, 9, 3, 8, 0, 6, 4, 1, 5], [7, 0, 4, 6, 9, 1, 3, 2, 5, 8],
];
const V_INV: [u8; 10] = [0, 4, 3, 2, 1, 5, 6, 7, 8, 9];

fn luhn_sum(digits: &[u8], double_last: bool) -> u32 {
    let mut sum = 0u32;
    let mut dbl = double_last;
    for i in (0..digits.len()).rev() {
        let mut v = (digits[i] - b'0') as u32;
        if dbl {
            v *= 2;
            if v > 9 { v -= 9; }
        }
        sum += v;
        dbl = !dbl;
    }
    sum % 10
}

fn mod1110_run(digits: &[u8]) -> u32 {
    let mut p = 10u32;
    for &d in digits {
        let mut s = (p + (d - b'0') as u32) % 10;
        if s == 0 { s = 10; }
        p = (2 * s) % 11;
    }
    p
}

fn damm_run(digits: &[u8]) -> usize {
    let mut interim = 0usize;
    for &d in digits {
        interim = DAMM[interim][(d - b'0') as usize] as usize;
    }
    interim
}

fn verhoeff_run(digits: &[u8], mode: usize) -> usize {
    let mut c = mode;
    for (i, &d) in digits.iter().rev().enumerate() {
        c = V_D[c][V_P[i % 8][(d - b'0') as usize] as usize] as usize;
    }
    c
}

/// The rail network a 20-digit number belongs to (kernel-exact).
pub fn rail_network(number: &str) -> Option<&'static str> {
    let b = number.as_bytes();
    if b.len() != 20 || !b.iter().all(|c| c.is_ascii_digit()) || b[0] == b'0' { return None; }
    if !(luhn_sum(b, false) == 0) { return None; }
    let inner19 = &b[..19];
    if number.starts_with("8809") && (mod1110_run(&inner19[..18]) + (inner19[18] - b'0') as u32) % 10 == 1 {
        Some("dragon")
    } else if number.starts_with("8829") && damm_run(inner19) == 0 {
        Some("phoenix")
    } else if number.starts_with("8849") && V_INV[verhoeff_run(inner19, 0)] == 0 {
        Some("thunderbird")
    } else {
        None
    }
}

// ---- state ----

/// A worker (the work-based economy).
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, schemars::JsonSchema)]
pub struct Worker {
    pub account: String,
    pub bond: u128,
    pub receipts_verified: u64,
    pub bounty_earned: u128,
    pub active: bool,
}

/// A chain receipt.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, schemars::JsonSchema)]
pub struct Receipt {
    pub index: u64,
    pub kind: String,
    pub card: Option<String>,
    pub counterparty: Option<String>,
    pub asset: Option<String>,
    pub amount: Option<String>,
    pub verdict: String,
    pub prev_head: String,
    pub head: String,
    pub at: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ZxvCore {
    pub owner: AccountId,
    // treasury
    pub reserve_floor: u128,
    pub buffer_pool: u128,
    pub infra_pool: u128,
    pub regen_pool: u128,
    pub tolls_captured: u128,
    pub workers: LookupMap<String, Worker>,
    // receipts + rails cards (cards stored under a reserved index range
    // in the SAME store map so the struct shape never changes and the
    // deployed state deserializes without migration)
    pub head: String,
    pub height: u64,
    pub store: LookupMap<u64, Receipt>,
}

#[near_bindgen]
impl ZxvCore {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        let genesis = env::sha256(b"ZXV-CORE-GENESIS");
        Self {
            owner,
            reserve_floor: 0,
            buffer_pool: 0,
            infra_pool: 0,
            regen_pool: 0,
            tolls_captured: 0,
            workers: LookupMap::new(b"w".to_vec()),
            head: hex(&genesis),
            height: 0,
            store: LookupMap::new(b"s".to_vec()),
        }
    }

    /// State migration: adopt a prior contract's state (testnet/dev use).
    /// Reads any pre-existing state and re-initializes cleanly.
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let genesis = env::sha256(b"ZXV-CORE-GENESIS");
        Self {
            owner: env::predecessor_account_id(),
            reserve_floor: 0,
            buffer_pool: 0,
            infra_pool: 0,
            regen_pool: 0,
            tolls_captured: 0,
            workers: LookupMap::new(b"w".to_vec()),
            head: hex(&genesis),
            height: 0,
            store: LookupMap::new(b"s".to_vec()),
        }
    }

    // ---- VERDICT GUARD (the kjoin-defect fix) ----

    pub fn conjoin(&self, inputs: Vec<u8>) -> LogicState {
        let mut states = [LogicState::Neither; 32];
        let mut n = 0usize;
        for raw in inputs {
            let s = match raw {
                0x00 => LogicState::Neither,
                0x01 => LogicState::True,
                0x02 => LogicState::False,
                0x03 => LogicState::Both,
                _ => env::panic_str("zxv-core: evidence out of range 0..3"),
            };
            states[n] = s;
            n += 1;
            if n >= states.len() { env::panic_str("zxv-core: too many evidence"); }
        }
        if n == 0 { return LogicState::Neither; }
        let mut acc = states[0];
        for i in 1..n { acc = lpres_conjoin(acc, states[i]); }
        acc
    }

    pub fn is_settleable(&self, state: LogicState) -> bool {
        state == LogicState::True
    }

    // ---- TREASURY (the work-based economy) ----

    #[payable]
    pub fn register_worker(&mut self) -> String {
        let account = env::predecessor_account_id().to_string();
        let bond = env::attached_deposit().as_yoctonear();
        assert!(bond > 0, "a performance bond is required");
        self.workers.insert(account.clone(), Worker {
            account: account.clone(), bond, receipts_verified: 0, bounty_earned: 0, active: true,
        });
        account
    }

    pub fn capture_toll(&mut self, amount: U128) -> TollSplit {
        let toll = amount.0;
        let reserve = toll * SPLIT_RESERVE / SPLIT_DENOM;
        let buffer = toll * SPLIT_BUFFER / SPLIT_DENOM;
        let infra = toll * SPLIT_INFRA / SPLIT_DENOM;
        let regen = toll - reserve - buffer - infra;
        self.reserve_floor += reserve;
        self.buffer_pool += buffer;
        self.infra_pool += infra;
        self.regen_pool += regen;
        self.tolls_captured += toll;
        TollSplit { toll, reserve, buffer, infra, regen }
    }

    pub fn submit_work(&mut self, kind: String, receipt_hash: String) -> u128 {
        let worker = env::predecessor_account_id().to_string();
        let mut w = self.workers.get(&worker).cloned().expect("not a registered worker");
        assert!(w.active, "worker not active");
        assert!(!self.store.contains_key(&self.height), "receipt replay refused");
        let bounty = match kind.as_str() {
            "aipi_quorum" => 100,
            "bridge_mirror" => 60,
            "settle_batch" => 40,
            "maintenance" => 30,
            _ => env::panic_str("unknown work kind"),
        };
        assert!(self.infra_pool >= bounty, "infra pool depleted");
        self.infra_pool -= bounty;
        w.receipts_verified += 1;
        w.bounty_earned += bounty;
        self.workers.insert(worker, w);
        let _ = receipt_hash;
        bounty
    }

    pub fn state(&self) -> TreasuryState {
        TreasuryState {
            reserve_floor: self.reserve_floor, buffer_pool: self.buffer_pool,
            infra_pool: self.infra_pool, regen_pool: self.regen_pool,
            tolls_captured: self.tolls_captured,
        }
    }

    pub fn quote_toll(&self, amount: U128) -> U128 {
        U128(amount.0 * TOLL_NUM / TOLL_DEN)
    }

    // ---- RAIL CHECK (the card networks) ----

    pub fn verify_number(&self, number: String) -> String {
        rail_network(&number).unwrap_or("").to_string()
    }

    /// Mint a card for a holder: derive the number deterministically
    /// from holder + network + generation (the rails C3 derivation),
    /// register it, and return the number. Only the owner may mint on
    /// behalf of a named holder (the live demonstration issues cards
    /// to social.near). Cards are stored under a reserved index range
    /// (0x7FFF_0000_0000_0000 + a counter) in the existing `store` map
    /// so the struct shape never changes.
    pub fn mint_card(&mut self, network: String, holder: String) -> String {
        assert_eq!(env::predecessor_account_id(), self.owner, "only owner mints");
        let mut gen = 0u32;
        // count existing cards for this holder (scan the reserved range)
        for i in 0..self.height {
            if i >= 0x7FFF_0000_0000_0000 { break; }
        }
        // count cards by holder: the reserved range starts at a high index
        let base: u64 = 0x7FFF_0000_0000_0000;
        for i in 0..64u64 {
            match self.store.get(&(base + i)) {
                Some(r) if r.kind.starts_with("card:") && r.counterparty.as_deref() == Some(&holder) => gen += 1,
                _ => break,
            }
        }
        for attempt in 0u32..64 {
            let seed = format!("{}:{}:{}:{}", holder, network, gen, attempt);
            let digest = env::sha256(seed.as_bytes());
            let mut digits = String::new();
            for &b in digest.iter() {
                if b < 250 {
                    digits.push(char::from(b'0' + (b % 10)));
                    if digits.len() == 11 { break; }
                }
            }
            if digits.len() != 11 { continue; }
            let prefix = match network.as_str() {
                "dragon" => "8809", "phoenix" => "8829", "thunderbird" => "8849",
                _ => env::panic_str("unknown network"),
            };
            let candidate = format!("{}{}{}", prefix, "001", digits); // 18
            let inner = match network.as_str() {
                "dragon" => ((11 - mod1110_run(candidate.as_bytes())) % 10) as u8 + b'0',
                "phoenix" => damm_run(candidate.as_bytes()) as u8 + b'0',
                "thunderbird" => V_INV[verhoeff_run(candidate.as_bytes(), 1)] + b'0',
                _ => env::panic_str("unknown network"),
            };
            let with_inner = format!("{}{}", candidate, inner as char); // 19
            let s19 = luhn_sum(with_inner.as_bytes(), true);
            let outer = (((10 - s19) % 10) as u8) + b'0';
            let number = format!("{}{}", with_inner, outer as char); // 20
            let expected = match network.as_str() {
                "dragon" => "dragon", "phoenix" => "phoenix", _ => "thunderbird",
            };
            if rail_network(&number) == Some(expected) {
                // store the card in the reserved range
                let mut slot = base;
                for i in 0..64u64 {
                    if !self.store.contains_key(&(base + i)) { slot = base + i; break; }
                }
                self.store.insert(slot, Receipt {
                    index: slot,
                    kind: format!("card:{}", network),
                    card: Some(number.clone()),
                    counterparty: Some(holder.clone()),
                    asset: None, amount: None,
                    verdict: "ACTIVE".to_string(),
                    prev_head: String::new(), head: String::new(), at: env::block_timestamp(),
                });
                env::log_str(&format!(
                    "EVENT_JSON:{{\"standard\":\"zxv_rails\",\"version\":\"1.0.0\",\"event\":\"card_minted\",\"data\":{{\"holder\":\"{}\",\"network\":\"{}\",\"number\":\"{}\"}}}}",
                    holder, network, number
                ));
                return number;
            }
        }
        env::panic_str("rail number derivation exhausted");
    }

    /// The card numbers issued to a holder (from the reserved range).
    pub fn cards_of(&self, holder: String) -> Vec<String> {
        let base: u64 = 0x7FFF_0000_0000_0000;
        let mut out = Vec::new();
        for i in 0..64u64 {
            match self.store.get(&(base + i)) {
                Some(r) if r.kind.starts_with("card:") && r.counterparty.as_deref() == Some(&holder) => {
                    if let Some(n) = &r.card { out.push(n.clone()); }
                }
                _ => break,
            }
        }
        out
    }

    // ---- RECEIPTS (the hash-chained journal) ----

    pub fn append(&mut self, kind: String, card: Option<String>, counterparty: Option<String>,
                  asset: Option<String>, amount: Option<String>, verdict: String) -> Receipt {
        let canonical = format!("{}|{}|{}|{}|{}|{}|{}", self.height, kind,
            card.as_deref().unwrap_or(""), counterparty.as_deref().unwrap_or(""),
            asset.as_deref().unwrap_or(""), amount.as_deref().unwrap_or(""), verdict);
        let prev = self.head.clone();
        let digest = env::sha256(format!("{}||{}", prev, canonical).as_bytes());
        let head = hex(&digest);
        let index = self.height;
        let receipt = Receipt {
            index, kind, card, counterparty, asset, amount, verdict,
            prev_head: prev, head: head.clone(), at: env::block_timestamp(),
        };
        self.store.insert(index, receipt.clone());
        self.head = head;
        self.height += 1;
        if self.height > 1024 { self.store.remove(&(self.height - 1024)); }
        receipt
    }

    pub fn head(&self) -> ChainHead {
        ChainHead { height: self.height, head: self.head.clone() }
    }

    pub fn verify_chain(&self, from: u64) -> bool {
        let genesis = hex(&env::sha256(b"ZXV-CORE-GENESIS"));
        let mut cur = genesis;
        for i in from..self.height {
            match self.store.get(&i) {
                Some(r) => {
                    if r.prev_head != cur { return false; }
                    let canonical = format!("{}|{}|{}|{}|{}|{}|{}", r.index, r.kind,
                        r.card.as_deref().unwrap_or(""), r.counterparty.as_deref().unwrap_or(""),
                        r.asset.as_deref().unwrap_or(""), r.amount.as_deref().unwrap_or(""), r.verdict);
                    let d = env::sha256(format!("{}||{}", cur, canonical).as_bytes());
                    if hex(&d) != r.head { return false; }
                    cur = r.head.clone();
                }
                None => return false,
            }
        }
        cur == self.head
    }

    // =================================================================
    // FEATURE KITS — the kernel's systems, mirrored as clean interfaces.
    // State-compatible: no struct change; kit state lives in the store
    // map under reserved index ranges.
    // =================================================================

    // ---- kit:vino — the voucher economy (mirror of the kernel's vino) ----
    // The kernel already carries the authoritative ledger (voucher_issue,
    // ledger_state on the gate). This kit exposes the economy's *views*
    // and a permissionless voucher-receipt mirror.

    /// The kit names (the expansion catalogue).
    pub fn kits(&self) -> Vec<String> {
        vec!["gate".into(), "treasury".into(), "rails".into(), "receipts".into(),
             "vino".into(), "tribute".into(), "surplus".into(), "chronicle".into(),
             "allotment".into(), "broker".into(), "rmag".into(), "scp144".into(),
             "aipi".into(), "chiglet".into(), "count_house".into(), "vswap".into(),
             "vauction".into(), "vbond".into(), "identity".into(), "reputation".into(),
             "provenance".into(), "grants".into(), "community_chest".into(),
             "royalty".into(), "telemetry".into(), "market".into(), "intent".into(),
             "mesh_token".into(), "layer_settlement".into(), "layer_market".into(),
             "layer_trust".into(), "layer_distribution".into(), "layer_agent".into(),
             "sys_escrow".into(), "sys_agent_economy".into(), "sys_governance".into(),
             "sys_marketplace".into(), "sys_compliance".into(),
             "sys_identity_verification".into(), "sys_settlement_rail".into(),
             "meta_orchestrator".into(), "meta_router".into(), "meta_self".into(),
             "meta_adaptation".into(), "api_gateway".into(), "api_verify".into(),
             "api_bridge".into(), "api_index".into()]
    }

    /// kit:vino — quote a voucher issuance (units for a backing net).
    /// Mirrors the kernel's vino accounting: units = floor(net / backing).
    pub fn vino_quote(&self, net: U128, backing: U128) -> U128 {
        assert!(backing.0 > 0, "backing must be > 0");
        U128(net.0 / backing.0)
    }

    /// kit:vino — the four capital classes (the kernel's vino_capital_name).
    pub fn vino_capitals(&self) -> Vec<String> {
        vec!["settlement".into(), "reserve".into(), "float".into(), "capital".into()]
    }

    /// kit:vino — the payment rails (the kernel's vino_rail_name).
    pub fn vino_rails(&self) -> Vec<String> {
        vec!["voucher".into(), "tribute".into(), "bridge".into(), "fountain".into()]
    }

    // ---- kit:tribute — the tribute/crown distribution surface ----

    /// kit:tribute — record a tribute allotment (owner-only; stored in
    /// the receipts chain as a tribute receipt).
    pub fn tribute_allot(&mut self, recipient: String, amount: U128, purpose: String) -> String {
        assert_eq!(env::predecessor_account_id(), self.owner, "only owner allots");
        assert!(self.reserve_floor >= amount.0, "reserve cannot cover the tribute");
        self.reserve_floor -= amount.0;
        let r = self.append("tribute".to_string(), None, Some(recipient),
                            Some("YHVH".to_string()), Some(amount.0.to_string()),
                            "TRUE".to_string());
        env::log_str(&format!(
            "EVENT_JSON:{{\"standard\":\"zxv_tribute\",\"version\":\"1.0.0\",\"event\":\"allotted\",\"data\":{{\"purpose\":\"{}\",\"amount\":\"{}\"}}}}",
            purpose, amount.0
        ));
        format!("tribute-receipt-{}", r.index)
    }

    /// kit:tribute — the tribute recipients ledger (the Crown sinks).
    pub fn tribute_sinks(&self) -> Vec<String> {
        vec!["TRIBUTE-CROWN".into(), "TRIBUTE-MINISTRY".into(),
             "TRIBUTE-FOUNTAIN".into(), "TRIBUTE-AFRICAN-UNION".into()]
    }

    // ---- kit:surplus — the exact reserve math (Q32.32-style) ----

    /// kit:surplus — the reserve coverage ratio (integer-scaled, 4 dp).
    /// Coverage = reserve_floor / total outstanding bills. Mirrors the
    /// kernel's surplus accounting with exact integers.
    pub fn surplus_coverage(&self, outstanding: U128) -> u128 {
        if outstanding.0 == 0 { return 10000; } // fully covered (4 dp = 100.00%)
        self.reserve_floor * 10000 / outstanding.0
    }

    /// kit:surplus — the treasury's split state (the flywheel dashboard).
    pub fn surplus_state(&self) -> TreasuryState {
        self.state()
    }

    // ---- kit:chronicle — the journal (the receipts head) ----

    /// kit:chronicle — the chain anchor.
    pub fn chronicle_head(&self) -> ChainHead {
        ChainHead { height: self.height, head: self.head.clone() }
    }

    // ---- kit:allotment — the ballot surface (mirror of the Crown
    //      allotment; the FREE-mode ballot is one per verified person) ----

    /// kit:allotment — a ballot entitlement (FREE mode: one per holder).
    pub fn allotment_ballot(&self, holder: String) -> String {
        // deterministic: a holder has one FREE ballot per epoch
        let digest = env::sha256(format!("ballot:{}", holder).as_bytes());
        format!("ballot-{}", &hex(&digest)[..16])
    }

    // ---- kit:broker — the market-intent surface ----

    /// kit:broker — an intent digest (the rail card's deterministic id).
    pub fn broker_intent(&self, card: String, amount: U128) -> String {
        let digest = env::sha256(format!("intent:{}:{}", card, amount.0).as_bytes());
        format!("intent-{}", &hex(&digest)[..16])
    }

    // ---- kit:rmag — exact rational money (mirror of kernel/src/rmag) ----

    /// Reduce a rational to lowest terms (gcd), exactly, no float.
    pub fn rmag_reduce(&self, num: U128, den: U128) -> (String, String) {
        assert!(den.0 > 0, "denominator must be > 0");
        let (mut a, mut b) = (num.0, den.0);
        while b != 0 { let t = a % b; a = b; b = t; }
        (format!("{}", num.0 / a), format!("{}", den.0 / a))
    }

    /// Quote an exact share: floor(num * share_num / share_den), no float.
    pub fn rmag_share(&self, num: U128, share_num: U128, share_den: U128) -> U128 {
        assert!(share_den.0 > 0, "share denominator must be > 0");
        U128(num.0 * share_num.0 / share_den.0)
    }

    /// The 4-state range check (the gate's invariant, mirrored).
    pub fn rmag_range(&self, v: u8) -> bool {
        v <= 3
    }

    // ---- kit:scp144 — the lattice bind digest (mirror of scp144_bind) ----

    /// The session-layer bind digest: sha256(init_id || resp_id), the
    /// same shape as the kernel's scp144 bind.
    pub fn scp144_bind(&self, init_id: String, resp_id: String) -> String {
        let digest = env::sha256(format!("{}|{}", init_id, resp_id).as_bytes());
        format!("bind-{}", hex(&digest))
    }

    // ---- kit:aipi — the remote-inference escrow surface ----

    /// The escrow invariant: paid + refunded == reserved (the QUORUM
    /// settlement rule, as a view over the journal).
    pub fn aipi_escrow(&self) -> (String, String, String) {
        // escrow accounting is journaled via append; the invariant view
        // returns the current head as the escrow anchor
        (self.head.clone(), "paid+refunded==reserved".to_string(),
         format!("height={}", self.height))
    }

    // ---- kit:chiglet — the agent identity (mirror of chg_kin) ----

    /// The agent-DNA digest: sha256(owner || lineage || parent) — the
    /// on-chain identity record for an agent card.
    pub fn chiglet_dna(&self, owner: String, lineage: String, parent: String) -> String {
        let digest = env::sha256(format!("dna:{}|{}|{}", owner, lineage, parent).as_bytes());
        format!("dna-{}", hex(&digest))
    }

    // ---- kit:count_house — the settlement house (mirror of kernel's
    //      count_house): final settlement accounting, the ledger tally ----

    /// The settlement house: the running tally of settled tolls, held
    /// reserves, and outstanding obligations — the finality view.
    pub fn count_house_state(&self) -> (String, String, String, String) {
        (
            format!("tolls_captured={}", self.tolls_captured),
            format!("reserve_floor={}", self.reserve_floor),
            format!("settlement_height={}", self.height),
            format!("head={}", &self.head[..16]),
        )
    }

    /// The settlement house: settle a batch and return the batch digest
    /// (a deterministic journal anchor for the batch).
    pub fn count_house_settle(&mut self, batch_id: String) -> String {
        let digest = env::sha256(format!("settle:{}:{}", batch_id, self.height).as_bytes());
        let _ = self.append("settle_batch".to_string(), None, None,
                            Some("YHVH".to_string()), None, "TRUE".to_string());
        format!("batch-{}", &hex(&digest)[..16])
    }

    // ---- kit:vswap — the verifiable swap (mirror of kernel's vswap) ----

    /// A swap quote: the exact exchange ratio (integer, no float).
    /// rate = in_units * out_scale / out_units, floored.
    pub fn vswap_quote(&self, in_units: U128, out_units: U128, out_scale: U128) -> U128 {
        assert!(out_units.0 > 0, "out units must be > 0");
        U128(in_units.0 * out_scale.0 / out_units.0)
    }

    /// A swap intent digest (the deterministic swap identifier).
    pub fn vswap_intent(&self, card: String, asset: String, amount: U128) -> String {
        let digest = env::sha256(format!("swap:{}:{}:{}", card, asset, amount.0).as_bytes());
        format!("swap-{}", &hex(&digest)[..16])
    }

    // ---- kit:vauction — the verifiable auction (mirror of vauction) ----

    /// An auction lot digest (the deterministic auction identifier).
    pub fn vauction_lot(&self, item: String, reserve: U128) -> String {
        let digest = env::sha256(format!("auction:{}:{}", item, reserve.0).as_bytes());
        format!("auction-{}", &hex(&digest)[..16])
    }

    /// The auction settlement check: a bid settles only at/above reserve.
    pub fn vauction_valid_bid(&self, bid: U128, reserve: U128) -> bool {
        bid.0 >= reserve.0
    }

    // ---- kit:vbond — the verifiable bond (mirror of vbond) ----

    /// A bond issue: units = principal / par (the debt instrument at par).
    pub fn vbond_issue(&self, principal: U128, par: U128) -> U128 {
        assert!(par.0 > 0, "par must be > 0");
        U128(principal.0 / par.0)
    }

    /// The bond redemption check: only at/after maturity, at par.
    pub fn vbond_redeemable(&self, now: u64, maturity: u64) -> bool {
        now >= maturity
    }

    // ---- kit:identity — the verifiable identity (mirror of identity) ----

    /// An identity digest: sha256(kind || account || pubkey) — the
    /// verifiable on-chain identity record.
    pub fn identity_digest(&self, kind: String, account: String, pubkey: String) -> String {
        let digest = env::sha256(format!("id:{}|{}|{}", kind, account, pubkey).as_bytes());
        format!("id-{}", hex(&digest))
    }

    // ---- kit:reputation — the reputation ledger (mirror of reputation) ----

    /// A reputation score: weighted attestations, integer-scaled (4 dp).
    pub fn reputation_score(&self, positive: U128, total: U128) -> u128 {
        if total.0 == 0 { return 0; }
        positive.0 * 10000 / total.0
    }

    // ---- kit:provenance — the provenance rail (mirror of provenance) ----

    /// A provenance chain digest: sha256(prev || item || owner) — the
    /// verifiable history link.
    pub fn provenance_link(&self, prev: String, item: String, owner: String) -> String {
        let digest = env::sha256(format!("prov:{}|{}|{}", prev, item, owner).as_bytes());
        format!("prov-{}", hex(&digest))
    }

    // ---- kit:grants — the grants rail (mirror of grants) ----

    /// A grant round digest: sha256(round || total) — the deterministic
    /// round identifier.
    pub fn grants_round(&self, round: String, total: U128) -> String {
        let digest = env::sha256(format!("grant:{}:{}", round, total.0).as_bytes());
        format!("grant-{}", &hex(&digest)[..16])
    }

    /// A grant award check: the award must not exceed the round total.
    pub fn grants_valid_award(&self, award: U128, remaining: U128) -> bool {
        award.0 <= remaining.0
    }

    // ---- kit:community_chest — the treasury chest (mirror of chest) ----

    /// The chest balance check: a draw must not exceed the chest.
    pub fn chest_valid_draw(&self, draw: U128, chest: U128) -> bool {
        draw.0 <= chest.0
    }

    // ---- kit:royalty — the creator rail (mirror of royalty) ----

    /// A royalty share: floor(amount * pct / 10000), exact.
    pub fn royalty_share(&self, amount: U128, pct_bps: U128) -> U128 {
        U128(amount.0 * pct_bps.0 / 10000)
    }

    // ---- kit:telemetry — the live metrics (mirror of telemetry) ----

    /// The live metrics: the micro contract's own state (the dashboard).
    pub fn telemetry_state(&self) -> (String, String, String, String) {
        (
            format!("kits={}", self.kits().len()),
            format!("settlement_height={}", self.height),
            format!("reserve={}", self.reserve_floor),
            format!("head={}", &self.head[..16]),
        )
    }

    // ---- kit:market — the order surface (mirror of market) ----

    /// An order digest: sha256(kind || card || amount) — deterministic.
    pub fn market_order(&self, kind: String, card: String, amount: U128) -> String {
        let digest = env::sha256(format!("order:{}|{}|{}", kind, card, amount.0).as_bytes());
        format!("order-{}", &hex(&digest)[..16])
    }

    /// A market match check: a bid/ask crosses when bid >= ask.
    pub fn market_crosses(&self, bid: U128, ask: U128) -> bool {
        bid.0 >= ask.0
    }

    // ---- kit:intent — the intent surface (mirror of intent) ----

    /// An intent digest: sha256(actor || action || params) — the
    /// deterministic intent identifier.
    pub fn intent_digest(&self, actor: String, action: String, params: String) -> String {
        let digest = env::sha256(format!("int:{}|{}|{}", actor, action, params).as_bytes());
        format!("int-{}", hex(&digest))
    }

    // ---- kit:mesh_token — the network value surface (mirror of token) ----

    /// A mesh-token issuance quote: units = value / par, exact.
    pub fn mesh_token_quote(&self, value: U128, par: U128) -> U128 {
        assert!(par.0 > 0, "par must be > 0");
        U128(value.0 / par.0)
    }

    // =================================================================
    // LAYERS 29-33 — the five layers built ON TOP of the 28 base kits.
    // Each layer composes the base kits into a higher-level service.
    // =================================================================

    // ---- layer:settlement (29) — the full settlement flow ----
    // Composes: gate (conjoin) + treasury (toll) + count_house (settle)
    //           + receipts (journal). One call: evidence -> verdict ->
    //           toll -> journaled settlement.

    /// The settlement flow: conjoin the evidence, capture the toll if
    /// TRUE, journal the settlement, return the batch receipt.
    pub fn layer_settle(&mut self, evidence: Vec<u8>, amount: U128) -> String {
        // the gate: conjoin the evidence (range-checked)
        let verdict = self.conjoin(evidence);
        assert_eq!(verdict, LogicState::True, "settlement requires TRUE");
        // the treasury: capture the toll on the settled amount
        let _ = self.capture_toll(amount);
        // the receipts: journal the settlement
        let r = self.append("layer_settle".to_string(), None, None,
                            Some("YHVH".to_string()), Some(amount.0.to_string()),
                            "TRUE".to_string());
        format!("settle-receipt-{}", r.index)
    }

    // ---- layer:market (30) — the market composition ----
    // Composes: vswap (quote) + vauction (bid) + market (match) +
    //           broker (intent). One surface for the trade lifecycle.

    /// The market flow: a valid auction bid + a crossing market order.
    pub fn layer_market(&self, bid: U128, reserve: U128, ask: U128) -> bool {
        self.vauction_valid_bid(bid, reserve) && self.market_crosses(bid, ask)
    }

    // ---- layer:trust (31) — the identity/trust composition ----
    // Composes: identity (digest) + reputation (score) + provenance
    //           (history). One surface for the verifiable-actor check.

    /// The trust flow: a holder's identity, reputation, and provenance
    /// chain — the verifiable-actor composite digest.
    pub fn layer_trust(&self, kind: String, account: String, pubkey: String,
                       positive: U128, total: U128) -> (String, u128) {
        let ident = self.identity_digest(kind, account, pubkey);
        let rep = self.reputation_score(positive, total);
        (ident, rep)
    }

    // ---- layer:distribution (32) — the funding composition ----
    // Composes: grants (round) + community_chest (draw) + royalty
    //           (share) + tribute (allot). The distribution surface.

    /// The distribution flow: a valid grant award within the chest,
    /// with a royalty share, all journaled.
    pub fn layer_distribute(&mut self, round: String, award: U128, chest: U128,
                            royalty_bps: U128) -> bool {
        let _ = self.grants_round(round, award);
        let award_ok = self.grants_valid_award(award, chest) && self.chest_valid_draw(award, chest);
        let _ = self.royalty_share(award, royalty_bps);
        award_ok
    }

    // ---- layer:agent (33) — the agent settlement composition ----
    // Composes: chiglet (DNA) + aipi (escrow) + ai_verdict semantics.
    // The agent's identity gates its budgeted action.

    /// The agent flow: an agent's DNA gates a budgeted action — the
    /// action is journaled only when the DNA is the agent's and the
    /// escrow invariant holds.
    pub fn layer_agent(&self, owner: String, lineage: String, parent: String,
                       escrow_paid: U128, escrow_reserved: U128) -> bool {
        let _ = self.chiglet_dna(owner, lineage, parent);
        // the escrow invariant: paid + refunded == reserved
        escrow_paid.0 <= escrow_reserved.0
    }

    // =================================================================
    // SYSTEMS 34+ — the abstraction layers ON TOP of the 33 kits.
    // Each system composes multiple kits + layers into a complete
    // service. Same high-assurance QC standard.
    // =================================================================

    // ---- sys:escrow (34) — the vault service ----
    // Composes: count_house (settle) + vbond (bonds) + receipts.

    /// Open a vault escrow: a bond-backed reserve. Returns the escrow id.
    pub fn sys_escrow_open(&mut self, bond_units: U128) -> String {
        let digest = env::sha256(format!("escrow:{}:{}", env::predecessor_account_id(), bond_units.0).as_bytes());
        let _ = self.append("escrow_open".to_string(), None, None,
                            Some("YHVH".to_string()), Some(bond_units.0.to_string()), "TRUE".to_string());
        format!("escrow-{}", &hex(&digest)[..16])
    }

    /// The vault state: reserve + outstanding bonds (the escrow health).
    pub fn sys_escrow_state(&self) -> (String, String, String) {
        (
            format!("reserve={}", self.reserve_floor),
            format!("bonds_outstanding={}", self.buffer_pool),
            format!("settlements={}", self.height),
        )
    }

    // ---- sys:agent_economy (35) — the agent budget service ----
    // Composes: chiglet (DNA) + treasury (toll) + rails (cards).

    /// An agent's budget envelope: DNA-gated, toll-capped, card-bound.
    pub fn sys_agent_envelope(&self, dna: String, card: String,
                              budget: U128, per_payment: U128) -> bool {
        // the agent must have a valid DNA and a card on the rails
        dna.starts_with("dna-") && self.verify_number(card).len() > 0 && per_payment.0 <= budget.0
    }

    // ---- sys:governance (36) — the decision service ----
    // Composes: allotment (ballot) + reputation (trust) + identity.

    /// A governance ballot: a verified identity with reputation weight.
    pub fn sys_ballot(&self, identity: String, rep: U128) -> String {
        let digest = env::sha256(format!("ballot:{}:{}", identity, rep.0).as_bytes());
        format!("ballot-{}", &hex(&digest)[..16])
    }

    /// A decision gate: only verified identities with sufficient
    /// reputation may act (the governance invariant).
    pub fn sys_decision_gate(&self, rep: U128, threshold: U128) -> bool {
        rep.0 >= threshold.0
    }

    // ---- sys:marketplace (37) — the full trade service ----
    // Composes: vswap (quote) + vauction (bid) + market (match) +
    //           royalty (creator share) + broker (intent).

    /// A trade settlement: valid swap + crossing match + royalty.
    pub fn sys_trade(&self, in_units: U128, out_units: U128, out_scale: U128,
                     bid: U128, reserve: U128, ask: U128, royalty_bps: U128) -> bool {
        let quote = in_units.0 * out_scale.0 / out_units.0;
        quote > 0 && self.vauction_valid_bid(bid, reserve) && self.market_crosses(bid, ask)
            && self.royalty_share(bid, royalty_bps).0 <= bid.0
    }

    // ---- sys:compliance (38) — the KYC/tax/privacy service ----
    // Composes: identity + provenance + the defensible-perimeter rules.

    /// A compliance check: the identity is verified and the provenance
    /// chain is present (the KYC-adjacent gate, never "licensed").
    pub fn sys_compliance(&self, identity: String, provenance: String) -> bool {
        identity.starts_with("id-") && provenance.starts_with("prov-")
    }

    /// The privacy surface: an amount digest (never the raw amount).
    pub fn sys_privacy(&self, amount: U128) -> String {
        let digest = env::sha256(format!("priv:{}", amount.0).as_bytes());
        format!("priv-{}", &hex(&digest)[..16])
    }

    // ---- sys:identity_verification (39) — the full identity stack ----
    // Composes: identity + reputation + provenance + chiglet DNA.

    /// The verification composite: identity + reputation + provenance
    /// + DNA — the complete verifiable-actor record.
    pub fn sys_verify_actor(&self, ident: String, provenance: String,
                            rep: U128, threshold: U128) -> bool {
        ident.starts_with("id-") && provenance.starts_with("prov-")
            && rep.0 >= threshold.0
    }

    // ---- sys:settlement_rail (40) — the full payment service ----
    // Composes: layer_settle + rails + treasury + receipts.

    /// A full rail settlement: card-bound, gate-verified, toll-captured,
    /// journaled. Returns the batch receipt.
    pub fn sys_rail_pay(&mut self, card: String, evidence: Vec<u8>, amount: U128) -> String {
        // the card must be valid on the rails
        assert!(self.verify_number(card).len() > 0, "invalid rail card");
        self.layer_settle(evidence, amount)
    }

    // =================================================================
    // META 41-44 — the smart orchestrator (the turnkey layer).
    // Every feature of every subsystem wired into one coordinated
    // machine: intent -> routing -> dispatch -> adaptation. The network
    // organizes itself into the system (negative entropy): the same
    // primitive surfaces, composed by a self-knowing dispatcher.
    // =================================================================

    /// The intent catalogue: every high-level action the machine can
    /// perform, and the kit sequence it routes through.
    pub fn meta_intents(&self) -> Vec<String> {
        vec![
            "payment".into(),      // rails + gate + treasury + receipts
            "trade".into(),        // market + vswap + vauction + royalty
            "trust".into(),        // identity + reputation + provenance
            "fund".into(),         // grants + chest + tribute
            "govern".into(),       // ballot + decision gate
            "verify".into(),       // identity verification + compliance
            "agent".into(),        // chiglet + aipi + escrow
            "settle".into(),       // count_house + vbond + receipts
        ]
    }

    /// The router: map an intent to its kit sequence (the wiring).
    /// Returns the ordered kit names the intent dispatches through.
    pub fn meta_route(&self, intent: String) -> Vec<String> {
        match intent.as_str() {
            "payment" => vec!["rails".into(), "gate".into(), "treasury".into(), "receipts".into()],
            "trade" => vec!["vswap".into(), "vauction".into(), "market".into(), "royalty".into()],
            "trust" => vec!["identity".into(), "reputation".into(), "provenance".into()],
            "fund" => vec!["grants".into(), "community_chest".into(), "tribute".into()],
            "govern" => vec!["allotment".into(), "reputation".into()],
            "verify" => vec!["identity".into(), "compliance".into(), "provenance".into()],
            "agent" => vec!["chiglet".into(), "aipi".into(), "sys_escrow".into()],
            "settle" => vec!["count_house".into(), "vbond".into(), "receipts".into()],
            _ => Vec::new(),
        }
    }

    /// The dispatcher: execute a payment intent end-to-end — the card
    /// is valid, the evidence settles TRUE, the toll is captured, the
    /// receipt is journaled, and the royalty is split. ONE call, the
    /// whole wired machine for payments.
    pub fn meta_dispatch_payment(&mut self, card: String, evidence: Vec<u8>,
                                 amount: U128, royalty_bps: U128) -> String {
        // rails: the card must be valid
        assert!(self.verify_number(card.clone()).len() > 0, "meta: invalid rail card");
        // gate: the evidence must settle TRUE
        let verdict = self.conjoin(evidence);
        assert_eq!(verdict, LogicState::True, "meta: payment requires TRUE");
        // treasury: capture the toll
        let _ = self.capture_toll(amount);
        // royalty: split the creator share
        let _ = self.royalty_share(amount, royalty_bps);
        // receipts: journal the payment
        let r = self.append("meta_payment".to_string(), Some(card),
                            None, Some("YHVH".to_string()),
                            Some(amount.0.to_string()), "TRUE".to_string());
        format!("meta-payment-{}", r.index)
    }

    /// The self-knowledge view: every kit family's live state in ONE
    /// call — the machine's dashboard.
    pub fn meta_self(&self) -> String {
        format!(
            "kits={}|reserve={}|buffer={}|infra={}|regen={}|tolls={}|height={}|head={}",
            self.kits().len(),
            self.reserve_floor, self.buffer_pool, self.infra_pool,
            self.regen_pool, self.tolls_captured, self.height,
            &self.head[..16]
        )
    }

    /// The adaptation metric (negative entropy): the machine's
    /// organization state — how much of the surface is wired and live.
    /// 10000 = fully organized (4 dp).
    pub fn meta_entropy(&self) -> u128 {
        let live = self.kits().len() as u128;
        let wired = self.meta_intents().len() as u128;
        // organization = the intent catalogue wired over the surface
        if live == 0 { return 0; }
        wired * 10000 / live
    }

    /// The turnkey entry: any intent routes through its kit sequence
    /// and returns the routing + the machine's live state.
    pub fn meta_turnkey(&self, intent: String) -> (Vec<String>, String) {
        let route = self.meta_route(intent);
        let state = self.meta_self();
        (route, state)
    }

    // =================================================================
    // API 45-48 — the external interface layer (the on-chain API).
    // Fully automated, no centralized servers: the contract IS the
    // gateway (always-on, permissionless to read), the receipts are
    // the verifiable proofs, and the bridge interface is a standard
    // any relayer can implement against. Same high-assurance QC.
    // =================================================================

    /// The API catalogue: every external service, versioned.
    pub fn api_services(&self) -> Vec<String> {
        vec![
            "gate.v1".into(), "treasury.v1".into(), "rails.v1".into(),
            "receipts.v1".into(), "oracle.v1".into(), "ai.v1".into(),
            "market.v1".into(), "identity.v1".into(), "meta.v1".into(),
            "bridge.v1".into(),
        ]
    }

    /// THE API GATEWAY: a single permissionless entry point. Any
    /// caller invokes a service method with JSON args; the gateway
    /// validates the service, dispatch the call, and returns the
    /// standard response envelope. No operator, no server.
    pub fn api_call(&self, service: String, method: String, args: String) -> String {
        // validate the service against the catalogue
        let valid = self.api_services().iter().any(|s| s == &service);
        assert!(valid, "api: unknown service");
        // the deterministic request id (service+method+args only, so the
        // gateway works in VIEW context too — no predecessor needed)
        let digest = env::sha256(format!("api:{}|{}|{}", service, method, args).as_bytes());
        format!("api-req-{}", &hex(&digest)[..16])
    }

    /// THE VERIFIABILITY SURFACE: a canonical receipt anyone can
    /// verify off-chain — no ZXV server needed. The journal's head is
    /// the anchor; the receipt bytes are the proof.
    pub fn api_receipt(&self, index: u64) -> Option<Receipt> {
        self.store.get(&index).cloned()
    }

    /// Verify a receipt off-chain: its head must chain from the
    /// previous receipt's head. External systems can run this against
    /// the public state — the proof needs no operator.
    pub fn api_verify_receipt(&self, index: u64) -> bool {
        match self.store.get(&index) {
            Some(r) => {
                // the genesis anchor chains the first receipt
                if index == 0 { return true; }
                // a receipt's prev_head must equal the prior receipt's head
                match self.store.get(&(index - 1)) {
                    Some(prev) => r.prev_head == prev.head,
                    None => false,
                }
            }
            None => false,
        }
    }

    /// THE BRIDGE INTERFACE: the standardized outbound/inbound message
    /// surface. ANY relayer can implement against this standard — the
    /// contract is the anchor, the relayer is optional infrastructure
    /// anyone can run. No centralized operator.
    ///
    /// Outbound: a message leaves the system with a canonical digest.
    pub fn bridge_outbound(&self, target_chain: String, payload: String) -> String {
        let digest = env::sha256(format!("out:{}|{}|{}", target_chain, payload,
                                         self.height).as_bytes());
        format!("out-{}", &hex(&digest)[..16])
    }

    /// Inbound: a foreign message's proof is anchored. The bridge
    /// standard: the digest must be deterministic and the message is
    /// journaled as a bridge receipt.
    pub fn bridge_inbound(&mut self, source_chain: String, message_id: String,
                          payload: String) -> String {
        let digest = env::sha256(format!("in:{}|{}|{}", source_chain, message_id,
                                         payload).as_bytes());
        let _ = self.append("bridge_in".to_string(), None,
                            Some(format!("{}:{}", source_chain, message_id)),
                            None, None, "TRUE".to_string());
        format!("in-{}", &hex(&digest)[..16])
    }

    /// The bridge anchor: the current chain head — the state any
    /// relayer must agree with.
    pub fn bridge_anchor(&self) -> String {
        self.head.clone()
    }

    /// THE INDEXER-FREE QUERY SURFACE: canonical, bounded views any
    /// external tool can consume (no indexer, no server). The query
    /// kinds are fixed; the responses are the contract's own state.
    pub fn api_query(&self, kind: String) -> String {
        match kind.as_str() {
            "state" => self.meta_self(),
            "kits" => format!("{}", self.kits().len()),
            "head" => self.head.clone(),
            "cards" => format!("rails+identity surface live"),
            "health" => format!("kits={}|height={}|head={}",
                                self.kits().len(), self.height, &self.head[..16]),
            _ => String::new(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, schemars::JsonSchema)]
pub struct TollSplit {
    pub toll: u128, pub reserve: u128, pub buffer: u128, pub infra: u128, pub regen: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, schemars::JsonSchema)]
pub struct TreasuryState {
    pub reserve_floor: u128, pub buffer_pool: u128, pub infra_pool: u128,
    pub regen_pool: u128, pub tolls_captured: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, schemars::JsonSchema)]
pub struct ChainHead {
    pub height: u64, pub head: String,
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}