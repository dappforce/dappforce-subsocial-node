//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.1

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

pub struct WeightInfo;
impl crate::WeightInfo for WeightInfo {
	fn follow_space() -> Weight {
		(244_719_000 as Weight)
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(7 as Weight))
	}
	fn unfollow_space() -> Weight {
		(272_821_000 as Weight)
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(7 as Weight))
	}
}

impl crate::WeightInfo for () {
	fn follow_space() -> u64 {
		0
	}

	fn unfollow_space() -> u64 {
		0
	}
}
