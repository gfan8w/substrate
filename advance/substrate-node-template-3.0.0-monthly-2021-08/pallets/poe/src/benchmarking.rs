//! Benchmarking setup for pallet-poe

use super::*;

/*
编译：
cargo build --features runtime-benchmarks --release

运行
 ./target/release/node-template benchmark \
   --chain dev \
   --execution=wasm \
   --wasm-execution=compiled \
   --pallet=pallet-poe \
   --extrinsic create_claim_benchmark \
   --steps=10 \
   --repeat=50

运行结果：
$ ./target/release/node-template benchmark    --chain dev    --execution=wasm    --wasm-execution=compiled    --pallet=pallet-poe    --extrinsic create_claim_benchmark    --steps=10    --repeat=50
Pallet: "pallet_poe", Extrinsic: "create_claim_benchmark", Lowest values: [], Highest values: [], Steps: [10], Repeat: 50
Median Slopes Analysis
========
-- Extrinsic Time --

Model:
Time ~=       38
              µs

Reads = 1
Writes = 1
Min Squares Analysis
========
-- Extrinsic Time --

Model:
Time ~=       38
              µs

Reads = 1
Writes = 1


生成weights.rs
./target/release/node-template benchmark \
--chain dev \
--execution=wasm \
--wasm-execution=compiled \
--pallet pallet-poe \
--extrinsic create_claim_benchmark \
--steps 50 \
--repeat 20 \
--output ./pallets/poe/src/weights.rs --template ./pallets/frame-weight-template.hbs






*/


#[allow(unused)]
use crate::Pallet as Poe;
use crate::{Error, Proofs};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use frame_benchmarking::Vec;

benchmarks! {
	create_claim_benchmark {
		let mut s: Vec<u8> = Vec::new();
		s.push(1);
		s.push(1);
		let mut r: Vec<u8> = Vec::new();
		r.push(1);
		r.push(1);

		let caller: T::AccountId = whitelisted_caller();
		let callerb = caller.clone();
	}: create_claim(RawOrigin::Signed(caller),s)
	verify {
		assert_eq!(Proofs::<T>::get(&r),
			Some((callerb,frame_system::Pallet::<T>::block_number())
			)
		);
	}
}

impl_benchmark_test_suite!(Poe, crate::mock::new_test_ext(), crate::mock::Test);


/*#[cfg(test)]
mod test {
	use super::*;
	use crate::{new_test_ext,Test};

	#[test]
	fn test_benchmark(){
		new_test_ext().execute_with(||{
			assert_ok!(test_benchmark_create_claim_benchmark::<Test>())
		});
	}

}*/



