// This file is part of Mangata.

// Copyright (C) 2020-2022 Mangata Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for parachain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/mangata-node
// benchmark
// pallet
// -l=info,xyk=error,collective-mangata=warn,bootstrap=warn
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template
// ./templates/module-weight-template.hbs
// --output
// ./benchmarks/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for parachain_staking.
pub trait WeightInfo {
	fn set_total_selected() -> Weight;
	fn set_collator_commission() -> Weight;
	fn join_candidates(x: u32, y: u32, ) -> Weight;
	fn schedule_leave_candidates(x: u32, ) -> Weight;
	fn execute_leave_candidates(x: u32, ) -> Weight;
	fn cancel_leave_candidates(x: u32, ) -> Weight;
	fn go_offline() -> Weight;
	fn go_online() -> Weight;
	fn schedule_candidate_bond_more() -> Weight;
	fn schedule_candidate_bond_less() -> Weight;
	fn execute_candidate_bond_more() -> Weight;
	fn execute_candidate_bond_less() -> Weight;
	fn cancel_candidate_bond_more() -> Weight;
	fn cancel_candidate_bond_less() -> Weight;
	fn delegate(x: u32, y: u32, ) -> Weight;
	fn schedule_leave_delegators() -> Weight;
	fn execute_leave_delegators(x: u32, ) -> Weight;
	fn cancel_leave_delegators() -> Weight;
	fn schedule_revoke_delegation() -> Weight;
	fn schedule_delegator_bond_more() -> Weight;
	fn schedule_delegator_bond_less() -> Weight;
	fn execute_revoke_delegation() -> Weight;
	fn execute_delegator_bond_more() -> Weight;
	fn execute_delegator_bond_less() -> Weight;
	fn cancel_revoke_delegation() -> Weight;
	fn cancel_delegator_bond_more() -> Weight;
	fn cancel_delegator_bond_less() -> Weight;
	fn add_staking_liquidity_token(x: u32, ) -> Weight;
	fn remove_staking_liquidity_token(x: u32, ) -> Weight;
	fn passive_session_change() -> Weight;
	fn active_session_change(x: u32, y: u32, z: u32, w: u32, ) -> Weight;
}

/// Weights for parachain_staking using the Mangata node and recommended hardware.
pub struct ModuleWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> parachain_staking::WeightInfo for ModuleWeight<T> {
	// Storage: ParachainStaking TotalSelected (r:1 w:1)
	fn set_total_selected() -> Weight {
		(Weight::from_ref_time(24_350_000))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CollatorCommission (r:1 w:1)
	fn set_collator_commission() -> Weight {
		(Weight::from_ref_time(24_780_000))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking StakingLiquidityTokens (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn join_candidates(x: u32, y: u32, ) -> Weight {
		(Weight::from_ref_time(86_133_703))
			// Standard Error: 8_043
			.saturating_add((Weight::from_ref_time(533_770)).saturating_mul(x as u64))
			// Standard Error: 2_594
			.saturating_add((Weight::from_ref_time(83_926)).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn schedule_leave_candidates(x: u32, ) -> Weight {
		(Weight::from_ref_time(46_728_747))
			// Standard Error: 5_272
			.saturating_add((Weight::from_ref_time(524_090)).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_leave_candidates(x: u32, ) -> Weight {
		(Weight::from_ref_time(63_768_377))
			// Standard Error: 36_320
			.saturating_add((Weight::from_ref_time(25_162_816)).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn cancel_leave_candidates(x: u32, ) -> Weight {
		(Weight::from_ref_time(44_692_784))
			// Standard Error: 5_107
			.saturating_add((Weight::from_ref_time(507_637)).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn go_offline() -> Weight {
		(Weight::from_ref_time(46_890_000))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn go_online() -> Weight {
		(Weight::from_ref_time(46_090_000))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:0)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_candidate_bond_more() -> Weight {
		(Weight::from_ref_time(60_370_000))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_candidate_bond_less() -> Weight {
		(Weight::from_ref_time(40_460_000))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn execute_candidate_bond_more() -> Weight {
		(Weight::from_ref_time(96_750_000))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn execute_candidate_bond_less() -> Weight {
		(Weight::from_ref_time(90_650_000))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	fn cancel_candidate_bond_more() -> Weight {
		(Weight::from_ref_time(38_680_000))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	fn cancel_candidate_bond_less() -> Weight {
		(Weight::from_ref_time(38_150_000))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn delegate(x: u32, y: u32, ) -> Weight {
		(Weight::from_ref_time(94_170_667))
			// Standard Error: 6_316
			.saturating_add((Weight::from_ref_time(639_847)).saturating_mul(x as u64))
			// Standard Error: 16_524
			.saturating_add((Weight::from_ref_time(646_942)).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_leave_delegators() -> Weight {
		(Weight::from_ref_time(40_350_000))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_leave_delegators(x: u32, ) -> Weight {
		(Weight::from_ref_time(41_932_764))
			// Standard Error: 32_909
			.saturating_add((Weight::from_ref_time(29_709_209)).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	fn cancel_leave_delegators() -> Weight {
		(Weight::from_ref_time(37_910_000))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_revoke_delegation() -> Weight {
		(Weight::from_ref_time(40_750_000))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:0)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_delegator_bond_more() -> Weight {
		(Weight::from_ref_time(61_310_000))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_delegator_bond_less() -> Weight {
		(Weight::from_ref_time(41_000_000))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_revoke_delegation() -> Weight {
		(Weight::from_ref_time(111_790_000))
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_delegator_bond_more() -> Weight {
		(Weight::from_ref_time(104_990_000))
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_delegator_bond_less() -> Weight {
		(Weight::from_ref_time(102_090_000))
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	fn cancel_revoke_delegation() -> Weight {
		(Weight::from_ref_time(39_000_000))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	fn cancel_delegator_bond_more() -> Weight {
		(Weight::from_ref_time(49_650_000))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	fn cancel_delegator_bond_less() -> Weight {
		(Weight::from_ref_time(52_100_000))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking StakingLiquidityTokens (r:1 w:1)
	fn add_staking_liquidity_token(x: u32, ) -> Weight {
		(Weight::from_ref_time(17_563_500))
			// Standard Error: 759
			.saturating_add((Weight::from_ref_time(91_140)).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking StakingLiquidityTokens (r:1 w:1)
	fn remove_staking_liquidity_token(x: u32, ) -> Weight {
		(Weight::from_ref_time(17_589_687))
			// Standard Error: 738
			.saturating_add((Weight::from_ref_time(92_659)).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:0)
	fn passive_session_change() -> Weight {
		(Weight::from_ref_time(10_670_000))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: Session CurrentIndex (r:1 w:1)
	// Storage: Session QueuedChanged (r:1 w:1)
	// Storage: Session QueuedKeys (r:1 w:1)
	// Storage: Session DisabledValidators (r:1 w:0)
	// Storage: ParachainStaking Points (r:1 w:1)
	// Storage: Issuance SessionIssuance (r:1 w:1)
	// Storage: ParachainStaking CollatorCommission (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:34 w:33)
	// Storage: ParachainStaking AtStake (r:33 w:66)
	// Storage: Tokens Accounts (r:431 w:431)
	// Storage: System Account (r:430 w:429)
	// Storage: ParachainStaking StakingLiquidityTokens (r:1 w:1)
	// Storage: Xyk LiquidityPools (r:3 w:0)
	// Storage: Xyk Pools (r:3 w:0)
	// Storage: Tokens TotalIssuance (r:4 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking CandidateState (r:33 w:0)
	// Storage: Issuance IssuanceConfigStore (r:1 w:0)
	// Storage: Issuance PromotedPoolsRewardsV2 (r:1 w:1)
	// Storage: Session NextKeys (r:33 w:0)
	// Storage: Aura Authorities (r:1 w:0)
	// Storage: ParachainStaking SelectedCandidates (r:0 w:1)
	// Storage: Session Validators (r:0 w:1)
	fn active_session_change(x: u32, y: u32, z: u32, w: u32, ) -> Weight {
		(Weight::from_ref_time(7_206_509_000))
			// Standard Error: 7_392_219
			.saturating_add((Weight::from_ref_time(594_710)).saturating_mul(y as u64))
			// Standard Error: 8_267_185
			.saturating_add((Weight::from_ref_time(966_049_137)).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(715 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().reads((46 as u64).saturating_mul(z as u64)))
			.saturating_add(T::DbWeight::get().writes(376 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(y as u64)))
			.saturating_add(T::DbWeight::get().writes((49 as u64).saturating_mul(z as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(w as u64)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: ParachainStaking TotalSelected (r:1 w:1)
	fn set_total_selected() -> Weight {
		(Weight::from_ref_time(24_350_000))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CollatorCommission (r:1 w:1)
	fn set_collator_commission() -> Weight {
		(Weight::from_ref_time(24_780_000))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking StakingLiquidityTokens (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn join_candidates(x: u32, y: u32, ) -> Weight {
		(Weight::from_ref_time(86_133_703))
			// Standard Error: 8_043
			.saturating_add((Weight::from_ref_time(533_770)).saturating_mul(x as u64))
			// Standard Error: 2_594
			.saturating_add((Weight::from_ref_time(83_926)).saturating_mul(y as u64))
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn schedule_leave_candidates(x: u32, ) -> Weight {
		(Weight::from_ref_time(46_728_747))
			// Standard Error: 5_272
			.saturating_add((Weight::from_ref_time(524_090)).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_leave_candidates(x: u32, ) -> Weight {
		(Weight::from_ref_time(63_768_377))
			// Standard Error: 36_320
			.saturating_add((Weight::from_ref_time(25_162_816)).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().reads((3 as u64).saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
			.saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn cancel_leave_candidates(x: u32, ) -> Weight {
		(Weight::from_ref_time(44_692_784))
			// Standard Error: 5_107
			.saturating_add((Weight::from_ref_time(507_637)).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn go_offline() -> Weight {
		(Weight::from_ref_time(46_890_000))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn go_online() -> Weight {
		(Weight::from_ref_time(46_090_000))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:0)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_candidate_bond_more() -> Weight {
		(Weight::from_ref_time(60_370_000))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_candidate_bond_less() -> Weight {
		(Weight::from_ref_time(40_460_000))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn execute_candidate_bond_more() -> Weight {
		(Weight::from_ref_time(96_750_000))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn execute_candidate_bond_less() -> Weight {
		(Weight::from_ref_time(90_650_000))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	fn cancel_candidate_bond_more() -> Weight {
		(Weight::from_ref_time(38_680_000))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	fn cancel_candidate_bond_less() -> Weight {
		(Weight::from_ref_time(38_150_000))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn delegate(x: u32, y: u32, ) -> Weight {
		(Weight::from_ref_time(94_170_667))
			// Standard Error: 6_316
			.saturating_add((Weight::from_ref_time(639_847)).saturating_mul(x as u64))
			// Standard Error: 16_524
			.saturating_add((Weight::from_ref_time(646_942)).saturating_mul(y as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_leave_delegators() -> Weight {
		(Weight::from_ref_time(40_350_000))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_leave_delegators(x: u32, ) -> Weight {
		(Weight::from_ref_time(41_932_764))
			// Standard Error: 32_909
			.saturating_add((Weight::from_ref_time(29_709_209)).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	fn cancel_leave_delegators() -> Weight {
		(Weight::from_ref_time(37_910_000))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_revoke_delegation() -> Weight {
		(Weight::from_ref_time(40_750_000))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:0)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_delegator_bond_more() -> Weight {
		(Weight::from_ref_time(61_310_000))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_delegator_bond_less() -> Weight {
		(Weight::from_ref_time(41_000_000))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_revoke_delegation() -> Weight {
		(Weight::from_ref_time(111_790_000))
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_delegator_bond_more() -> Weight {
		(Weight::from_ref_time(104_990_000))
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidateState (r:1 w:1)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_delegator_bond_less() -> Weight {
		(Weight::from_ref_time(102_090_000))
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	fn cancel_revoke_delegation() -> Weight {
		(Weight::from_ref_time(39_000_000))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	fn cancel_delegator_bond_more() -> Weight {
		(Weight::from_ref_time(49_650_000))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	fn cancel_delegator_bond_less() -> Weight {
		(Weight::from_ref_time(52_100_000))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking StakingLiquidityTokens (r:1 w:1)
	fn add_staking_liquidity_token(x: u32, ) -> Weight {
		(Weight::from_ref_time(17_563_500))
			// Standard Error: 759
			.saturating_add((Weight::from_ref_time(91_140)).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking StakingLiquidityTokens (r:1 w:1)
	fn remove_staking_liquidity_token(x: u32, ) -> Weight {
		(Weight::from_ref_time(17_589_687))
			// Standard Error: 738
			.saturating_add((Weight::from_ref_time(92_659)).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:0)
	fn passive_session_change() -> Weight {
		(Weight::from_ref_time(10_670_000))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: Session CurrentIndex (r:1 w:1)
	// Storage: Session QueuedChanged (r:1 w:1)
	// Storage: Session QueuedKeys (r:1 w:1)
	// Storage: Session DisabledValidators (r:1 w:0)
	// Storage: ParachainStaking Points (r:1 w:1)
	// Storage: Issuance SessionIssuance (r:1 w:1)
	// Storage: ParachainStaking CollatorCommission (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:34 w:33)
	// Storage: ParachainStaking AtStake (r:33 w:66)
	// Storage: Tokens Accounts (r:431 w:431)
	// Storage: System Account (r:430 w:429)
	// Storage: ParachainStaking StakingLiquidityTokens (r:1 w:1)
	// Storage: Xyk LiquidityPools (r:3 w:0)
	// Storage: Xyk Pools (r:3 w:0)
	// Storage: Tokens TotalIssuance (r:4 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking CandidateState (r:33 w:0)
	// Storage: Issuance IssuanceConfigStore (r:1 w:0)
	// Storage: Issuance PromotedPoolsRewardsV2 (r:1 w:1)
	// Storage: Session NextKeys (r:33 w:0)
	// Storage: Aura Authorities (r:1 w:0)
	// Storage: ParachainStaking SelectedCandidates (r:0 w:1)
	// Storage: Session Validators (r:0 w:1)
	fn active_session_change(x: u32, y: u32, z: u32, w: u32, ) -> Weight {
		(Weight::from_ref_time(7_206_509_000))
			// Standard Error: 7_392_219
			.saturating_add((Weight::from_ref_time(594_710)).saturating_mul(y as u64))
			// Standard Error: 8_267_185
			.saturating_add((Weight::from_ref_time(966_049_137)).saturating_mul(z as u64))
			.saturating_add(RocksDbWeight::get().reads(715 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().reads((46 as u64).saturating_mul(z as u64)))
			.saturating_add(RocksDbWeight::get().writes(376 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(y as u64)))
			.saturating_add(RocksDbWeight::get().writes((49 as u64).saturating_mul(z as u64)))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(w as u64)))
	}
}
