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
// lockdrop
// --extrinsic
// *
// --raw
// --execution=wasm
// --wasm-execution=compiled
// --output
// runtime/src/weights/lockdrop.rs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for lockdrop.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> lockdrop::WeightInfo for WeightInfo<T> {
	fn create_campaign() -> Weight {
		(30_100_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn conclude_campaign() -> Weight {
		(77_800_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_expired_child_storage() -> Weight {
		(9_500_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn lock() -> Weight {
		(44_100_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn unlock() -> Weight {
		(7_200_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}
