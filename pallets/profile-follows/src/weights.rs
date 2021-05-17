//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.1

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

pub struct WeightInfo;
impl crate::WeightInfo for WeightInfo {
	fn follow_account() -> Weight {
		(224_251_000 as Weight)
			.saturating_add(DbWeight::get().reads(6 as Weight))
			.saturating_add(DbWeight::get().writes(6 as Weight))
	}
	fn unfollow_account() -> Weight {
		(267_392_000 as Weight)
			.saturating_add(DbWeight::get().reads(6 as Weight))
			.saturating_add(DbWeight::get().writes(6 as Weight))
	}
}

impl crate::WeightInfo for () {
	fn follow_account() -> u64 {
		0
	}

	fn unfollow_account() -> u64 {
		0
	}
}