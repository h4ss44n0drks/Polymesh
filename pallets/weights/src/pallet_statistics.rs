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

//! Autogenerated weights for pallet_statistics
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

/// Weights for pallet_statistics using the Substrate node and recommended hardware.
pub struct SubstrateWeight;
impl pallet_statistics::WeightInfo for SubstrateWeight {
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Proof Skipped: ExternalAgents GroupOfAgent (max_values: None, max_size: None, mode: Measured)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Proof Skipped: Permissions CurrentPalletName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Proof Skipped: Permissions CurrentDispatchableName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Statistics AssetTransferCompliances (r:1 w:0)
    // Proof Skipped: Statistics AssetTransferCompliances (max_values: None, max_size: None, mode: Measured)
    // Storage: Statistics ActiveAssetStats (r:1 w:1)
    // Proof Skipped: Statistics ActiveAssetStats (max_values: None, max_size: None, mode: Measured)
    /// The range of component `i` is `[1, 9]`.
    fn set_active_asset_stats(i: u32) -> Weight {
        // Minimum execution time: 39_780 nanoseconds.
        Weight::from_ref_time(44_031_265)
            // Standard Error: 77_955
            .saturating_add(Weight::from_ref_time(330_085).saturating_mul(i.into()))
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Proof Skipped: ExternalAgents GroupOfAgent (max_values: None, max_size: None, mode: Measured)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Proof Skipped: Permissions CurrentPalletName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Proof Skipped: Permissions CurrentDispatchableName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Statistics ActiveAssetStats (r:1 w:0)
    // Proof Skipped: Statistics ActiveAssetStats (max_values: None, max_size: None, mode: Measured)
    // Storage: Statistics AssetStats (r:0 w:250)
    // Proof Skipped: Statistics AssetStats (max_values: None, max_size: None, mode: Measured)
    /// The range of component `i` is `[1, 250]`.
    fn batch_update_asset_stats(i: u32) -> Weight {
        // Minimum execution time: 42_724 nanoseconds.
        Weight::from_ref_time(39_205_484)
            // Standard Error: 10_917
            .saturating_add(Weight::from_ref_time(3_180_085).saturating_mul(i.into()))
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(i.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Proof Skipped: ExternalAgents GroupOfAgent (max_values: None, max_size: None, mode: Measured)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Proof Skipped: Permissions CurrentPalletName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Proof Skipped: Permissions CurrentDispatchableName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Statistics ActiveAssetStats (r:1 w:0)
    // Proof Skipped: Statistics ActiveAssetStats (max_values: None, max_size: None, mode: Measured)
    // Storage: Statistics AssetTransferCompliances (r:1 w:1)
    // Proof Skipped: Statistics AssetTransferCompliances (max_values: None, max_size: None, mode: Measured)
    /// The range of component `i` is `[1, 3]`.
    fn set_asset_transfer_compliance(i: u32) -> Weight {
        // Minimum execution time: 42_233 nanoseconds.
        Weight::from_ref_time(41_799_979)
            // Standard Error: 257_984
            .saturating_add(Weight::from_ref_time(4_691_354).saturating_mul(i.into()))
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Proof Skipped: ExternalAgents GroupOfAgent (max_values: None, max_size: None, mode: Measured)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Proof Skipped: Permissions CurrentPalletName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Proof Skipped: Permissions CurrentDispatchableName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Statistics TransferConditionExemptEntities (r:0 w:1000)
    // Proof Skipped: Statistics TransferConditionExemptEntities (max_values: None, max_size: None, mode: Measured)
    /// The range of component `i` is `[0, 1000]`.
    fn set_entities_exempt(i: u32) -> Weight {
        // Minimum execution time: 28_813 nanoseconds.
        Weight::from_ref_time(28_545_771)
            // Standard Error: 5_396
            .saturating_add(Weight::from_ref_time(3_090_924).saturating_mul(i.into()))
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(i.into())))
    }
    // Storage: Statistics AssetStats (r:1 w:0)
    // Proof Skipped: Statistics AssetStats (max_values: None, max_size: None, mode: Measured)
    /// The range of component `a` is `[0, 1]`.
    fn max_investor_count_restriction(a: u32) -> Weight {
        // Minimum execution time: 541 nanoseconds.
        Weight::from_ref_time(805_566)
            // Standard Error: 43_814
            .saturating_add(Weight::from_ref_time(7_416_767).saturating_mul(a.into()))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(a.into())))
    }
    fn max_investor_ownership_restriction() -> Weight {
        // Minimum execution time: 861 nanoseconds.
        Weight::from_ref_time(1_092_000)
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    /// The range of component `c` is `[0, 1]`.
    fn claim_count_restriction_no_stats(c: u32) -> Weight {
        // Minimum execution time: 571 nanoseconds.
        Weight::from_ref_time(912_176)
            // Standard Error: 229_762
            .saturating_add(Weight::from_ref_time(18_293_823).saturating_mul(c.into()))
            .saturating_add(DbWeight::get().reads((3_u64).saturating_mul(c.into())))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: Statistics AssetStats (r:1 w:0)
    // Proof Skipped: Statistics AssetStats (max_values: None, max_size: None, mode: Measured)
    fn claim_count_restriction_with_stats() -> Weight {
        // Minimum execution time: 21_162 nanoseconds.
        Weight::from_ref_time(22_164_000).saturating_add(DbWeight::get().reads(4))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: Statistics AssetStats (r:1 w:0)
    // Proof Skipped: Statistics AssetStats (max_values: None, max_size: None, mode: Measured)
    /// The range of component `a` is `[0, 1]`.
    fn claim_ownership_restriction(a: u32) -> Weight {
        // Minimum execution time: 14_992 nanoseconds.
        Weight::from_ref_time(16_782_771)
            // Standard Error: 400_993
            .saturating_add(Weight::from_ref_time(7_463_562).saturating_mul(a.into()))
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(a.into())))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: Statistics AssetStats (r:2 w:2)
    // Proof Skipped: Statistics AssetStats (max_values: None, max_size: None, mode: Measured)
    /// The range of component `a` is `[0, 2]`.
    fn update_asset_count_stats(a: u32) -> Weight {
        // Minimum execution time: 15_353 nanoseconds.
        Weight::from_ref_time(16_605_847)
            // Standard Error: 101_131
            .saturating_add(Weight::from_ref_time(6_846_034).saturating_mul(a.into()))
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(a.into())))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(a.into())))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: Statistics AssetStats (r:2 w:2)
    // Proof Skipped: Statistics AssetStats (max_values: None, max_size: None, mode: Measured)
    /// The range of component `a` is `[0, 2]`.
    fn update_asset_balance_stats(a: u32) -> Weight {
        // Minimum execution time: 15_252 nanoseconds.
        Weight::from_ref_time(16_650_158)
            // Standard Error: 90_168
            .saturating_add(Weight::from_ref_time(7_750_425).saturating_mul(a.into()))
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(a.into())))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(a.into())))
    }
    /// The range of component `i` is `[0, 4]`.
    fn verify_requirements(i: u32) -> Weight {
        // Minimum execution time: 421 nanoseconds.
        Weight::from_ref_time(737_345)
            // Standard Error: 20_511
            .saturating_add(Weight::from_ref_time(162_577).saturating_mul(i.into()))
    }
    // Storage: Statistics ActiveAssetStats (r:1 w:0)
    // Proof Skipped: Statistics ActiveAssetStats (max_values: None, max_size: None, mode: Measured)
    /// The range of component `a` is `[1, 10]`.
    fn active_asset_statistics_load(a: u32) -> Weight {
        // Minimum execution time: 7_381 nanoseconds.
        Weight::from_ref_time(8_385_558)
            // Standard Error: 24_708
            .saturating_add(Weight::from_ref_time(23_673).saturating_mul(a.into()))
            .saturating_add(DbWeight::get().reads(1))
    }
    // Storage: Statistics TransferConditionExemptEntities (r:1 w:0)
    // Proof Skipped: Statistics TransferConditionExemptEntities (max_values: None, max_size: None, mode: Measured)
    fn is_exempt() -> Weight {
        // Minimum execution time: 8_683 nanoseconds.
        Weight::from_ref_time(8_954_000).saturating_add(DbWeight::get().reads(1))
    }
}
