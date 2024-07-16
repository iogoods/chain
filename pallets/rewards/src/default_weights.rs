// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of IO.

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

impl crate::WeightInfo for () {
	fn on_initialize() -> Weight {
		(14_800_000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn on_finalize() -> Weight {
		(121_500_000 as Weight)
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn unlock() -> Weight {
		(46_000_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn set_schedule() -> Weight {
		(32_900_000 as Weight).saturating_add(DbWeight::get().writes(4 as Weight))
	}
	fn set_lock_params() -> Weight {
		(0 as Weight).saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn set_miner_share() -> Weight {
		(0 as Weight).saturating_add(DbWeight::get().writes(1 as Weight))
	}
}
