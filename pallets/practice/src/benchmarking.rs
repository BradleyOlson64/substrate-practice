//! Benchmarking setup for pallet-practice
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as PalletPractice;
use frame_benchmarking::v2::*;
use frame_system::{ ensure_signed, Origin, RawOrigin };
use sp_runtime::MultiSignature;

const SEED: u32 = 0;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn mint() {
		let value = 100u32.into();
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		mint(RawOrigin::Signed(caller.clone()), value);

		assert_eq!(AccountToBalance::<T>::get(&caller), value);
	}

	#[benchmark]
	fn transfer() {
		let caller: T::AccountId = account("caller", 0, SEED);
		let caller_origin = RawOrigin::Signed(caller.clone());
        let recipient: T::AccountId = account("recipient", 0, SEED);
		let value: <T as pallet::Config>::Balance = 100u32.into();
        // Set up accounts and place a balance in one
		AccountToBalance::<T>::insert(caller, value);

		#[extrinsic_call]
		transfer(caller_origin, recipient.clone(), value);

		assert_eq!(AccountToBalance::<T>::get(&recipient), value);
	}

	impl_benchmark_test_suite!(PalletPractice, crate::mock::new_test_ext(), crate::mock::Test);
}