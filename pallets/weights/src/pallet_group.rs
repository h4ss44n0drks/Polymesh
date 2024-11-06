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

//! Autogenerated weights for pallet_group
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

/// Weights for pallet_group using the Substrate node and recommended hardware.
pub struct SubstrateWeight;
impl pallet_group::WeightInfo for SubstrateWeight {
    // Storage: Instance2Group ActiveMembersLimit (r:1 w:1)
    // Proof Skipped: Instance2Group ActiveMembersLimit (max_values: Some(1), max_size: None, mode: Measured)
    fn set_active_members_limit() -> Weight {
        // Minimum execution time: 14_232 nanoseconds.
        Weight::from_ref_time(15_674_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Instance2Group ActiveMembers (r:1 w:1)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembersLimit (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembersLimit (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity Claims (r:1 w:1)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    fn add_member() -> Weight {
        // Minimum execution time: 102_413 nanoseconds.
        Weight::from_ref_time(108_412_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Instance2Group InactiveMembers (r:1 w:1)
    // Proof Skipped: Instance2Group InactiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    fn remove_member() -> Weight {
        // Minimum execution time: 78_647 nanoseconds.
        Weight::from_ref_time(80_401_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Instance2Group ActiveMembers (r:1 w:1)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:1 w:1)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Instance2Group InactiveMembers (r:1 w:1)
    // Proof Skipped: Instance2Group InactiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    fn disable_member() -> Weight {
        // Minimum execution time: 107_641 nanoseconds.
        Weight::from_ref_time(113_250_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Instance2Group ActiveMembers (r:1 w:1)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity Claims (r:2 w:2)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    fn swap_member() -> Weight {
        // Minimum execution time: 125_188 nanoseconds.
        Weight::from_ref_time(136_303_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Instance2Group ActiveMembersLimit (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembersLimit (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:1)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity Claims (r:1001 w:1001)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    /// The range of component `m` is `[1, 1000]`.
    fn reset_members(m: u32) -> Weight {
        // Minimum execution time: 47_890 nanoseconds.
        Weight::from_ref_time(53_350_000)
            // Standard Error: 37_131
            .saturating_add(Weight::from_ref_time(14_607_210).saturating_mul(m.into()))
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(m.into())))
            .saturating_add(DbWeight::get().writes(2))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(m.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: Identity DidRecords (r:1 w:0)
    // Proof Skipped: Identity DidRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:1)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:1 w:1)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    fn abdicate_membership() -> Weight {
        // Minimum execution time: 108_652 nanoseconds.
        Weight::from_ref_time(120_459_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(2))
    }
}
