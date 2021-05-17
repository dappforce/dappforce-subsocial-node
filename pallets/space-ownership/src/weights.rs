//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.1

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

pub struct WeightInfo;
impl crate::WeightInfo for WeightInfo {
	fn transfer_space_ownership() -> Weight {
		(85_322_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn accept_pending_ownership() -> Weight {
		(350_220_000 as Weight)
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(5 as Weight))
	}
	fn reject_pending_ownership() -> Weight {
		(93_273_000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
}

impl crate::WeightInfo for () {
	fn transfer_space_ownership() -> u64 {
		0
	}

	fn accept_pending_ownership() -> u64 {
		0
	}

	fn reject_pending_ownership() -> u64 {
		0
	}
}
