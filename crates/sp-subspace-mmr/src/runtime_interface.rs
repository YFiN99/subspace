#[cfg(feature = "std")]
use crate::host_functions::SubspaceMmrExtension;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;
#[cfg(feature = "std")]
use sp_externalities::ExternalitiesExt;
use sp_mmr_primitives::EncodableOpaqueLeaf;
use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

/// MMR related runtime interface
#[runtime_interface]
pub trait SubspaceMmrRuntimeInterface {
    /// Returns the MMR leaf.
    fn get_mmr_leaf_data(&mut self, consensus_block_hash: H256) -> Option<LeafData> {
        self.extension::<SubspaceMmrExtension>()
            .expect("No `SubspaceMmrExtension` associated for the current context!")
            .get_mmr_leaf_data(consensus_block_hash)
    }
}

/// Leaf data sent back from host function.
#[derive(Debug, Decode, Encode, TypeInfo, PartialEq, Eq, Clone, Default)]
pub struct LeafData {
    pub state_root: H256,
    pub extrinsics_root: H256,
}

#[runtime_interface]
pub trait DomainMmrRuntimeInterface {
    /// Verifies the given MMR proof using the leaves provided
    fn verify_mmr_proof(
        &mut self,
        leaves: Vec<EncodableOpaqueLeaf>,
        encoded_proof: Vec<u8>,
    ) -> bool {
        self.extension::<SubspaceMmrExtension>()
            .expect("No `SubspaceMmrExtension` associated for the current context!")
            .verify_mmr_proof(leaves, encoded_proof)
    }
}
