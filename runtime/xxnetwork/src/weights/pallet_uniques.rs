
//! Autogenerated weights for `pallet_uniques`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("xxnetwork-dev"), DB CACHE: 128

// Executed Command:
// ./xxnetwork-chain
// benchmark
// --chain=xxnetwork-dev
// --steps=50
// --repeat=20
// --pallet=pallet_uniques
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/pallet_uniques.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_uniques`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_uniques::WeightInfo for WeightInfo<T> {
	// Storage: Uniques Class (r:1 w:1)
	fn create() -> Weight {
		(38_944_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn force_create() -> Weight {
		(19_677_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: Uniques Attribute (r:0 w:1000)
	// Storage: Uniques ClassMetadataOf (r:0 w:1)
	// Storage: Uniques InstanceMetadataOf (r:0 w:1000)
	// Storage: Uniques Account (r:0 w:20)
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 13_000
			.saturating_add((15_526_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 13_000
			.saturating_add((921_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 13_000
			.saturating_add((954_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Account (r:0 w:1)
	fn mint() -> Weight {
		(51_126_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Account (r:0 w:1)
	fn burn() -> Weight {
		(51_767_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Account (r:0 w:2)
	fn transfer() -> Weight {
		(38_363_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:100 w:100)
	fn redeposit(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 10_000
			.saturating_add((24_323_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn freeze() -> Weight {
		(27_552_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn thaw() -> Weight {
		(27_221_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn freeze_class() -> Weight {
		(20_278_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn thaw_class() -> Weight {
		(20_038_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn transfer_ownership() -> Weight {
		(46_548_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn set_team() -> Weight {
		(21_140_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn force_asset_status() -> Weight {
		(19_286_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:1 w:1)
	fn set_attribute() -> Weight {
		(62_307_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:1 w:1)
	fn clear_attribute() -> Weight {
		(57_107_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	fn set_metadata() -> Weight {
		(47_039_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	fn clear_metadata() -> Weight {
		(46_377_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassMetadataOf (r:1 w:1)
	fn set_class_metadata() -> Weight {
		(46_467_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques ClassMetadataOf (r:1 w:1)
	fn clear_class_metadata() -> Weight {
		(41_959_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn approve_transfer() -> Weight {
		(28_364_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn cancel_approval() -> Weight {
		(28_393_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
