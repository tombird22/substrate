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

//! Autogenerated weights for pallet_election_provider_multi_phase
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm2`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_election_provider_multi_phase
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/election-provider-multi-phase/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_election_provider_multi_phase.
pub trait WeightInfo {
	fn on_initialize_nothing() -> Weight;
	fn on_initialize_open_signed() -> Weight;
	fn on_initialize_open_unsigned() -> Weight;
	fn finalize_signed_phase_accept_solution() -> Weight;
	fn finalize_signed_phase_reject_solution() -> Weight;
	fn create_snapshot_internal(v: u32, t: u32, ) -> Weight;
	fn elect_queued(a: u32, d: u32, ) -> Weight;
	fn submit() -> Weight;
	fn submit_unsigned(v: u32, t: u32, a: u32, d: u32, ) -> Weight;
	fn feasibility_check(v: u32, t: u32, a: u32, d: u32, ) -> Weight;
}

/// Weights for pallet_election_provider_multi_phase using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking CurrentPlannedSession (r:1 w:0)
	// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	// Storage: Babe EpochIndex (r:1 w:0)
	// Storage: Babe GenesisSlot (r:1 w:0)
	// Storage: Babe CurrentSlot (r:1 w:0)
	// Storage: Staking ForceEra (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	fn on_initialize_nothing() -> Weight {
		Weight::from_ref_time(16_707_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:0 w:1)
	fn on_initialize_open_signed() -> Weight {
		Weight::from_ref_time(17_550_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:0 w:1)
	fn on_initialize_open_unsigned() -> Weight {
		Weight::from_ref_time(17_011_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:0 w:1)
	fn finalize_signed_phase_accept_solution() -> Weight {
		Weight::from_ref_time(35_011_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: System Account (r:1 w:1)
	fn finalize_signed_phase_reject_solution() -> Weight {
		Weight::from_ref_time(26_839_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:0 w:1)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	fn create_snapshot_internal(v: u32, _t: u32, ) -> Weight {
		Weight::from_ref_time(558_496_000 as u64)
			// Standard Error: 7_171
			.saturating_add(Weight::from_ref_time(396_673 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:1 w:0)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	// Storage: ElectionProviderMultiPhase Round (r:1 w:1)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:0 w:1)
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn elect_queued(a: u32, _d: u32, ) -> Weight {
		Weight::from_ref_time(1_289_552_000 as u64)
			// Standard Error: 3_818
			.saturating_add(Weight::from_ref_time(266_728 as u64).saturating_mul(a as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:0 w:1)
	fn submit() -> Weight {
		Weight::from_ref_time(58_963_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn submit_unsigned(v: u32, _t: u32, a: u32, _d: u32, ) -> Weight {
		Weight::from_ref_time(5_586_779_000 as u64)
			// Standard Error: 18_837
			.saturating_add(Weight::from_ref_time(607_682 as u64).saturating_mul(v as u64))
			// Standard Error: 55_822
			.saturating_add(Weight::from_ref_time(3_158_078 as u64).saturating_mul(a as u64))
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn feasibility_check(v: u32, _t: u32, a: u32, _d: u32, ) -> Weight {
		Weight::from_ref_time(5_034_507_000 as u64)
			// Standard Error: 16_874
			.saturating_add(Weight::from_ref_time(697_758 as u64).saturating_mul(v as u64))
			// Standard Error: 50_006
			.saturating_add(Weight::from_ref_time(2_234_039 as u64).saturating_mul(a as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking CurrentPlannedSession (r:1 w:0)
	// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	// Storage: Babe EpochIndex (r:1 w:0)
	// Storage: Babe GenesisSlot (r:1 w:0)
	// Storage: Babe CurrentSlot (r:1 w:0)
	// Storage: Staking ForceEra (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	fn on_initialize_nothing() -> Weight {
		Weight::from_ref_time(16_707_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:0 w:1)
	fn on_initialize_open_signed() -> Weight {
		Weight::from_ref_time(17_550_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:0 w:1)
	fn on_initialize_open_unsigned() -> Weight {
		Weight::from_ref_time(17_011_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:0 w:1)
	fn finalize_signed_phase_accept_solution() -> Weight {
		Weight::from_ref_time(35_011_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: System Account (r:1 w:1)
	fn finalize_signed_phase_reject_solution() -> Weight {
		Weight::from_ref_time(26_839_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:0 w:1)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	fn create_snapshot_internal(v: u32, _t: u32, ) -> Weight {
		Weight::from_ref_time(558_496_000 as u64)
			// Standard Error: 7_171
			.saturating_add(Weight::from_ref_time(396_673 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:1 w:0)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	// Storage: ElectionProviderMultiPhase Round (r:1 w:1)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:0 w:1)
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn elect_queued(a: u32, _d: u32, ) -> Weight {
		Weight::from_ref_time(1_289_552_000 as u64)
			// Standard Error: 3_818
			.saturating_add(Weight::from_ref_time(266_728 as u64).saturating_mul(a as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:0 w:1)
	fn submit() -> Weight {
		Weight::from_ref_time(58_963_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn submit_unsigned(v: u32, _t: u32, a: u32, _d: u32, ) -> Weight {
		Weight::from_ref_time(5_586_779_000 as u64)
			// Standard Error: 18_837
			.saturating_add(Weight::from_ref_time(607_682 as u64).saturating_mul(v as u64))
			// Standard Error: 55_822
			.saturating_add(Weight::from_ref_time(3_158_078 as u64).saturating_mul(a as u64))
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn feasibility_check(v: u32, _t: u32, a: u32, _d: u32, ) -> Weight {
		Weight::from_ref_time(5_034_507_000 as u64)
			// Standard Error: 16_874
			.saturating_add(Weight::from_ref_time(697_758 as u64).saturating_mul(v as u64))
			// Standard Error: 50_006
			.saturating_add(Weight::from_ref_time(2_234_039 as u64).saturating_mul(a as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
	}
}
