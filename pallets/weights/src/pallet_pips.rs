// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_pips
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-11-03, STEPS: `100`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 512
//! HOSTNAME: `ubuntu-8gb-nbg1-1-bench2`, CPU: `AMD EPYC-Milan Processor`

// Executed Command:
// ./polymesh
// benchmark
// pallet
// -s
// 100
// -r
// 5
// -p=*
// -e=*
// --heap-pages
// 4096
// --db-cache
// 512
// --execution
// wasm
// --wasm-execution
// compiled
// --output
// ./Polymesh/pallets/weights/src/
// --template
// ./Polymesh/.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use polymesh_runtime_common::{RocksDbWeight as DbWeight, Weight};

/// Weights for pallet_pips using the Substrate node and recommended hardware.
pub struct SubstrateWeight;
impl pallet_pips::WeightInfo for SubstrateWeight {
    // Storage: Pips PruneHistoricalPips (r:1 w:1)
    // Proof Skipped: Pips PruneHistoricalPips (max_values: Some(1), max_size: None, mode: Measured)
    fn set_prune_historical_pips() -> Weight {
        // Minimum execution time: 13_651 nanoseconds.
        Weight::from_ref_time(15_423_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Pips MinimumProposalDeposit (r:1 w:1)
    // Proof Skipped: Pips MinimumProposalDeposit (max_values: Some(1), max_size: None, mode: Measured)
    fn set_min_proposal_deposit() -> Weight {
        // Minimum execution time: 15_974 nanoseconds.
        Weight::from_ref_time(18_488_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Pips DefaultEnactmentPeriod (r:1 w:1)
    // Proof Skipped: Pips DefaultEnactmentPeriod (max_values: Some(1), max_size: None, mode: Measured)
    fn set_default_enactment_period() -> Weight {
        // Minimum execution time: 16_775 nanoseconds.
        Weight::from_ref_time(19_199_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Pips PendingPipExpiry (r:1 w:1)
    // Proof Skipped: Pips PendingPipExpiry (max_values: Some(1), max_size: None, mode: Measured)
    fn set_pending_pip_expiry() -> Weight {
        // Minimum execution time: 17_005 nanoseconds.
        Weight::from_ref_time(19_139_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Pips MaxPipSkipCount (r:1 w:1)
    // Proof Skipped: Pips MaxPipSkipCount (max_values: Some(1), max_size: None, mode: Measured)
    fn set_max_pip_skip_count() -> Weight {
        // Minimum execution time: 13_270 nanoseconds.
        Weight::from_ref_time(13_480_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Pips ActivePipLimit (r:1 w:1)
    // Proof Skipped: Pips ActivePipLimit (max_values: Some(1), max_size: None, mode: Measured)
    fn set_active_pip_limit() -> Weight {
        // Minimum execution time: 14_882 nanoseconds.
        Weight::from_ref_time(15_773_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips PipIdSequence (r:1 w:1)
    // Proof Skipped: Pips PipIdSequence (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips ActivePipLimit (r:1 w:0)
    // Proof Skipped: Pips ActivePipLimit (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips ActivePipCount (r:1 w:1)
    // Proof Skipped: Pips ActivePipCount (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips MinimumProposalDeposit (r:1 w:0)
    // Proof Skipped: Pips MinimumProposalDeposit (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Balances Locks (r:1 w:1)
    // Proof Skipped: Balances Locks (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:1)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Proof Skipped: ProtocolFee Coefficient (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: ProtocolFee BaseFees (r:1 w:0)
    // Proof Skipped: ProtocolFee BaseFees (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips PendingPipExpiry (r:1 w:0)
    // Proof Skipped: Pips PendingPipExpiry (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips ProposalResult (r:1 w:1)
    // Proof Skipped: Pips ProposalResult (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalVotes (r:1 w:1)
    // Proof Skipped: Pips ProposalVotes (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips LiveQueue (r:1 w:1)
    // Proof Skipped: Pips LiveQueue (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips ProposalMetadata (r:0 w:1)
    // Proof Skipped: Pips ProposalMetadata (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips Deposits (r:0 w:1)
    // Proof Skipped: Pips Deposits (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips Proposals (r:0 w:1)
    // Proof Skipped: Pips Proposals (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalStates (r:0 w:1)
    // Proof Skipped: Pips ProposalStates (max_values: None, max_size: None, mode: Measured)
    fn propose_from_community() -> Weight {
        // Minimum execution time: 109_675 nanoseconds.
        Weight::from_ref_time(112_487_000)
            .saturating_add(DbWeight::get().reads(13))
            .saturating_add(DbWeight::get().writes(11))
    }
    // Storage: Pips PipIdSequence (r:1 w:1)
    // Proof Skipped: Pips PipIdSequence (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Proof Skipped: ProtocolFee Coefficient (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: ProtocolFee BaseFees (r:1 w:0)
    // Proof Skipped: ProtocolFee BaseFees (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips PendingPipExpiry (r:1 w:0)
    // Proof Skipped: Pips PendingPipExpiry (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips ActivePipCount (r:1 w:1)
    // Proof Skipped: Pips ActivePipCount (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips CommitteePips (r:1 w:1)
    // Proof Skipped: Pips CommitteePips (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips ProposalMetadata (r:0 w:1)
    // Proof Skipped: Pips ProposalMetadata (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips Proposals (r:0 w:1)
    // Proof Skipped: Pips Proposals (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalStates (r:0 w:1)
    // Proof Skipped: Pips ProposalStates (max_values: None, max_size: None, mode: Measured)
    fn propose_from_committee() -> Weight {
        // Minimum execution time: 53_711 nanoseconds.
        Weight::from_ref_time(55_283_000)
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().writes(6))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips Proposals (r:1 w:0)
    // Proof Skipped: Pips Proposals (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalStates (r:1 w:0)
    // Proof Skipped: Pips ProposalStates (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalResult (r:1 w:1)
    // Proof Skipped: Pips ProposalResult (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips Deposits (r:1 w:1)
    // Proof Skipped: Pips Deposits (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalVotes (r:1 w:1)
    // Proof Skipped: Pips ProposalVotes (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips LiveQueue (r:1 w:1)
    // Proof Skipped: Pips LiveQueue (max_values: Some(1), max_size: None, mode: Measured)
    fn vote() -> Weight {
        // Minimum execution time: 97_947 nanoseconds.
        Weight::from_ref_time(105_979_000)
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().writes(4))
    }
    // Storage: Pips ProposalStates (r:1 w:1)
    // Proof Skipped: Pips ProposalStates (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips Proposals (r:1 w:0)
    // Proof Skipped: Pips Proposals (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips DefaultEnactmentPeriod (r:1 w:0)
    // Proof Skipped: Pips DefaultEnactmentPeriod (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Proof: Scheduler Agenda (max_values: None, max_size: Some(10463), added: 12938, mode: MaxEncodedLen)
    // Storage: Pips PipToSchedule (r:0 w:1)
    // Proof Skipped: Pips PipToSchedule (max_values: None, max_size: None, mode: Measured)
    fn approve_committee_proposal() -> Weight {
        // Minimum execution time: 52_168 nanoseconds.
        Weight::from_ref_time(59_849_000)
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().writes(4))
    }
    // Storage: Pips ProposalStates (r:1 w:1)
    // Proof Skipped: Pips ProposalStates (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips LiveQueue (r:1 w:1)
    // Proof Skipped: Pips LiveQueue (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips SnapshotMeta (r:1 w:0)
    // Proof Skipped: Pips SnapshotMeta (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips ActivePipCount (r:1 w:1)
    // Proof Skipped: Pips ActivePipCount (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips PruneHistoricalPips (r:1 w:0)
    // Proof Skipped: Pips PruneHistoricalPips (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips Deposits (r:2 w:1)
    // Proof Skipped: Pips Deposits (max_values: None, max_size: None, mode: Measured)
    // Storage: Balances Locks (r:1 w:1)
    // Proof Skipped: Balances Locks (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:1)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Pips ProposalVotes (r:1 w:1)
    // Proof Skipped: Pips ProposalVotes (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips Proposals (r:1 w:1)
    // Proof Skipped: Pips Proposals (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalMetadata (r:0 w:1)
    // Proof Skipped: Pips ProposalMetadata (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips PipSkipCount (r:0 w:1)
    // Proof Skipped: Pips PipSkipCount (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalResult (r:0 w:1)
    // Proof Skipped: Pips ProposalResult (max_values: None, max_size: None, mode: Measured)
    fn reject_proposal() -> Weight {
        // Minimum execution time: 95_354 nanoseconds.
        Weight::from_ref_time(100_941_000)
            .saturating_add(DbWeight::get().reads(11))
            .saturating_add(DbWeight::get().writes(11))
    }
    // Storage: Pips ProposalStates (r:1 w:1)
    // Proof Skipped: Pips ProposalStates (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips Deposits (r:1 w:0)
    // Proof Skipped: Pips Deposits (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalVotes (r:1 w:1)
    // Proof Skipped: Pips ProposalVotes (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips Proposals (r:1 w:1)
    // Proof Skipped: Pips Proposals (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalMetadata (r:0 w:1)
    // Proof Skipped: Pips ProposalMetadata (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips PipSkipCount (r:0 w:1)
    // Proof Skipped: Pips PipSkipCount (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalResult (r:0 w:1)
    // Proof Skipped: Pips ProposalResult (max_values: None, max_size: None, mode: Measured)
    fn prune_proposal() -> Weight {
        // Minimum execution time: 51_487 nanoseconds.
        Weight::from_ref_time(53_941_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(6))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance1Committee ReleaseCoordinator (r:1 w:0)
    // Proof Skipped: Instance1Committee ReleaseCoordinator (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips ProposalStates (r:1 w:0)
    // Proof Skipped: Pips ProposalStates (max_values: None, max_size: None, mode: Measured)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
    // Storage: Scheduler Agenda (r:2 w:2)
    // Proof: Scheduler Agenda (max_values: None, max_size: Some(10463), added: 12938, mode: MaxEncodedLen)
    // Storage: Pips PipToSchedule (r:0 w:1)
    // Proof Skipped: Pips PipToSchedule (max_values: None, max_size: None, mode: Measured)
    fn reschedule_execution() -> Weight {
        // Minimum execution time: 66_459 nanoseconds.
        Weight::from_ref_time(69_594_000)
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().writes(4))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance1Committee Members (r:1 w:0)
    // Proof Skipped: Instance1Committee Members (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips SnapshotMeta (r:1 w:1)
    // Proof Skipped: Pips SnapshotMeta (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips SnapshotQueue (r:0 w:1)
    // Proof Skipped: Pips SnapshotQueue (max_values: Some(1), max_size: None, mode: Measured)
    fn clear_snapshot() -> Weight {
        // Minimum execution time: 33_560 nanoseconds.
        Weight::from_ref_time(36_505_000)
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance1Committee Members (r:1 w:0)
    // Proof Skipped: Instance1Committee Members (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips SnapshotIdSequence (r:1 w:1)
    // Proof Skipped: Pips SnapshotIdSequence (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips LiveQueue (r:1 w:0)
    // Proof Skipped: Pips LiveQueue (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips SnapshotQueue (r:0 w:1)
    // Proof Skipped: Pips SnapshotQueue (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips SnapshotMeta (r:0 w:1)
    // Proof Skipped: Pips SnapshotMeta (max_values: Some(1), max_size: None, mode: Measured)
    fn snapshot() -> Weight {
        // Minimum execution time: 97_827 nanoseconds.
        Weight::from_ref_time(101_082_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Pips MaxPipSkipCount (r:1 w:0)
    // Proof Skipped: Pips MaxPipSkipCount (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips SnapshotQueue (r:1 w:1)
    // Proof Skipped: Pips SnapshotQueue (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips PipSkipCount (r:33 w:33)
    // Proof Skipped: Pips PipSkipCount (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips LiveQueue (r:1 w:1)
    // Proof Skipped: Pips LiveQueue (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips ProposalStates (r:66 w:66)
    // Proof Skipped: Pips ProposalStates (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ActivePipCount (r:1 w:1)
    // Proof Skipped: Pips ActivePipCount (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips PruneHistoricalPips (r:1 w:0)
    // Proof Skipped: Pips PruneHistoricalPips (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips Deposits (r:13233 w:13200)
    // Proof Skipped: Pips Deposits (max_values: None, max_size: None, mode: Measured)
    // Storage: Balances Locks (r:400 w:400)
    // Proof Skipped: Balances Locks (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:400 w:400)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Pips DefaultEnactmentPeriod (r:1 w:0)
    // Proof Skipped: Pips DefaultEnactmentPeriod (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Scheduler Lookup (r:33 w:33)
    // Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Proof: Scheduler Agenda (max_values: None, max_size: Some(10463), added: 12938, mode: MaxEncodedLen)
    // Storage: Pips SnapshotMeta (r:1 w:0)
    // Proof Skipped: Pips SnapshotMeta (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips PipToSchedule (r:0 w:33)
    // Proof Skipped: Pips PipToSchedule (max_values: None, max_size: None, mode: Measured)
    /// The range of component `a` is `[0, 33]`.
    /// The range of component `r` is `[0, 33]`.
    /// The range of component `s` is `[0, 33]`.
    fn enact_snapshot_results(a: u32, r: u32, s: u32) -> Weight {
        // Minimum execution time: 1_649_642 nanoseconds.
        Weight::from_ref_time(1_657_344_000)
            // Standard Error: 5_391_730
            .saturating_add(Weight::from_ref_time(54_095_498).saturating_mul(a.into()))
            // Standard Error: 5_391_730
            .saturating_add(Weight::from_ref_time(7_306_498_155).saturating_mul(r.into()))
            // Standard Error: 5_391_730
            .saturating_add(Weight::from_ref_time(13_777_923).saturating_mul(s.into()))
            .saturating_add(DbWeight::get().reads(685))
            .saturating_add(DbWeight::get().reads((3_u64).saturating_mul(a.into())))
            .saturating_add(DbWeight::get().reads((405_u64).saturating_mul(r.into())))
            .saturating_add(DbWeight::get().reads((2_u64).saturating_mul(s.into())))
            .saturating_add(DbWeight::get().writes(681))
            .saturating_add(DbWeight::get().writes((4_u64).saturating_mul(a.into())))
            .saturating_add(DbWeight::get().writes((404_u64).saturating_mul(r.into())))
            .saturating_add(DbWeight::get().writes((2_u64).saturating_mul(s.into())))
    }
    // Storage: Pips Proposals (r:1 w:1)
    // Proof Skipped: Pips Proposals (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalStates (r:1 w:1)
    // Proof Skipped: Pips ProposalStates (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ActivePipCount (r:1 w:1)
    // Proof Skipped: Pips ActivePipCount (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips PruneHistoricalPips (r:1 w:0)
    // Proof Skipped: Pips PruneHistoricalPips (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips Deposits (r:401 w:400)
    // Proof Skipped: Pips Deposits (max_values: None, max_size: None, mode: Measured)
    // Storage: Balances Locks (r:400 w:400)
    // Proof Skipped: Balances Locks (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:400 w:400)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Pips ProposalVotes (r:400 w:400)
    // Proof Skipped: Pips ProposalVotes (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalMetadata (r:0 w:1)
    // Proof Skipped: Pips ProposalMetadata (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips PipSkipCount (r:0 w:1)
    // Proof Skipped: Pips PipSkipCount (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips PipToSchedule (r:0 w:1)
    // Proof Skipped: Pips PipToSchedule (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalResult (r:0 w:1)
    // Proof Skipped: Pips ProposalResult (max_values: None, max_size: None, mode: Measured)
    fn execute_scheduled_pip() -> Weight {
        // Minimum execution time: 11_167_440 nanoseconds.
        Weight::from_ref_time(11_456_713_000)
            .saturating_add(DbWeight::get().reads(1605))
            .saturating_add(DbWeight::get().writes(1607))
    }
    // Storage: Pips ProposalStates (r:1 w:1)
    // Proof Skipped: Pips ProposalStates (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips LiveQueue (r:1 w:1)
    // Proof Skipped: Pips LiveQueue (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips SnapshotMeta (r:1 w:0)
    // Proof Skipped: Pips SnapshotMeta (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips SnapshotQueue (r:1 w:1)
    // Proof Skipped: Pips SnapshotQueue (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips ActivePipCount (r:1 w:1)
    // Proof Skipped: Pips ActivePipCount (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips PruneHistoricalPips (r:1 w:0)
    // Proof Skipped: Pips PruneHistoricalPips (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Pips Deposits (r:401 w:400)
    // Proof Skipped: Pips Deposits (max_values: None, max_size: None, mode: Measured)
    // Storage: Balances Locks (r:400 w:400)
    // Proof Skipped: Balances Locks (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:400 w:400)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Pips ProposalVotes (r:400 w:400)
    // Proof Skipped: Pips ProposalVotes (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips Proposals (r:1 w:1)
    // Proof Skipped: Pips Proposals (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalMetadata (r:0 w:1)
    // Proof Skipped: Pips ProposalMetadata (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips PipSkipCount (r:0 w:1)
    // Proof Skipped: Pips PipSkipCount (max_values: None, max_size: None, mode: Measured)
    // Storage: Pips ProposalResult (r:0 w:1)
    // Proof Skipped: Pips ProposalResult (max_values: None, max_size: None, mode: Measured)
    fn expire_scheduled_pip() -> Weight {
        // Minimum execution time: 11_294_591 nanoseconds.
        Weight::from_ref_time(11_411_686_000)
            .saturating_add(DbWeight::get().reads(1608))
            .saturating_add(DbWeight::get().writes(1608))
    }
}
