
//! Autogenerated weights for pallet_poe
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-28, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/node-template
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet-poe
// --extrinsic
// create_claim_benchmark
// --steps
// 50
// --repeat
// 20
// --output
// ./pallets/poe/src/weights.rs
// --template
// ./pallets/frame-weight-template.hbs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight,constants::RocksDbWeight}};
use sp_std::marker::PhantomData;
use frame_benchmarking::Vec;


/// Weight functions needed for pallet_template.
pub trait WeightInfo {
	fn create_claim_benchmark(s: Vec<u8>, ) -> Weight;
}

/// Weight functions for pallet_poe.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create_claim_benchmark(_s: Vec<u8>,) -> Weight {
		(54_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_claim_benchmark(_s: Vec<u8>,) -> Weight {
		(54_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}