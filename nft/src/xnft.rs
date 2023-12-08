use core::marker::PhantomData;
use core::ops::Deref;

use sp_std::vec;

use frame_support::{pallet_prelude::DispatchResult, Parameter};
use parity_scale_codec::MaxEncodedLen;
use sp_runtime::{
	traits::{Get, Member},
	DispatchError,
};
use xcm::v3::prelude::*;

use pallet_xnft::traits::{DerivativeWithdrawal, IntoXcmError, NftPallet, PalletError};

use crate::{Config, Error, Pallet};
pub struct XnftAdapter<T, CollectionId, TokenId, DerivativeClassData, DerivativeTokenData>(
	PhantomData<(T, CollectionId, TokenId, DerivativeClassData, DerivativeTokenData)>,
)
where
	T: Config,
	CollectionId:
		Deref<Target = T::ClassId> + From<T::ClassId> + Member + Parameter + MaxEncodedLen + TryFrom<Junction>,
	TokenId:
		Deref<Target = T::TokenId> + From<T::TokenId> + Member + Parameter + MaxEncodedLen + TryFrom<AssetInstance>,
	DerivativeClassData: Get<T::ClassData>,
	DerivativeTokenData: Get<T::TokenData>;

impl<T, CollectionId, TokenId, DerivativeClassData, DerivativeTokenData> NftPallet<T>
	for XnftAdapter<T, CollectionId, TokenId, DerivativeClassData, DerivativeTokenData>
where
	T: Config,
	CollectionId:
		Deref<Target = T::ClassId> + From<T::ClassId> + Member + Parameter + MaxEncodedLen + TryFrom<Junction>,
	TokenId:
		Deref<Target = T::TokenId> + From<T::TokenId> + Member + Parameter + MaxEncodedLen + TryFrom<AssetInstance>,
	DerivativeClassData: Get<T::ClassData>,
	DerivativeTokenData: Get<T::TokenData>,
{
	type CollectionId = CollectionId;
	type TokenId = TokenId;
	type PalletDispatchErrors = Error<T>;

	fn create_derivative_collection(owner: &T::AccountId) -> Result<Self::CollectionId, DispatchError> {
		<Pallet<T>>::create_class(owner, vec![], DerivativeClassData::get()).map(Into::into)
	}

	fn mint_derivative(
		collection_id: &Self::CollectionId,
		to: &<T as frame_system::Config>::AccountId,
	) -> Result<Self::TokenId, DispatchError> {
		<Pallet<T>>::mint(to, *collection_id.clone(), vec![], DerivativeTokenData::get()).map(Into::into)
	}

	fn withdraw_derivative(
		collection_id: &Self::CollectionId,
		token_id: &Self::TokenId,
		from: &T::AccountId,
	) -> Result<DerivativeWithdrawal, DispatchError> {
		<Pallet<T>>::burn(from, (*collection_id.deref(), *token_id.deref())).map(|()| DerivativeWithdrawal::Burned)
	}

	fn transfer(
		collection_id: &Self::CollectionId,
		token_id: &Self::TokenId,
		from: &T::AccountId,
		to: &T::AccountId,
	) -> DispatchResult {
		<Pallet<T>>::transfer(from, to, (*collection_id.deref(), *token_id.deref()))
	}
}

impl<T: Config> PalletError for Error<T> {
	type Pallet = Pallet<T>;
}
impl<T: Config> IntoXcmError for Error<T> {
	fn into_xcm_error(self) -> XcmError {
		match self {
			Error::ClassNotFound => XcmError::AssetNotFound,
			Error::NoPermission => XcmError::NoPermission,
			_ => XcmError::FailedToTransactAsset(self.into()),
		}
	}
}
