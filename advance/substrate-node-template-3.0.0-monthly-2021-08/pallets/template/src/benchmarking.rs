//! Benchmarking setup for pallet-template

use super::*;

/*
编译：
cargo build --features runtime-benchmarks --release

运行
 ./target/release/node-template benchmark \
   --chain dev \
   --execution=wasm \
   --wasm-execution=compiled \
   --pallet=pallet-template \
   --extrinsic do_something \
   --steps=10 \
   --repeat=50



*/


#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	do_something {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}
}

impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
