// SPDX-License-Identifier: GPL-3.0-or-later


// Executed Command:
// ./target/release/poscan-consensus
// benchmark
// --chain
// dev
// --steps
// 50
// --repeat
// 20
// --pallet
// rewards
// --extrinsic
// *
// --raw
// --execution=wasm
// --wasm-execution=compiled
// --output
// runtime/src/weights/rewards.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for rewards.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> rewards::WeightInfo for WeightInfo<T> {
	fn on_initialize() -> Weight {
		(14_700_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn on_finalize() -> Weight {
		(121_300_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn unlock() -> Weight {
		(45_200_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_schedule() -> Weight {
		(32_500_000 as Weight).saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn set_lock_params() -> Weight {
		(0 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_miner_share() -> Weight {
		(0 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
