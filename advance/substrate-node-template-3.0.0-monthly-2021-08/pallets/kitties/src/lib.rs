#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

extern crate frame_support;
extern crate frame_system;

/// hello kitty
/// 运行效果： https://www.awesomescreenshot.com/video/4968603?key=f0b4d770c81f67d52a1d00174d55e9dc
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

//把数据类型暴露出去
pub use pallet::*;


#[frame_support::pallet]
pub mod pallet {

	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
						traits::Randomness,
						traits::Currency,
						traits::ReservableCurrency,
						traits::ExistenceRequirement
	};
	use frame_system::pallet_prelude::*;
	use codec::{Encode,Decode};
	use sp_io::hashing::blake2_128;
	use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded, One};

	//use sp_std::prelude::*;

	/*
	浏览器的setting/json里要加上
	{
	  "KittyIndex": "u32",
	  "Kitty": "[u8;16]"
	}
	*/

	#[derive(Encode,Decode)]
	pub struct Kitty(pub [u8;16]);

	type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// 该定义被移动到 runtime里
	// type KittyIndex =u32;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

		// 定义 KittyIndex 类型，要求实现指定的 trait
		type KittyIndex: Parameter + Member + AtLeast32BitUnsigned  + Default + Copy;

		// 创建Kitty需要质押数量
		type KittyReserve:Get<BalanceOf<Self>>;

		// Currency 类型，用于质押等于资产相关的操作
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// when a kitty was created, the message will be sent
		/// 当一只kitty被创建的时候，会发出这个消息
		KittyCreate(T::AccountId, T::KittyIndex),

		/// when a kitty was give to her/him, the message will be sent
		/// 当一只kitty被转移给别人的时候，会发出这个消息
		KittyTransfer(T::AccountId, T::AccountId, T::KittyIndex),

		KittyForSale(T::AccountId, T::KittyIndex, Option<BalanceOf<T>>),
		KittySaleOut(T::AccountId, T::KittyIndex, Option<BalanceOf<T>>),
	}

	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	pub type KittiesCount<T: Config> = StorageValue<_, T::KittyIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<Kitty>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_prices)]
	pub type KittyPrices<T: Config> =
	StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<BalanceOf<T>>, ValueQuery>;

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		///数量太多，溢出
		KittiesCountOverflow,
		NotOwner,
		SameParentIndex,
		InvalidKittyIndex,
		MoneyNotEnough,
		AlreadyOwned,
		NotForSale,
	}


	#[pallet::call]
	impl<T:Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// 质押资产
			T::Currency::reserve(&who, T::KittyReserve::get())
				.map_err(|_| Error::<T>::MoneyNotEnough)?;

			let kitty_id = Self::next_kitty_id()?;

			/*let kitty_id = match Self::kitties_count() {
				Some(id) => {
					ensure!(id!=KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				},
				None => {
					0
				}
			};*/

			let dna = Self::random_value(&who);

			Kitties::<T>::insert(kitty_id,Some(Kitty(dna)));

			Owner::<T>::insert(kitty_id, Some(who.clone()));

			KittiesCount::<T>::put(kitty_id+One::one());

			Self::deposit_event(Event::KittyCreate(who,kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(origin: OriginFor<T>, her: T::AccountId, kitty_id: T::KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id), Error::<T>::NotOwner);

			ensure!(
                Some(who.clone()) != Some(her.clone()),
                Error::<T>::AlreadyOwned
            );

			// 新拥有者质押资产
			T::Currency::reserve(&her, T::KittyReserve::get())
				.map_err(|_| Error::<T>::MoneyNotEnough)?;
			// 解除原质押资产
			T::Currency::unreserve(&who, T::KittyReserve::get());

			Owner::<T>::insert(kitty_id, Some(her.clone()));

			Self::deposit_event(Event::KittyTransfer(who,her,kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn breed(origin: OriginFor<T>, kitty_id_mom: T::KittyIndex, kitty_id_dad: T::KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(kitty_id_mom != kitty_id_dad, Error::<T>::SameParentIndex);

			let kitty_d=Self::kitties(kitty_id_dad).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty_m=Self::kitties(kitty_id_mom).ok_or(Error::<T>::InvalidKittyIndex)?;

			/*let kitty_child = match Self::kitties_count() {
				Some(id) => {
					ensure!(id!=KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				},
				None => {
					0
				}
			};*/

			let kitty_child = Self::next_kitty_id()?;

			let dna_mom= kitty_d.0;
			let dna_dad=kitty_m.0;

			let selector = Self::random_value(&who);
			let mut new_dna =[0u8; 16];

			for i in 0..dna_dad.len(){
				new_dna[i]= (selector[i] & dna_dad[i]) | (!selector[i] & dna_mom[i]);
			}

			// 质押资产
			T::Currency::reserve(&who, T::KittyReserve::get())
				.map_err(|_| Error::<T>::MoneyNotEnough)?;

			Kitties::<T>::insert(kitty_child,Some(Kitty(new_dna)));

			Owner::<T>::insert(kitty_child, Some(who.clone()));

			KittiesCount::<T>::put(kitty_child+One::one());


			Self::deposit_event(Event::KittyCreate(who, kitty_child));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn buy(origin: OriginFor<T>, kitty_id: T::KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty_owner = Owner::<T>::get(kitty_id).ok_or(Error::<T>::NotOwner)?;
			let kitty_price = KittyPrices::<T>::get(kitty_id).ok_or(Error::<T>::NotForSale)?;
			ensure!(
                Some(who.clone()) != Some(kitty_owner.clone()),
                Error::<T>::AlreadyOwned
            );

			//转账（购买）
			T::Currency::transfer(
				&who,
				&kitty_owner,
				kitty_price,
				ExistenceRequirement::KeepAlive,
			)?;

			// 新拥有者质押资产
			T::Currency::reserve(&who, T::KittyReserve::get())
				.map_err(|_| Error::<T>::MoneyNotEnough)?;
			// 解除原质押资产
			T::Currency::unreserve(&kitty_owner, T::KittyReserve::get());

			//更改拥有人
			Owner::<T>::insert(kitty_id, Some(who.clone()));

			//移除挂售
			KittyPrices::<T>::remove(kitty_id);

			Self::deposit_event(Event::KittySaleOut(who, kitty_id, Some(kitty_price)));
			Ok(())
		}


		#[pallet::weight(0)]
		pub fn sale(
			origin: OriginFor<T>,
			kitty_id: T::KittyIndex,
			sale_price: Option<BalanceOf<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(
                Some(who.clone()) == Owner::<T>::get(kitty_id),
                Error::<T>::NotOwner
            );

			KittyPrices::<T>::insert(kitty_id, sale_price);

			Self::deposit_event(Event::KittyForSale(who, kitty_id, sale_price));
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

		// 获取当前Kitty_id (从0开始)
		fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex, DispatchError> {
			let kitty_id = Self::kitties_count();
			//ensure!(kitty_id!=T::KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
			if kitty_id == T::KittyIndex::max_value() {
				return Err(Error::<T>::KittiesCountOverflow.into());
			}
			Ok(kitty_id)
		}
	}
}
