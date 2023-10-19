use crate::*;
pub(crate) type ExecutorOf<T> = <T as Config>::NftExecutor;

impl<T: Config> Inspect<T::AccountId> for Pallet<T>
where
	ItemIdOf<T>: MaxEncodedLen,
	CollectionIdOf<T>: MaxEncodedLen,
{
	type ItemId = ItemIdOf<T>;

	type CollectionId = CollectionIdOf<T>;

	fn owner(collection: &Self::CollectionId, item: &Self::ItemId) -> Option<T::AccountId> {
		<ExecutorOf<T>>::owner(collection, item)
	}

	fn collection_owner(collection: &Self::CollectionId) -> Option<T::AccountId> {
		<ExecutorOf<T>>::collection_owner(collection)
	}

	fn attribute(collection: &Self::CollectionId, item: &Self::ItemId, key: &[u8]) -> Option<Vec<u8>> {
		<ExecutorOf<T>>::attribute(collection, item, key)
	}

	fn custom_attribute(
		account: &T::AccountId,
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		key: &[u8],
	) -> Option<Vec<u8>> {
		<ExecutorOf<T>>::custom_attribute(account, collection, item, key)
	}

	fn system_attribute(collection: &Self::CollectionId, item: &Self::ItemId, key: &[u8]) -> Option<Vec<u8>> {
		<ExecutorOf<T>>::system_attribute(collection, item, key)
	}

	fn typed_attribute<K: Encode, V: Decode>(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		key: &K,
	) -> Option<V> {
		<ExecutorOf<T>>::typed_attribute(collection, item, key)
	}

	fn typed_custom_attribute<K: Encode, V: Decode>(
		account: &T::AccountId,
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		key: &K,
	) -> Option<V> {
		<ExecutorOf<T>>::typed_custom_attribute(account, collection, item, key)
	}

	fn typed_system_attribute<K: Encode, V: Decode>(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		key: &K,
	) -> Option<V> {
		<ExecutorOf<T>>::typed_system_attribute(collection, item, key)
	}

	fn collection_attribute(collection: &Self::CollectionId, key: &[u8]) -> Option<Vec<u8>> {
		<ExecutorOf<T>>::collection_attribute(collection, key)
	}

	fn typed_collection_attribute<K: Encode, V: Decode>(collection: &Self::CollectionId, key: &K) -> Option<V> {
		<ExecutorOf<T>>::typed_collection_attribute(collection, key)
	}

	fn can_transfer(collection: &Self::CollectionId, item: &Self::ItemId) -> bool {
		<ExecutorOf<T>>::can_transfer(collection, item)
	}
}

impl<T: Config> Transfer<T::AccountId> for Pallet<T>
where
	ItemIdOf<T>: MaxEncodedLen,
	CollectionIdOf<T>: MaxEncodedLen,
	// ItemId: MaxEncodedLen + Parameter,
	// CollectionId: MaxEncodedLen + Parameter,
{
	fn disable_transfer(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
	) -> frame_support::pallet_prelude::DispatchResult {
		<ExecutorOf<T>>::disable_transfer(collection, item)
	}

	fn enable_transfer(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
	) -> frame_support::pallet_prelude::DispatchResult {
		<ExecutorOf<T>>::enable_transfer(collection, item)
	}

	fn transfer(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		destination: &T::AccountId,
	) -> frame_support::pallet_prelude::DispatchResult {
		<ExecutorOf<T>>::transfer(collection, item, destination)
	}
}

impl<T: Config> Mutate<T::AccountId, T::ItemConfig> for Pallet<T>
where
	ItemIdOf<T>: MaxEncodedLen,
	CollectionIdOf<T>: MaxEncodedLen,
{
	fn mint_into(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		who: &T::AccountId,
		config: &T::ItemConfig,
		deposit_collection_owner: bool,
	) -> frame_support::pallet_prelude::DispatchResult {
		<ExecutorOf<T>>::mint_into(collection, item, who, config, deposit_collection_owner)
	}

	fn burn(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		maybe_check_owner: Option<&T::AccountId>,
	) -> frame_support::pallet_prelude::DispatchResult {
		<ExecutorOf<T>>::burn(collection, item, maybe_check_owner)
	}

	fn set_attribute(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		key: &[u8],
		value: &[u8],
	) -> frame_support::pallet_prelude::DispatchResult {
		<ExecutorOf<T>>::set_attribute(collection, item, key, value)
	}

	fn set_typed_attribute<K: Encode, V: Encode>(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		key: &K,
		value: &V,
	) -> frame_support::pallet_prelude::DispatchResult {
		<ExecutorOf<T>>::set_typed_attribute(collection, item, key, value)
	}

	fn set_collection_attribute(
		collection: &Self::CollectionId,
		key: &[u8],
		value: &[u8],
	) -> frame_support::pallet_prelude::DispatchResult {
		<ExecutorOf<T>>::set_collection_attribute(collection, key, value)
	}

	fn set_typed_collection_attribute<K: Encode, V: Encode>(
		collection: &Self::CollectionId,
		key: &K,
		value: &V,
	) -> frame_support::pallet_prelude::DispatchResult {
		<ExecutorOf<T>>::set_typed_collection_attribute(collection, key, value)
	}

	fn clear_attribute(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		key: &[u8],
	) -> frame_support::pallet_prelude::DispatchResult {
		<ExecutorOf<T>>::clear_attribute(collection, item, key)
	}

	fn clear_typed_attribute<K: Encode>(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		key: &K,
	) -> frame_support::pallet_prelude::DispatchResult {
		<ExecutorOf<T>>::clear_typed_attribute(collection, item, key)
	}

	fn clear_collection_attribute(
		collection: &Self::CollectionId,
		key: &[u8],
	) -> frame_support::pallet_prelude::DispatchResult {
		<ExecutorOf<T>>::clear_collection_attribute(collection, key)
	}

	fn clear_typed_collection_attribute<K: Encode>(
		collection: &Self::CollectionId,
		key: &K,
	) -> frame_support::pallet_prelude::DispatchResult {
		<ExecutorOf<T>>::clear_typed_collection_attribute(collection, key)
	}
}

impl<T: Config> Create<T::AccountId, T::CollectionConfig> for Pallet<T>
where
	ItemIdOf<T>: MaxEncodedLen,
	CollectionIdOf<T>: MaxEncodedLen,
{
	fn create_collection(
		who: &T::AccountId,
		admin: &T::AccountId,
		config: &T::CollectionConfig,
	) -> Result<Self::CollectionId, DispatchError> {
		<ExecutorOf<T>>::create_collection(who, admin, config)
	}
}
