// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module expose one function `P_NPoS` (Payout NPoS) or `compute_total_payout` which returns
//! the total payout for the era given the era duration and the staking rate in NPoS.
//! The staking rate in NPoS is the total amount of tokens staked by nominators and validators,
//! divided by the total token supply.

use sp_runtime::{curve::PiecewiseLinear, traits::AtLeast32BitUnsigned, Perbill};

/// The total payout to all validators (and their nominators) per era and maximum payout.
///
/// Defined as such:
/// `staker-payout = yearly_inflation(npos_token_staked / total_tokens) * total_tokens /
/// era_per_year` `maximum-payout = max_yearly_inflation * total_tokens / era_per_year`
///
/// `era_duration` is expressed in millisecond.
pub fn compute_total_payout<N>(
    yearly_inflation: &PiecewiseLinear<'static>,
    npos_token_staked: N,
    total_tokens: N,
    era_duration: u64,
    max_inflated_issuance: N,
    non_inflated_yearly_reward: N,
) -> (N, N)
where
    N: AtLeast32BitUnsigned + Clone,
{
    // Milliseconds per year for the Julian year (365.25 days).
    const MILLISECONDS_PER_YEAR: u64 = 1000 * 3600 * 24 * 36525 / 100;

    let portion = Perbill::from_rational(era_duration as u64, MILLISECONDS_PER_YEAR);
    let payout = portion
        * yearly_inflation
            .calculate_for_fraction_times_denominator(npos_token_staked, total_tokens.clone());

    // Polymesh change
    // -----------------------------------------------------------------
    if total_tokens >= max_inflated_issuance {
        let fixed_payout = portion * non_inflated_yearly_reward;
        if fixed_payout <= payout {
            // payout is always maximum.
            return (fixed_payout.clone(), fixed_payout);
        }
    }
    // -----------------------------------------------------------------

    let maximum = portion * (yearly_inflation.maximum * total_tokens);
    (payout, maximum)
}

#[cfg(test)]
mod test {
    use sp_runtime::curve::PiecewiseLinear;

    pallet_staking_reward_curve::build! {
        const I_NPOS: PiecewiseLinear<'static> = curve!(
            min_inflation: 0_025_000,
            max_inflation: 0_100_000,
            ideal_stake: 0_500_000,
            falloff: 0_050_000,
            max_piece_count: 40,
            test_precision: 0_005_000,
        );
    }

    #[test]
    fn npos_curve_is_sensible() {
        const YEAR: u64 = 365 * 24 * 60 * 60 * 1000;
        const FIXED_YEARLY_REWARD: u64 = 1_000_000;
        const MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE: u64 = 1_000_000_000;

        // check maximum inflation.
        // not 10_000 due to rounding error.
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                0,
                100_000u64,
                YEAR,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .1,
            9_993
        );

        // super::I_NPOS.calculate_for_fraction_times_denominator(25, 100)
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                0,
                100_000u64,
                YEAR,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            2_498
        );
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                5_000,
                100_000u64,
                YEAR,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            3_248
        );
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                25_000,
                100_000u64,
                YEAR,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            6_246
        );
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                40_000,
                100_000u64,
                YEAR,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            8_494
        );
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                50_000,
                100_000u64,
                YEAR,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            9_993
        );
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                60_000,
                100_000u64,
                YEAR,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            4_379
        );
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                75_000,
                100_000u64,
                YEAR,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            2_733
        );
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                95_000,
                100_000u64,
                YEAR,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            2_513
        );
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                100_000,
                100_000u64,
                YEAR,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            2_505
        );

        const DAY: u64 = 24 * 60 * 60 * 1000;
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                25_000,
                100_000u64,
                DAY,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            17
        );
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                50_000,
                100_000u64,
                DAY,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            27
        );
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                75_000,
                100_000u64,
                DAY,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            7
        );

        const SIX_HOURS: u64 = 6 * 60 * 60 * 1000;
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                25_000,
                100_000u64,
                SIX_HOURS,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            4
        );
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                50_000,
                100_000u64,
                SIX_HOURS,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            7
        );
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                75_000,
                100_000u64,
                SIX_HOURS,
                MAX_VARIABLE_INFLATION_TOTAL_ISSUANCE,
                FIXED_YEARLY_REWARD
            )
            .0,
            2
        );

        const HOUR: u64 = 60 * 60 * 1000;
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                2_500_000_000_000_000_000_000_000_000u128,
                5_000_000_000_000_000_000_000_000_000u128,
                HOUR,
                7_000_000_000_000_000_000_000_000_000u128,
                FIXED_YEARLY_REWARD.into()
            )
            .0,
            57_038_500_000_000_000_000_000
        );

        // Even though the total issuance is above `max_inflated_issuance` we still have
        // inflation calculated via the curve as this is below the non_inflated_yearly_reward
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                1_000_000,
                1_074_582_300_000_000u128,
                SIX_HOURS,
                1_000_000_000_000_000u128,
                sp_runtime::Perbill::from_percent(5) * 1_000_000_000_000_000u128
            ),
            (18387768858, 73551075022)
        );

        // Since the staking ratio is high enough, the curve calculated inflation is
        // above the fixed `non_inflated_yearly_reward` hence we use the latter
        // i.e. expected response is 5% of 1 billion rather than 10% of 1.5 billion.
        assert_eq!(
            super::compute_total_payout(
                &I_NPOS,
                750_000_000_000_000u128, //50% staking ratio
                1_500_000_000_000_000u128,
                YEAR,
                1_000_000_000_000_000u128,
                sp_runtime::Perbill::from_percent(5) * 1_000_000_000_000_000u128
            ),
            (49_965_776_850_000, 49_965_776_850_000)
        );
    }
}
