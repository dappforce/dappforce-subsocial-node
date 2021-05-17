//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.1

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

pub struct WeightInfo;
impl crate::WeightInfo for WeightInfo {
	fn create_space() -> Weight {
		(279_346_000 as Weight)
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(8 as Weight))
	}
	fn update_space() -> Weight {
		(202_097_000 as Weight)
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
}
impl crate::WeightInfo for () {
	fn create_space() -> u64 { 0 }

	fn update_space() -> u64 { 0 }
}