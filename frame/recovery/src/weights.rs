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

//! Autogenerated weights for pallet_recovery
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
// --pallet=pallet_recovery
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/recovery/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_recovery.
pub trait WeightInfo {
	fn as_recovered() -> Weight;
	fn set_recovered() -> Weight;
	fn create_recovery(n: u32, ) -> Weight;
	fn initiate_recovery() -> Weight;
	fn vouch_recovery(n: u32, ) -> Weight;
	fn claim_recovery(n: u32, ) -> Weight;
	fn close_recovery(n: u32, ) -> Weight;
	fn remove_recovery(n: u32, ) -> Weight;
	fn cancel_recovered() -> Weight;
}

/// Weights for pallet_recovery using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Recovery Proxy (r:1 w:0)
	fn as_recovered() -> Weight {
		Weight::from_ref_time(10_309_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Recovery Proxy (r:0 w:1)
	fn set_recovered() -> Weight {
		Weight::from_ref_time(16_438_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Recovery Recoverable (r:1 w:1)
	/// The range of component `n` is `[1, 9]`.
	fn create_recovery(n: u32, ) -> Weight {
		Weight::from_ref_time(32_436_000 as u64)
			// Standard Error: 2_211
			.saturating_add(Weight::from_ref_time(284_036 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	fn initiate_recovery() -> Weight {
		Weight::from_ref_time(38_463_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	/// The range of component `n` is `[1, 9]`.
	fn vouch_recovery(n: u32, ) -> Weight {
		Weight::from_ref_time(26_520_000 as u64)
			// Standard Error: 3_315
			.saturating_add(Weight::from_ref_time(416_349 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:0)
	// Storage: Recovery Proxy (r:1 w:1)
	/// The range of component `n` is `[1, 9]`.
	fn claim_recovery(n: u32, ) -> Weight {
		Weight::from_ref_time(35_164_000 as u64)
			// Standard Error: 2_271
			.saturating_add(Weight::from_ref_time(213_857 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `n` is `[1, 9]`.
	fn close_recovery(n: u32, ) -> Weight {
		Weight::from_ref_time(38_754_000 as u64)
			// Standard Error: 2_485
			.saturating_add(Weight::from_ref_time(262_163 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Recovery ActiveRecoveries (r:1 w:0)
	// Storage: Recovery Recoverable (r:1 w:1)
	/// The range of component `n` is `[1, 9]`.
	fn remove_recovery(n: u32, ) -> Weight {
		Weight::from_ref_time(37_763_000 as u64)
			// Standard Error: 2_780
			.saturating_add(Weight::from_ref_time(230_373 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Recovery Proxy (r:1 w:1)
	fn cancel_recovered() -> Weight {
		Weight::from_ref_time(18_889_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Recovery Proxy (r:1 w:0)
	fn as_recovered() -> Weight {
		Weight::from_ref_time(10_309_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: Recovery Proxy (r:0 w:1)
	fn set_recovered() -> Weight {
		Weight::from_ref_time(16_438_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Recovery Recoverable (r:1 w:1)
	/// The range of component `n` is `[1, 9]`.
	fn create_recovery(n: u32, ) -> Weight {
		Weight::from_ref_time(32_436_000 as u64)
			// Standard Error: 2_211
			.saturating_add(Weight::from_ref_time(284_036 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	fn initiate_recovery() -> Weight {
		Weight::from_ref_time(38_463_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	/// The range of component `n` is `[1, 9]`.
	fn vouch_recovery(n: u32, ) -> Weight {
		Weight::from_ref_time(26_520_000 as u64)
			// Standard Error: 3_315
			.saturating_add(Weight::from_ref_time(416_349 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:0)
	// Storage: Recovery Proxy (r:1 w:1)
	/// The range of component `n` is `[1, 9]`.
	fn claim_recovery(n: u32, ) -> Weight {
		Weight::from_ref_time(35_164_000 as u64)
			// Standard Error: 2_271
			.saturating_add(Weight::from_ref_time(213_857 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `n` is `[1, 9]`.
	fn close_recovery(n: u32, ) -> Weight {
		Weight::from_ref_time(38_754_000 as u64)
			// Standard Error: 2_485
			.saturating_add(Weight::from_ref_time(262_163 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Recovery ActiveRecoveries (r:1 w:0)
	// Storage: Recovery Recoverable (r:1 w:1)
	/// The range of component `n` is `[1, 9]`.
	fn remove_recovery(n: u32, ) -> Weight {
		Weight::from_ref_time(37_763_000 as u64)
			// Standard Error: 2_780
			.saturating_add(Weight::from_ref_time(230_373 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Recovery Proxy (r:1 w:1)
	fn cancel_recovered() -> Weight {
		Weight::from_ref_time(18_889_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
