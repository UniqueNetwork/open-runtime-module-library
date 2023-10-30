#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	ensure,
	pallet_prelude::*,
	traits::tokens::nonfungibles_v2::{Create, Inspect, Mutate, Transfer},
};
use frame_system::pallet_prelude::*;
use frame_system::Config as SystemConfig;
use scale_info::TypeInfo;
use sp_runtime::{traits::AccountIdConversion, DispatchError, DispatchResult, RuntimeDebug};
use sp_std::{boxed::Box, vec::Vec};
use xcm::v3::{AssetId, AssetInstance, MultiAsset};

pub mod impl_matches;
pub mod impl_nonfungibles;
pub mod types;
pub use pallet::*;
pub(crate) use types::*;

#[frame_support::pallet]
pub mod pallet {
	use xcm::v3::Fungibility;

	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config
	where
		ItemIdOf<Self>: MaxEncodedLen,
		CollectionIdOf<Self>: MaxEncodedLen,
	{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type NftExecutor: Create<Self::AccountId, Self::CollectionConfig>
			+ Mutate<Self::AccountId, Self::ItemConfig>
			+ Transfer<Self::AccountId>;

		type ItemConfig: Default;

		type CollectionConfig: Default;
	}

	/// Error for non-fungible-token module.
	#[pallet::error]
	pub enum Error<T> {
		AssetAlreadyRegistered,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config>
	where
		ItemIdOf<T>: MaxEncodedLen,
		CollectionIdOf<T>: MaxEncodedLen,
	{
		RegisteredAsset {
			asset_id: AssetId,
			collection_id: CollectionIdOf<T>,
		},
	}

	#[pallet::storage]
	#[pallet::getter(fn assets)]
	pub type AssetsMapping<T: Config> = StorageMap<_, Twox64Concat, AssetId, CollectionIdOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn items)]
	pub type ItemsMapping<T: Config> =
		StorageDoubleMap<_, Twox64Concat, CollectionIdOf<T>, Twox64Concat, AssetInstance, ItemIdOf<T>, OptionQuery>;

	#[pallet::storage]
	pub type NextItemId<T: Config> = StorageMap<_, Twox64Concat, CollectionIdOf<T>, ItemIdOf<T>, OptionQuery>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		ItemIdOf<T>: MaxEncodedLen + Default,
		CollectionIdOf<T>: MaxEncodedLen,
	{
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn register_asset(origin: OriginFor<T>, foreign_asset: Box<AssetId>) -> DispatchResult {
			ensure_signed(origin)?;
			ensure!(
				!<AssetsMapping<T>>::contains_key(foreign_asset.as_ref()),
				<Error<T>>::AssetAlreadyRegistered,
			);
			let collection_id =
				<ExecutorOf<T>>::create_collection(&Self::account_id(), &Self::account_id(), &Default::default())?;
			<AssetsMapping<T>>::insert(foreign_asset.as_ref(), collection_id.clone());
			Self::deposit_event(Event::RegisteredAsset {
				asset_id: *foreign_asset,
				collection_id,
			});
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T>
where
	ItemIdOf<T>: MaxEncodedLen + Default,
	CollectionIdOf<T>: MaxEncodedLen,
{
	pub fn account_id() -> T::AccountId {
		frame_support::PalletId(*b"poc_xnft").into_account_truncating()
	}
}
