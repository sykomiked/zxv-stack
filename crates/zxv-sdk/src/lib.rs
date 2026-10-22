//! zxv-sdk — the ZXV developer SDK.
//!
//! Drop-in integration for the paraconsistent settlement layer on NEAR
//! mainnet: 18 registered subsystems, discoverable through one registry
//! call. This crate gives you the four-valued gate, the rail card
//! networks (Dragon/Phoenix/Thunderbird), the receipt chain, the
//! dispute-tolerant oracle, and the work-based treasury — typed and
//! verified, with zero runtime dependencies (the `near` feature adds
//! the JSON-RPC client).
//!
//! ```
//! use zxv_sdk::{LogicState, Gate, RailNetwork, rail_number_valid};
//!
//! // the settlement invariant: only TRUE settles
//! let verdict = Gate::conjoin(LogicState::True, LogicState::False);
//! assert!(!verdict.is_settleable());
//!
//! // a rail number verifies against its network
//! assert!(rail_number_valid("88090016741821469134", RailNetwork::Dragon));
//! ```
//!
//! Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
//! SPDX-License-Identifier: Apache-2.0
#![no_std]

pub mod check;
pub mod gate;

pub use check::{rail_network, rail_number_valid, RailNetwork};
pub use gate::{Gate, LogicState, settle_or_hold};

/// The mainnet discovery registry (the boot sector).
pub const WORLD_REGISTRY: &str =
    "7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a";

/// The paraconsistent gate (the kernel).
pub const GATE: &str =
    "6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8";

/// The rail card registry (Dragon/Phoenix/Thunderbird).
pub const RAILS_REGISTRY: &str = "e4111bdb1e38a5c7761b5ad5322af4569389b4ac7e3964b2";

/// The hash-chained settlement journal.
pub const RAILS_RECEIPTS: &str = "882728f5a6275951f33a06d8d8b3d19f1ec1e2a20f93009b";

/// The dispute-tolerant price harmonizer.
pub const PARACONSISTENT_ORACLE: &str =
    "ea85f5381ceff17553fa3093a2d67a0e6cc4853abd7c19487ed7a33391acc2f7";

/// The settlement router (receipts on TRUE).
pub const SETTLEMENT_ROUTER: &str =
    "8bf177019402d3d843b361a04d3ed7e21de637bf5d252956a15208f7950effae";

/// The settleable-AI adapter.
pub const AI_VERDICT: &str =
    "781fc03d4dfb9b29d88df561157b11db6dc1e02368eae6deef7998d188ff966e";

/// The work-based monetary policy engine.
pub const ZXV_CORE: &str =
    "f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf";

/// The safe evidence-combination gate (the kjoin-defect fix).
pub const VERDICT_GUARD: &str = "88e17c606d29c45e";

/// The wYHVH -> YHVH redeem path.
pub const YHV_REDEEMER: &str = "97db094fae2ba0dd";

/// The wYHVH wrapper.
pub const YHV_WRAPPER: &str = "1a051ac2465c9a04e532fe420e2efbd8cc12f9a1d9f2a350470782fd3255ecf1";