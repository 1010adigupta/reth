use alloy_primitives::{
    keccak256,
    Address,
    B256,
    U256,
};
use reth_primitives::Bytecode;
use reth_storage_api::
    VerkleStateProvider;
use reth_storage_errors::provider::ProviderResult;
use verkle_trie::{database::memory_db::MemoryDb, Trie, VerkleConfig, TrieTrait, Value};
use verkle_spec::{code::{ chunkify_code, Code }, Header, Hasher, Address32, Storage};
use ipa_multipoint::committer::DefaultCommitter;

/// Default hasher implementation for testing purposes
#[derive(Debug)]
pub struct DefaultHasher;

impl Hasher for DefaultHasher {}

/// Mock verkle state for testing
#[derive(Debug, Clone)]
pub struct VerkleStateProviderTest {
    trie: Trie<MemoryDb, DefaultCommitter>,
}

impl VerkleStateProviderTest {
    /// Creates a new VerkleStateProviderTest instance with an empty trie
    #[inline]
    pub fn new() -> Self {
        Self { 
            trie: Trie::new(VerkleConfig::new(MemoryDb::default()))
        }
    }

    /// Creates a new account with the given address and balance
    pub fn create_account(&mut self, address: Address, balance: U256) -> ProviderResult<()> {
        let header = Header::new::<DefaultHasher>(alloy_addr20_to_addr32(address));
        self.trie.insert([
            (header.version().0, alloy_to_trie_value(U256::ZERO)),
            (header.balance().0, alloy_to_trie_value(balance)),
            (header.nonce().0, alloy_to_trie_value(U256::ZERO)),
        ].into_iter());
        Ok(())
    }

    /// Inserts code for an account
    pub fn insert_code(&mut self, address: Address, code: Option<Bytecode>) -> ProviderResult<()> {
        let address_32 = alloy_addr20_to_addr32(address);
        let header = Header::new::<DefaultHasher>(address_32);

        match code {
            None => self.trie.insert_single(header.code_keccak().0, KECCAK_EMPTY.0),
            Some(code) => {
                // Convert directly to bytes, avoiding hex encoding/decoding
                let code_bytes = code.bytes().to_vec();
                let code_hash = keccak256(&code_bytes);
                
                // Insert code metadata
                self.trie.insert([
                    (header.code_keccak().0, code_hash.0),
                    (header.code_size().0, alloy_to_trie_value(U256::from(code_bytes.len()))),
                ].into_iter());

                // Insert chunked code
                self.trie.insert(
                    chunkify_code(code_bytes)
                        .into_iter()
                        .enumerate()
                        .map(|(chunk_id, code_chunk)| (
                            Code::new::<DefaultHasher>(
                                address_32,
                                verkle_spec::U256::from(chunk_id)
                            ).code_chunk().0,
                            code_chunk
                        ))
                );
            }
        }

        Ok(())
    }

    /// Sets a storage slot value for an account
    pub fn set_storage(&mut self, address: Address, slot: U256, value: B256) -> ProviderResult<()> {
        // Convert slot to verkle_spec::U256 directly from bytes to avoid extra conversions
        let key = Storage::new::<DefaultHasher>(
            alloy_addr20_to_addr32(address),
            verkle_spec::U256::from(slot.to_be_bytes()),
        )
        .storage_slot()
        .0;
        
        self.trie.insert_single(key, value.0);
        Ok(())
    }

}

impl VerkleStateProvider for VerkleStateProviderTest {
    
    /// Retrieves the value of a leaf node by its key
    fn leaf_node(&self, key: B256) -> ProviderResult<Option<B256>> {
        Ok(self.trie.get(key.into()).map(|v| B256::from(v)))
    }
}

/// Converts a 20-byte address to a 32-byte address by zero-padding the first 12 bytes
/// 
/// # Arguments
/// * `address` - The 20-byte address to convert
fn alloy_addr20_to_addr32(address: Address) -> Address32 {
    let bytes20: [u8; 20] = address.into();

    let mut bytes32: [u8; 32] = [0u8; 32];
    bytes32[12..].copy_from_slice(&bytes20);

    Address32::from(bytes32)
}

/// Converts a U256 value to a trie Value by converting to little-endian bytes
/// 
/// # Arguments
/// * `u256` - The U256 value to
fn alloy_to_trie_value(u256: U256) -> Value {
    let mut value = Value::default();
    value.copy_from_slice(&u256.to_le_bytes::<32>());
    value
}

/// Get the KECCAK (i.e. Keccak) hash of the empty bytes string.
pub const KECCAK_EMPTY: B256 = B256::new([
    0xc5, 0xd2, 0x46, 0x01, 0x86, 0xf7, 0x23, 0x3c, 
    0x92, 0x7e, 0x7d, 0xb2, 0xdc, 0xc7, 0x03, 0xc0, 
    0xe5, 0x00, 0xb6, 0x53, 0xca, 0x82, 0x27, 0x3b, 
    0x7b, 0xfa, 0xd8, 0x04, 0x5d, 0x85, 0xa4, 0x70,
]);
