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

//! Autogenerated weights for pallet_committee
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

/// Weights for pallet_committee using the Substrate node and recommended hardware.
pub struct SubstrateWeight;
impl pallet_committee::WeightInfo for SubstrateWeight {
    // Storage: Instance1Committee VoteThreshold (r:0 w:1)
    // Proof Skipped: Instance1Committee VoteThreshold (max_values: Some(1), max_size: None, mode: Measured)
    fn set_vote_threshold() -> Weight {
        // Minimum execution time: 10_135 nanoseconds.
        Weight::from_ref_time(11_227_000).saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Instance1Committee Members (r:1 w:0)
    // Proof Skipped: Instance1Committee Members (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Instance1Committee ReleaseCoordinator (r:0 w:1)
    // Proof Skipped: Instance1Committee ReleaseCoordinator (max_values: Some(1), max_size: None, mode: Measured)
    fn set_release_coordinator() -> Weight {
        // Minimum execution time: 48_141 nanoseconds.
        Weight::from_ref_time(53_229_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Instance1Committee ExpiresAfter (r:0 w:1)
    // Proof Skipped: Instance1Committee ExpiresAfter (max_values: Some(1), max_size: None, mode: Measured)
    fn set_expires_after() -> Weight {
        // Minimum execution time: 9_975 nanoseconds.
        Weight::from_ref_time(11_197_000).saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Instance1Committee Voting (r:1 w:1)
    // Proof Skipped: Instance1Committee Voting (max_values: None, max_size: None, mode: Measured)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance1Committee Members (r:1 w:0)
    // Proof Skipped: Instance1Committee Members (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Instance1Committee ProposalCount (r:1 w:1)
    // Proof Skipped: Instance1Committee ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Instance1Committee ProposalOf (r:1 w:1)
    // Proof Skipped: Instance1Committee ProposalOf (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance1Committee Proposals (r:1 w:1)
    // Proof Skipped: Instance1Committee Proposals (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Instance1Committee ExpiresAfter (r:1 w:0)
    // Proof Skipped: Instance1Committee ExpiresAfter (max_values: Some(1), max_size: None, mode: Measured)
    fn vote_or_propose_new_proposal() -> Weight {
        // Minimum execution time: 149_715 nanoseconds.
        Weight::from_ref_time(156_454_000)
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().writes(4))
    }
    // Storage: Instance1Committee Voting (r:1 w:1)
    // Proof Skipped: Instance1Committee Voting (max_values: None, max_size: None, mode: Measured)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance1Committee Members (r:1 w:0)
    // Proof Skipped: Instance1Committee Members (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Instance1Committee VoteThreshold (r:1 w:0)
    // Proof Skipped: Instance1Committee VoteThreshold (max_values: Some(1), max_size: None, mode: Measured)
    fn vote_or_propose_existing_proposal() -> Weight {
        // Minimum execution time: 119_660 nanoseconds.
        Weight::from_ref_time(135_473_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance1Committee Members (r:1 w:0)
    // Proof Skipped: Instance1Committee Members (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Instance1Committee Voting (r:1 w:1)
    // Proof Skipped: Instance1Committee Voting (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance1Committee VoteThreshold (r:1 w:0)
    // Proof Skipped: Instance1Committee VoteThreshold (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Instance1Committee ProposalOf (r:1 w:1)
    // Proof Skipped: Instance1Committee ProposalOf (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance1Committee Proposals (r:1 w:1)
    // Proof Skipped: Instance1Committee Proposals (max_values: Some(1), max_size: None, mode: Measured)
    fn vote_aye() -> Weight {
        // Minimum execution time: 266_349 nanoseconds.
        Weight::from_ref_time(275_382_000)
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance1Committee Members (r:1 w:0)
    // Proof Skipped: Instance1Committee Members (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Instance1Committee Voting (r:1 w:1)
    // Proof Skipped: Instance1Committee Voting (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance1Committee VoteThreshold (r:1 w:0)
    // Proof Skipped: Instance1Committee VoteThreshold (max_values: Some(1), max_size: None, mode: Measured)
    fn vote_nay() -> Weight {
        // Minimum execution time: 107_030 nanoseconds.
        Weight::from_ref_time(109_584_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(1))
    }
}
