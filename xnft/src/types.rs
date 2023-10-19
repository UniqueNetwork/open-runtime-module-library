use crate::*;

pub type ItemIdOf<T> = <<T as Config>::NftExecutor as Inspect<<T as SystemConfig>::AccountId>>::ItemId;

pub type CollectionIdOf<T> = <<T as Config>::NftExecutor as Inspect<<T as SystemConfig>::AccountId>>::CollectionId;

pub type ExecutorOf<T> = <T as Config>::NftExecutor;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct LocalAsset<CollectionId, ItemId> {
	pub collection_id: CollectionId,
	pub item_id: ItemId,
}
