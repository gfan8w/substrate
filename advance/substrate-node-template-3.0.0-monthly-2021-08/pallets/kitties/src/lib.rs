#![cfg_attr(not(feature = "std"), no_std)]

extern crate frame_support;
extern crate frame_system;

/// hello kitty
///
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

//把数据类型暴露出去
pub use pallet::*;


#[frame_support::pallet]
pub mod pallet {

	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::Randomness};
	use frame_system::pallet_prelude::*;
	use codec::{Encode,Decode};
	use sp_io::hashing::blake2_128;
	//use sp_std::prelude::*;

	#[derive(Encode,Decode)]
	pub struct Kitty(pub [u8;16]);

	type KittyIndex =u32;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreate(T::AccountId, KittyIndex),
		KittyTransfer(T::AccountId, T::AccountId, KittyIndex),
	}

	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	pub type KittiesCount<T> = StorageValue<_,u32>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyIndex, Option<Kitty>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> = StorageMap<_, Blake2_128Concat, KittyIndex, Option<T::AccountId>, ValueQuery>;


	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		///数量太多，溢出
		KittiesCountOverflow,
		NotOwner,
		SameParentIndex,
		InvalidKittyIndex,
	}


	#[pallet::call]
	impl<T:Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_id = match Self::kitties_count() {
				Some(id) => {
					ensure!(id!=KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				},
				None => {
					1
				}
			};

			let dna = Self::random_value(&who);

			Kitties::<T>::insert(kitty_id,Some(Kitty(dna)));

			Owner::<T>::insert(kitty_id, Some(who.clone()));

			KittiesCount::<T>::put(kitty_id+1);

			Self::deposit_event(Event::KittyCreate(who,kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(origin: OriginFor<T>, her: T::AccountId, kitty_id: KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id), Error::<T>::NotOwner);

			Owner::<T>::insert(kitty_id, Some(her.clone()));

			Self::deposit_event(Event::KittyTransfer(who,her,kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn breed(origin: OriginFor<T>, kitty_id_mom: KittyIndex, kitty_id_dad: KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(kitty_id_mom != kitty_id_dad, Error::<T>::SameParentIndex);

			let kitty_d=Self::kitties(kitty_id_dad).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty_m=Self::kitties(kitty_id_mom).ok_or(Error::<T>::InvalidKittyIndex)?;

			let kitty_child = match Self::kitties_count() {
				Some(id) => {
					ensure!(id!=KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				},
				None => {
					1
				}
			};

			let dna_mom= kitty_d.0;
			let dna_dad=kitty_m.0;

			let selector = Self::random_value(&who);
			let mut new_dna =[0u8; 16];

			for i in 0..dna_dad.len(){
				new_dna[i]= (selector[i] & dna_dad[i]) | (!selector[i] & dna_mom[i]);
			}

			Kitties::<T>::insert(kitty_child,Some(Kitty(new_dna)));

			Owner::<T>::insert(kitty_child, Some(who.clone()));

			KittiesCount::<T>::put(kitty_child+1);


			Self::deposit_event(Event::KittyCreate(who, kitty_child));

			Ok(())
		}

	}

	impl<T:Config> Pallet<T> {
		fn random_value(sender: &T::AccountId) ->[u8; 16] {
			let payload =(
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}
	}


}
