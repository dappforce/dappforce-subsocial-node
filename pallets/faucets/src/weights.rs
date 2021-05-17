//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.1

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

pub struct WeightInfo;
impl crate::WeightInfo for WeightInfo {
	fn add_faucet() -> Weight {
		(85_067_000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn update_faucet() -> Weight {
		(70_796_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn remove_faucets() -> Weight {
		(57_189_000 as Weight)
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn drip() -> Weight {
		(215_905_000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
}

impl crate::WeightInfo for () {
	fn add_faucet() -> u64 {
		0
	}

	fn update_faucet() -> u64 {
		0
	}

	fn remove_faucets() -> u64 {
		0
	}

	fn drip() -> u64 {
		0
	}
}

