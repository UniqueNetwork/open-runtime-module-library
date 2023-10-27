use crate::*;

pub type ItemIdOf<T> = <<T as Config>::NftExecutor as Inspect<<T as SystemConfig>::AccountId>>::ItemId;

pub type CollectionIdOf<T> = <<T as Config>::NftExecutor as Inspect<<T as SystemConfig>::AccountId>>::CollectionId;

pub type ExecutorOf<T> = <T as Config>::NftExecutor;
