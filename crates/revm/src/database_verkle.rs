// use crate::primitives::alloy_primitives::{BlockNumber, StorageKey, StorageValue};
// use alloy_primitives::{Address, B256, U256};
// use core::ops::{Deref, DerefMut};
// use reth_primitives::Account;
// use reth_storage_errors::provider::{ProviderError, ProviderResult};
// use revm::{
//     db::VerkleDatabaseRef,
//     primitives::{AccountInfo, Bytecode},
//     VerkleDatabase,
// };
// /// A helper trait responsible for providing state necessary for EVM execution.
// ///
// /// This serves as the data layer for [`Database`].
// pub trait VerkleEvmStateProvider: Send + Sync {

//     /// Get verkle leaf node for the given key.
//     fn leaf_node(& self, key: B256) -> ProviderResult<Option<B256>>;
// }

// // Blanket implementation of EvmStateProvider for any type that implements StateProvider.
// impl<T: reth_storage_api::VerkleStateProvider> VerkleEvmStateProvider for T {

//     fn leaf_node(&self, key: B256) -> ProviderResult<Option<B256>> {
//         <T as reth_storage_api::VerkleStateProvider>::leaf_node(self, key)
//     }
// }

// /// A [Database] and [`DatabaseRef`] implementation that uses [`EvmStateProvider`] as the underlying
// /// data source.
// #[derive(Debug, Clone)]
// pub struct VerkleStateProviderDatabase<DB>(pub DB);

// impl<DB> VerkleStateProviderDatabase<DB> {
//     /// Create new State with generic `StateProvider`.
//     pub const fn new(db: DB) -> Self {
//         Self(db)
//     }

//     /// Consume State and return inner `StateProvider`.
//     pub fn into_inner(self) -> DB {
//         self.0
//     }
// }

// impl<DB> Deref for VerkleStateProviderDatabase<DB> {
//     type Target = DB;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<DB> DerefMut for VerkleStateProviderDatabase<DB> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl<DB: VerkleEvmStateProvider> Database for VerkleStateProviderDatabase<DB> {
//     type Error = ProviderError;

//     /// Retrieves the leaf node for a given key.
//     ///
//     /// Returns `Ok` with `Some(B256)` if the account exists,
//     /// `None` if it doesn't, or an error if encountered.
//     fn get_leaf(&mut self, key: B256) -> Result<Option<B256>, Self::Error> {
//         DatabaseRef::get_leaf_ref(self, key)
//     }
// }

// impl<DB: EvmStateProvider> DatabaseRef for StateProviderDatabase<DB> {
//     type Error = <Self as Database>::Error;

//     /// Retrieves the leaf node for a given key.
//     ///
//     /// Returns `Ok` with `Some(B256)` if the account exists,
//     /// `None` if it doesn't, or an error if encountered.
//     fn get_leaf_ref(&self, key: B256) -> Result<Option<B256>, Self::Error> {
//         Ok(self.0.get_leaf(key)?)
//     }
// }
