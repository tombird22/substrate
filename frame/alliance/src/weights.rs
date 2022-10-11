// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_alliance
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-09, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm2`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_alliance
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/alliance/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_alliance.
pub trait WeightInfo {
	fn propose_proposed(b: u32, x: u32, y: u32, p: u32, ) -> Weight;
	fn vote(x: u32, y: u32, ) -> Weight;
	fn veto(p: u32, ) -> Weight;
	fn close_early_disapproved(x: u32, y: u32, p: u32, ) -> Weight;
	fn close_early_approved(b: u32, x: u32, y: u32, p: u32, ) -> Weight;
	fn close_disapproved(x: u32, y: u32, p: u32, ) -> Weight;
	fn close_approved(b: u32, x: u32, y: u32, p: u32, ) -> Weight;
	fn init_members(x: u32, y: u32, z: u32, ) -> Weight;
	fn disband(x: u32, y: u32, z: u32, ) -> Weight;
	fn set_rule() -> Weight;
	fn announce() -> Weight;
	fn remove_announcement() -> Weight;
	fn join_alliance() -> Weight;
	fn nominate_ally() -> Weight;
	fn elevate_ally() -> Weight;
	fn give_retirement_notice() -> Weight;
	fn retire() -> Weight;
	fn kick_member() -> Weight;
	fn add_unscrupulous_items(n: u32, l: u32, ) -> Weight;
	fn remove_unscrupulous_items(n: u32, l: u32, ) -> Weight;
}

/// Weights for pallet_alliance using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalCount (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[0, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(_b: u32, _x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(42_137_000 as u64)
			// Standard Error: 1_738
			.saturating_add(Weight::from_ref_time(58_740 as u64).saturating_mul(y as u64))
			// Standard Error: 1_578
			.saturating_add(Weight::from_ref_time(192_712 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:2 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	/// The range of component `x` is `[3, 10]`.
	/// The range of component `y` is `[2, 90]`.
	fn vote(x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(45_451_000 as u64)
			// Standard Error: 13_651
			.saturating_add(Weight::from_ref_time(249_084 as u64).saturating_mul(x as u64))
			// Standard Error: 1_557
			.saturating_add(Weight::from_ref_time(118_391 as u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn veto(p: u32, ) -> Weight {
		Weight::from_ref_time(33_083_000 as u64)
			// Standard Error: 1_537
			.saturating_add(Weight::from_ref_time(216_836 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(_x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(50_010_000 as u64)
			// Standard Error: 1_592
			.saturating_add(Weight::from_ref_time(55_636 as u64).saturating_mul(y as u64))
			// Standard Error: 1_427
			.saturating_add(Weight::from_ref_time(133_860 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(_b: u32, _x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(59_041_000 as u64)
			// Standard Error: 1_843
			.saturating_add(Weight::from_ref_time(56_707 as u64).saturating_mul(y as u64))
			// Standard Error: 1_649
			.saturating_add(Weight::from_ref_time(194_283 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(_x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(50_631_000 as u64)
			// Standard Error: 1_168
			.saturating_add(Weight::from_ref_time(74_052 as u64).saturating_mul(y as u64))
			// Standard Error: 1_047
			.saturating_add(Weight::from_ref_time(166_297 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(_b: u32, _x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(50_593_000 as u64)
			// Standard Error: 1_161
			.saturating_add(Weight::from_ref_time(73_655 as u64).saturating_mul(y as u64))
			// Standard Error: 1_039
			.saturating_add(Weight::from_ref_time(172_875 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:3 w:3)
	// Storage: AllianceMotion Members (r:1 w:1)
	/// The range of component `x` is `[1, 10]`.
	/// The range of component `y` is `[0, 90]`.
	/// The range of component `z` is `[0, 100]`.
	fn init_members(_x: u32, y: u32, z: u32, ) -> Weight {
		Weight::from_ref_time(47_831_000 as u64)
			// Standard Error: 1_626
			.saturating_add(Weight::from_ref_time(104_235 as u64).saturating_mul(y as u64))
			// Standard Error: 1_465
			.saturating_add(Weight::from_ref_time(85_144 as u64).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:3 w:3)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:101 w:50)
	// Storage: System Account (r:50 w:50)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	/// The range of component `x` is `[1, 100]`.
	/// The range of component `y` is `[0, 100]`.
	/// The range of component `z` is `[0, 50]`.
	fn disband(x: u32, y: u32, z: u32, ) -> Weight {
		Weight::from_ref_time(253_490_000 as u64)
			// Standard Error: 19_281
			.saturating_add(Weight::from_ref_time(424_258 as u64).saturating_mul(x as u64))
			// Standard Error: 19_188
			.saturating_add(Weight::from_ref_time(493_237 as u64).saturating_mul(y as u64))
			// Standard Error: 38_342
			.saturating_add(Weight::from_ref_time(8_740_369 as u64).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(154 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(z as u64)))
	}
	// Storage: Alliance Rule (r:0 w:1)
	fn set_rule() -> Weight {
		Weight::from_ref_time(18_647_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn announce() -> Weight {
		Weight::from_ref_time(21_715_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn remove_announcement() -> Weight {
		Weight::from_ref_time(22_930_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:4 w:1)
	// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Alliance DepositOf (r:0 w:1)
	fn join_alliance() -> Weight {
		Weight::from_ref_time(55_560_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:4 w:1)
	// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	fn nominate_ally() -> Weight {
		Weight::from_ref_time(43_177_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:3 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn elevate_ally() -> Weight {
		Weight::from_ref_time(37_807_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:4 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	// Storage: Alliance RetiringMembers (r:0 w:1)
	fn give_retirement_notice() -> Weight {
		Weight::from_ref_time(42_080_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Alliance RetiringMembers (r:1 w:1)
	// Storage: Alliance Members (r:1 w:1)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn retire() -> Weight {
		Weight::from_ref_time(44_398_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:3 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn kick_member() -> Weight {
		Weight::from_ref_time(63_269_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `l` is `[0, 255]`.
	fn add_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		Weight::from_ref_time(16_979_000 as u64)
			// Standard Error: 2_708
			.saturating_add(Weight::from_ref_time(1_268_666 as u64).saturating_mul(n as u64))
			// Standard Error: 1_060
			.saturating_add(Weight::from_ref_time(70_658 as u64).saturating_mul(l as u64))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `l` is `[0, 255]`.
	fn remove_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		Weight::from_ref_time(17_069_000 as u64)
			// Standard Error: 161_628
			.saturating_add(Weight::from_ref_time(12_573_335 as u64).saturating_mul(n as u64))
			// Standard Error: 63_300
			.saturating_add(Weight::from_ref_time(501_534 as u64).saturating_mul(l as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalCount (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[0, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(_b: u32, _x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(42_137_000 as u64)
			// Standard Error: 1_738
			.saturating_add(Weight::from_ref_time(58_740 as u64).saturating_mul(y as u64))
			// Standard Error: 1_578
			.saturating_add(Weight::from_ref_time(192_712 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:2 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	/// The range of component `x` is `[3, 10]`.
	/// The range of component `y` is `[2, 90]`.
	fn vote(x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(45_451_000 as u64)
			// Standard Error: 13_651
			.saturating_add(Weight::from_ref_time(249_084 as u64).saturating_mul(x as u64))
			// Standard Error: 1_557
			.saturating_add(Weight::from_ref_time(118_391 as u64).saturating_mul(y as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn veto(p: u32, ) -> Weight {
		Weight::from_ref_time(33_083_000 as u64)
			// Standard Error: 1_537
			.saturating_add(Weight::from_ref_time(216_836 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(_x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(50_010_000 as u64)
			// Standard Error: 1_592
			.saturating_add(Weight::from_ref_time(55_636 as u64).saturating_mul(y as u64))
			// Standard Error: 1_427
			.saturating_add(Weight::from_ref_time(133_860 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(_b: u32, _x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(59_041_000 as u64)
			// Standard Error: 1_843
			.saturating_add(Weight::from_ref_time(56_707 as u64).saturating_mul(y as u64))
			// Standard Error: 1_649
			.saturating_add(Weight::from_ref_time(194_283 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(_x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(50_631_000 as u64)
			// Standard Error: 1_168
			.saturating_add(Weight::from_ref_time(74_052 as u64).saturating_mul(y as u64))
			// Standard Error: 1_047
			.saturating_add(Weight::from_ref_time(166_297 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(_b: u32, _x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(50_593_000 as u64)
			// Standard Error: 1_161
			.saturating_add(Weight::from_ref_time(73_655 as u64).saturating_mul(y as u64))
			// Standard Error: 1_039
			.saturating_add(Weight::from_ref_time(172_875 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:3 w:3)
	// Storage: AllianceMotion Members (r:1 w:1)
	/// The range of component `x` is `[1, 10]`.
	/// The range of component `y` is `[0, 90]`.
	/// The range of component `z` is `[0, 100]`.
	fn init_members(_x: u32, y: u32, z: u32, ) -> Weight {
		Weight::from_ref_time(47_831_000 as u64)
			// Standard Error: 1_626
			.saturating_add(Weight::from_ref_time(104_235 as u64).saturating_mul(y as u64))
			// Standard Error: 1_465
			.saturating_add(Weight::from_ref_time(85_144 as u64).saturating_mul(z as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:3 w:3)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:101 w:50)
	// Storage: System Account (r:50 w:50)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	/// The range of component `x` is `[1, 100]`.
	/// The range of component `y` is `[0, 100]`.
	/// The range of component `z` is `[0, 50]`.
	fn disband(x: u32, y: u32, z: u32, ) -> Weight {
		Weight::from_ref_time(253_490_000 as u64)
			// Standard Error: 19_281
			.saturating_add(Weight::from_ref_time(424_258 as u64).saturating_mul(x as u64))
			// Standard Error: 19_188
			.saturating_add(Weight::from_ref_time(493_237 as u64).saturating_mul(y as u64))
			// Standard Error: 38_342
			.saturating_add(Weight::from_ref_time(8_740_369 as u64).saturating_mul(z as u64))
			.saturating_add(RocksDbWeight::get().reads(154 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
			.saturating_add(RocksDbWeight::get().writes((2 as u64).saturating_mul(z as u64)))
	}
	// Storage: Alliance Rule (r:0 w:1)
	fn set_rule() -> Weight {
		Weight::from_ref_time(18_647_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn announce() -> Weight {
		Weight::from_ref_time(21_715_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn remove_announcement() -> Weight {
		Weight::from_ref_time(22_930_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:4 w:1)
	// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Alliance DepositOf (r:0 w:1)
	fn join_alliance() -> Weight {
		Weight::from_ref_time(55_560_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:4 w:1)
	// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	fn nominate_ally() -> Weight {
		Weight::from_ref_time(43_177_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:3 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn elevate_ally() -> Weight {
		Weight::from_ref_time(37_807_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:4 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	// Storage: Alliance RetiringMembers (r:0 w:1)
	fn give_retirement_notice() -> Weight {
		Weight::from_ref_time(42_080_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Alliance RetiringMembers (r:1 w:1)
	// Storage: Alliance Members (r:1 w:1)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn retire() -> Weight {
		Weight::from_ref_time(44_398_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:3 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn kick_member() -> Weight {
		Weight::from_ref_time(63_269_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `l` is `[0, 255]`.
	fn add_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		Weight::from_ref_time(16_979_000 as u64)
			// Standard Error: 2_708
			.saturating_add(Weight::from_ref_time(1_268_666 as u64).saturating_mul(n as u64))
			// Standard Error: 1_060
			.saturating_add(Weight::from_ref_time(70_658 as u64).saturating_mul(l as u64))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `l` is `[0, 255]`.
	fn remove_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		Weight::from_ref_time(17_069_000 as u64)
			// Standard Error: 161_628
			.saturating_add(Weight::from_ref_time(12_573_335 as u64).saturating_mul(n as u64))
			// Standard Error: 63_300
			.saturating_add(Weight::from_ref_time(501_534 as u64).saturating_mul(l as u64))
	}
}
