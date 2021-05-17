//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.1

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

pub struct WeightInfo;
impl crate::WeightInfo for WeightInfo {
	fn create_post_reaction() -> Weight {
		(244_477_000 as Weight)
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(5 as Weight))
	}
	fn update_post_reaction() -> Weight {
		(148_702_000 as Weight)
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn delete_post_reaction() -> Weight {
		(153_318_000 as Weight)
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
}

impl crate::WeightInfo for () {
	fn create_post_reaction() -> u64 {
		0
	}

	fn update_post_reaction() -> u64 {
		0
	}

	fn delete_post_reaction() -> u64 {
		0
	}
}
