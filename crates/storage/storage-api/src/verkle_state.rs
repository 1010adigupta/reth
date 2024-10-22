use super::{
    AccountReader, BlockHashReader, BlockIdReader, StateProofProvider, StateRootProvider,
    StorageRootProvider,
};
use alloy_eips::{BlockId, BlockNumHash, BlockNumberOrTag};
use alloy_primitives::{Address, BlockHash, BlockNumber, StorageKey, StorageValue, B256, U256};
use auto_impl::auto_impl;
use reth_execution_types::ExecutionOutcome;
use reth_primitives::{Bytecode, KECCAK_EMPTY};
use reth_storage_errors::provider::{ProviderError, ProviderResult};

/// Type alias of boxed [`VerkleStateProvider`].
pub type VerkleStateProviderBox = Box<dyn VerkleStateProvider>;

/// An abstraction for a type that provides state data using verkle trie format.
#[auto_impl(&, Arc, Box)]
pub trait VerkleStateProvider: Send + Sync {
    /// Get verkle leaf node for the given key.
    fn leaf_node(&self, key: B256) -> ProviderResult<Option<B256>>;
}
