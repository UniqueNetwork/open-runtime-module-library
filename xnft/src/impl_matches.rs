use crate::*;
use frame_support::traits::Incrementable;
use xcm::v3::Fungibility;
use xcm_executor::traits::{Error as MatchError, MatchesNonFungibles};

impl<T: Config> MatchesNonFungibles<CollectionIdOf<T>, ItemIdOf<T>> for Pallet<T>
where
	ItemIdOf<T>: MaxEncodedLen + Incrementable,
	CollectionIdOf<T>: MaxEncodedLen,
{
	fn matches_nonfungibles(
		foreign_asset: &MultiAsset,
	) -> core::result::Result<(CollectionIdOf<T>, ItemIdOf<T>), MatchError> {
		let Fungibility::NonFungible(asset_instance) = foreign_asset.fun else {
			return Err(MatchError::AssetNotHandled);
		};
		let asset = Self::assets(foreign_asset.id).ok_or(MatchError::AssetNotHandled)?;
		let item = Self::items(&asset, asset_instance).unwrap_or(Self::get_next_item_of(&asset)?);
		Ok((asset, item))
	}
}

impl<T: Config> Pallet<T>
where
	ItemIdOf<T>: MaxEncodedLen + Incrementable,
	CollectionIdOf<T>: MaxEncodedLen,
{
	pub fn get_next_item_of(collection_id: &CollectionIdOf<T>) -> Result<ItemIdOf<T>, MatchError> {
		let item = <NextItemId<T>>::get(collection_id)
			.unwrap_or(<ItemIdOf<T>>::initial_value().ok_or(MatchError::InstanceConversionFailed)?);
		<NextItemId<T>>::set(
			collection_id,
			Some(item.increment().ok_or(MatchError::InstanceConversionFailed)?),
		);
		Ok(item)
	}
}
