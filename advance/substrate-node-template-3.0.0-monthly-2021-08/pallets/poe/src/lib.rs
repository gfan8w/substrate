#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

extern crate frame_support;
extern crate frame_system;

/// 一个简单的开始
/// 学习写一个 poe
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

//把数据类型暴露出去
pub use pallet::*;


#[frame_support::pallet]
pub mod pallet {

	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	//use sp_std::prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type MaxClaimLength: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T:Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>;

/*	#[pallet::storage]
	#[pallet::getter(fn daattte)]
	pub type Daaate<T:Config> = StorageValue<Vec<T::AccountId>>;*/

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId, Vec<u8>),
		ClaimTransfered(T::AccountId,T::AccountId, Vec<u8>), //存证转移
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		///存证已经存在
		ProofAlreadyExist,
		///存证不存在
		ProofNotExist,
		///不是存证的拥有者
		NotClaimOwner,
		///存证太长
		ClaimTooLarge,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}


	#[pallet::call]
	impl<T:Config> Pallet<T> {

		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			/*let claim_len=claim.len();
			println!("{}",claim_len);
			if claim_len>10 {
				Err(Error::<T>::ClaimTooLarge)?;
			}*/

			//ensure!(claim.len()<=10, Error::<T>::ClaimTooLarge);

			ensure!(
                T::MaxClaimLength::get() >= claim.len() as u32,
                Error::<T>::ClaimTooLarge
            );

			Proofs::<T>::insert(&claim, (sender.clone(), frame_system::Pallet::<T>::block_number()));

			Self::deposit_event(Event::ClaimCreated(sender,claim));

			Ok(().into())

		}

		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;

			//使用ok_or,这里注销掉
			//ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ProofNotExist);

			let (owner,_) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ProofNotExist)?;

			ensure!(owner==sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(Event::ClaimRevoked(sender,claim));

			Ok(().into())

		}

		#[pallet::weight(0)]
		pub fn transfer_claim(origin: OriginFor<T>, target:T::AccountId, claim: Vec<u8>) -> DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;

			//使用ok_or,这里注销掉
			//ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ProofNotExist);

			let (owner,_) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ProofNotExist)?;

			ensure!(owner==sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Proofs::<T>::insert(&claim, (target.clone(), frame_system::Pallet::<T>::block_number()));

			Self::deposit_event(Event::ClaimTransfered(sender,target,claim));

			Ok(().into())

		}

	}
}
