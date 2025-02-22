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

//! Autogenerated weights for polymesh_contracts
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

/// Weights for polymesh_contracts using the Substrate node and recommended hardware.
pub struct SubstrateWeight;
impl polymesh_contracts::WeightInfo for SubstrateWeight {
    // Storage: Identity KeyRecords (r:2 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:0)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity IsDidFrozen (r:1 w:0)
    // Proof Skipped: Identity IsDidFrozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: unknown `0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f` (r:1 w:0)
    // Proof Skipped: unknown `0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f` (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    // Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    // Storage: unknown `0x00` (r:1 w:0)
    // Proof Skipped: unknown `0x00` (r:1 w:0)
    /// The range of component `k` is `[1, 8192]`.
    /// The range of component `v` is `[1, 8192]`.
    fn chain_extension_read_storage(k: u32, v: u32) -> Weight {
        // Minimum execution time: 420_509 nanoseconds.
        Weight::from_ref_time(441_154_010)
            // Standard Error: 270
            .saturating_add(Weight::from_ref_time(4_894).saturating_mul(k.into()))
            // Standard Error: 270
            .saturating_add(Weight::from_ref_time(980).saturating_mul(v.into()))
            .saturating_add(DbWeight::get().reads(13))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:2 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:0)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity IsDidFrozen (r:1 w:0)
    // Proof Skipped: Identity IsDidFrozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: System EventTopics (r:2 w:2)
    // Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn chain_extension_get_version(r: u32) -> Weight {
        // Minimum execution time: 404_026 nanoseconds.
        Weight::from_ref_time(430_717_810)
            // Standard Error: 116_585
            .saturating_add(Weight::from_ref_time(57_558_916).saturating_mul(r.into()))
            .saturating_add(DbWeight::get().reads(12))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:2002 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:0)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity IsDidFrozen (r:2001 w:0)
    // Proof Skipped: Identity IsDidFrozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: System EventTopics (r:2 w:2)
    // Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[1, 20]`.
    fn chain_extension_get_key_did(r: u32) -> Weight {
        // Minimum execution time: 1_221_859 nanoseconds.
        Weight::from_ref_time(23_864_904)
            // Standard Error: 2_525_338
            .saturating_add(Weight::from_ref_time(870_955_515).saturating_mul(r.into()))
            .saturating_add(DbWeight::get().reads(12))
            .saturating_add(DbWeight::get().reads((200_u64).saturating_mul(r.into())))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:2 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:0)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity IsDidFrozen (r:1 w:0)
    // Proof Skipped: Identity IsDidFrozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: System EventTopics (r:2 w:2)
    // Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn chain_extension_hash_twox_64(r: u32) -> Weight {
        // Minimum execution time: 407_721 nanoseconds.
        Weight::from_ref_time(436_149_077)
            // Standard Error: 136_925
            .saturating_add(Weight::from_ref_time(78_627_903).saturating_mul(r.into()))
            .saturating_add(DbWeight::get().reads(12))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:2 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:0)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity IsDidFrozen (r:1 w:0)
    // Proof Skipped: Identity IsDidFrozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: System EventTopics (r:2 w:2)
    // Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 64]`.
    fn chain_extension_hash_twox_64_per_kb(n: u32) -> Weight {
        // Minimum execution time: 493_259 nanoseconds.
        Weight::from_ref_time(545_127_910)
            // Standard Error: 65_828
            .saturating_add(Weight::from_ref_time(27_618_039).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(12))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:2 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:0)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity IsDidFrozen (r:1 w:0)
    // Proof Skipped: Identity IsDidFrozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: System EventTopics (r:2 w:2)
    // Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn chain_extension_hash_twox_128(r: u32) -> Weight {
        // Minimum execution time: 403_573 nanoseconds.
        Weight::from_ref_time(444_653_976)
            // Standard Error: 133_724
            .saturating_add(Weight::from_ref_time(78_972_384).saturating_mul(r.into()))
            .saturating_add(DbWeight::get().reads(12))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:2 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:0)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity IsDidFrozen (r:1 w:0)
    // Proof Skipped: Identity IsDidFrozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: System EventTopics (r:2 w:2)
    // Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 64]`.
    fn chain_extension_hash_twox_128_per_kb(n: u32) -> Weight {
        // Minimum execution time: 490_355 nanoseconds.
        Weight::from_ref_time(549_217_273)
            // Standard Error: 60_751
            .saturating_add(Weight::from_ref_time(34_565_161).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(12))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:2 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:0)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity IsDidFrozen (r:1 w:0)
    // Proof Skipped: Identity IsDidFrozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: System EventTopics (r:2 w:2)
    // Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn chain_extension_hash_twox_256(r: u32) -> Weight {
        // Minimum execution time: 412_718 nanoseconds.
        Weight::from_ref_time(440_860_388)
            // Standard Error: 110_188
            .saturating_add(Weight::from_ref_time(85_042_455).saturating_mul(r.into()))
            .saturating_add(DbWeight::get().reads(12))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:2 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:0)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity IsDidFrozen (r:1 w:0)
    // Proof Skipped: Identity IsDidFrozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: System EventTopics (r:2 w:2)
    // Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 64]`.
    fn chain_extension_hash_twox_256_per_kb(n: u32) -> Weight {
        // Minimum execution time: 498_587 nanoseconds.
        Weight::from_ref_time(553_931_299)
            // Standard Error: 63_823
            .saturating_add(Weight::from_ref_time(48_829_146).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(12))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:2 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:0)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity IsDidFrozen (r:1 w:0)
    // Proof Skipped: Identity IsDidFrozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: PolymeshContracts CallRuntimeWhitelist (r:1 w:0)
    // Proof Skipped: PolymeshContracts CallRuntimeWhitelist (max_values: None, max_size: None, mode: Measured)
    // Storage: Identity CurrentPayer (r:1 w:1)
    // Proof Skipped: Identity CurrentPayer (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Permissions CurrentPalletName (r:1 w:1)
    // Proof Skipped: Permissions CurrentPalletName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Permissions CurrentDispatchableName (r:1 w:1)
    // Proof Skipped: Permissions CurrentDispatchableName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: System EventTopics (r:2 w:2)
    // Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[1, 8188]`.
    fn chain_extension_call_runtime(n: u32) -> Weight {
        // Minimum execution time: 445_737 nanoseconds.
        Weight::from_ref_time(480_099_782)
            // Standard Error: 375
            .saturating_add(Weight::from_ref_time(555).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(16))
            .saturating_add(DbWeight::get().writes(6))
    }
    // Storage: Identity KeyRecords (r:2 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:0)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity IsDidFrozen (r:1 w:0)
    // Proof Skipped: Identity IsDidFrozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: System EventTopics (r:2 w:2)
    // Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    fn dummy_contract() -> Weight {
        // Minimum execution time: 242_023 nanoseconds.
        Weight::from_ref_time(246_270_000)
            .saturating_add(DbWeight::get().reads(12))
            .saturating_add(DbWeight::get().writes(3))
    }
    /// The range of component `n` is `[1, 8188]`.
    fn basic_runtime_call(_n: u32) -> Weight {
        // Minimum execution time: 1_963 nanoseconds.
        Weight::from_ref_time(3_024_347)
    }
    /// The range of component `i` is `[0, 1048576]`.
    /// The range of component `s` is `[0, 1048576]`.
    fn base_weight_with_hash(i: u32, s: u32) -> Weight {
        // Minimum execution time: 1_154_999 nanoseconds.
        Weight::from_ref_time(73_388_718)
            // Standard Error: 10
            .saturating_add(Weight::from_ref_time(1_019).saturating_mul(i.into()))
            // Standard Error: 10
            .saturating_add(Weight::from_ref_time(1_157).saturating_mul(s.into()))
    }
    /// The range of component `c` is `[0, 61717]`.
    /// The range of component `i` is `[0, 1048576]`.
    /// The range of component `s` is `[0, 1048576]`.
    fn base_weight_with_code(c: u32, i: u32, s: u32) -> Weight {
        // Minimum execution time: 1_206_667 nanoseconds.
        Weight::from_ref_time(67_372_302)
            // Standard Error: 167
            .saturating_add(Weight::from_ref_time(1_447).saturating_mul(c.into()))
            // Standard Error: 9
            .saturating_add(Weight::from_ref_time(996).saturating_mul(i.into()))
            // Standard Error: 9
            .saturating_add(Weight::from_ref_time(1_137).saturating_mul(s.into()))
    }
    // Storage: PolymeshContracts CallRuntimeWhitelist (r:0 w:2000)
    // Proof Skipped: PolymeshContracts CallRuntimeWhitelist (max_values: None, max_size: None, mode: Measured)
    /// The range of component `u` is `[0, 2000]`.
    fn update_call_runtime_whitelist(u: u32) -> Weight {
        // Minimum execution time: 3_936 nanoseconds.
        Weight::from_ref_time(4_167_000)
            // Standard Error: 2_190
            .saturating_add(Weight::from_ref_time(1_653_031).saturating_mul(u.into()))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(u.into())))
    }
    // Storage: Identity KeyRecords (r:2 w:1)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: Identity DidKeys (r:0 w:1)
    // Proof Skipped: Identity DidKeys (max_values: None, max_size: None, mode: Measured)
    // Storage: Identity KeyPortfolioPermissions (r:0 w:1)
    // Proof Skipped: Identity KeyPortfolioPermissions (max_values: None, max_size: None, mode: Measured)
    // Storage: Identity KeyExtrinsicPermissions (r:0 w:1)
    // Proof Skipped: Identity KeyExtrinsicPermissions (max_values: None, max_size: None, mode: Measured)
    // Storage: Identity KeyAssetPermissions (r:0 w:1)
    // Proof Skipped: Identity KeyAssetPermissions (max_values: None, max_size: None, mode: Measured)
    fn link_contract_as_secondary_key() -> Weight {
        // Minimum execution time: 26_339 nanoseconds.
        Weight::from_ref_time(28_423_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(5))
    }
    // Storage: Identity KeyRecords (r:2 w:1)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: Identity ParentDid (r:1 w:1)
    // Proof Skipped: Identity ParentDid (max_values: None, max_size: None, mode: Measured)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Proof Skipped: ProtocolFee Coefficient (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: ProtocolFee BaseFees (r:1 w:0)
    // Proof Skipped: ProtocolFee BaseFees (max_values: None, max_size: None, mode: Measured)
    // Storage: Identity MultiPurposeNonce (r:1 w:1)
    // Proof Skipped: Identity MultiPurposeNonce (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: System ParentHash (r:1 w:0)
    // Proof: System ParentHash (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
    // Storage: Identity DidRecords (r:1 w:1)
    // Proof Skipped: Identity DidRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: Identity DidKeys (r:0 w:1)
    // Proof Skipped: Identity DidKeys (max_values: None, max_size: None, mode: Measured)
    // Storage: Identity ChildDid (r:0 w:1)
    // Proof Skipped: Identity ChildDid (max_values: None, max_size: None, mode: Measured)
    fn link_contract_as_primary_key() -> Weight {
        // Minimum execution time: 47_721 nanoseconds.
        Weight::from_ref_time(52_568_000)
            .saturating_add(DbWeight::get().reads(8))
            .saturating_add(DbWeight::get().writes(6))
    }
    // Storage: PolymeshContracts ApiNextUpgrade (r:0 w:1)
    // Proof Skipped: PolymeshContracts ApiNextUpgrade (max_values: None, max_size: None, mode: Measured)
    fn upgrade_api() -> Weight {
        // Minimum execution time: 12_098 nanoseconds.
        Weight::from_ref_time(13_571_000).saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:2 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:0)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity IsDidFrozen (r:1 w:0)
    // Proof Skipped: Identity IsDidFrozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: PolymeshContracts CurrentApiHash (r:1 w:1)
    // Proof Skipped: PolymeshContracts CurrentApiHash (max_values: None, max_size: None, mode: Measured)
    // Storage: PolymeshContracts ApiNextUpgrade (r:1 w:1)
    // Proof Skipped: PolymeshContracts ApiNextUpgrade (max_values: None, max_size: None, mode: Measured)
    // Storage: System EventTopics (r:2 w:2)
    // Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn chain_extension_get_latest_api_upgrade(r: u32) -> Weight {
        // Minimum execution time: 402_874 nanoseconds.
        Weight::from_ref_time(468_862_333)
            // Standard Error: 1_989_018
            .saturating_add(Weight::from_ref_time(376_144_330).saturating_mul(r.into()))
            .saturating_add(DbWeight::get().reads(14))
            .saturating_add(DbWeight::get().writes(5))
    }
    // Storage: Identity KeyRecords (r:2 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: System Account (r:1 w:0)
    // Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Identity IsDidFrozen (r:1 w:0)
    // Proof Skipped: Identity IsDidFrozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Proof Skipped: Instance2Group ActiveMembers (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Identity Claims (r:2 w:0)
    // Proof Skipped: Identity Claims (max_values: None, max_size: None, mode: Measured)
    // Storage: System BlockHash (r:1 w:0)
    // Proof: System BlockHash (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
    // Storage: Asset AssetNonce (r:1 w:0)
    // Proof Skipped: Asset AssetNonce (max_values: None, max_size: None, mode: Measured)
    // Storage: System EventTopics (r:2 w:2)
    // Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn chain_extension_get_next_asset_id(r: u32) -> Weight {
        // Minimum execution time: 413_229 nanoseconds.
        Weight::from_ref_time(437_313_575)
            // Standard Error: 863_761
            .saturating_add(Weight::from_ref_time(428_114_434).saturating_mul(r.into()))
            .saturating_add(DbWeight::get().reads(14))
            .saturating_add(DbWeight::get().writes(3))
    }
}
