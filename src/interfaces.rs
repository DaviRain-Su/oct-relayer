//! interface module
//! 
use sp_runtime::Digest;
use sp_core::H256;
use binary_merkle_tree::MerkleProof;

// type hash
type Hash = H256;

///
#[derive(Debug)]
pub enum ActionType {
    ///
    UpdateState,
    ///
    PlanNewEra,
    ///
    EraPayout,
    ///
    Lock,
    ///
    BurnAsset,
}

///
#[derive(Debug)]
pub struct Action {
    ///
    pub r#type: ActionType,
    ///
    pub status: usize,
    ///
    pub fialed_at: usize,
}

///
#[derive(Debug)]
pub struct Session {
    ///
    pub height: usize,
    ///
    pub status: usize,
    ///
    pub failed_at: usize,
}

///
#[derive(Debug)]
pub struct Proof {
    ///
    pub lead_index: usize,
    ///
    pub leaf_count: usize,
    ///
    pub items: Vec<usize>,
}


///
#[derive(Debug)]
pub struct HeaderPartial {
    /// The parent hash.
    pub parent_hash: Hash,
    /// The block number.
    pub number: usize,
    /// The state trie merkle root
    pub state_trie: Hash,
    /// The merkle root of the extrinsics.
    pub extrinsics_root: Hash,
    /// A chain-specific digest of data useful for light clients or referencing auxiliary data.
    pub digest: Digest,
}

///
#[derive(Debug)]
pub struct SyncedBlock {
    ///
    pub height: usize,
    ///
    pub r#type: usize,
}

///
pub struct MessageProof {
    ///
    pub encoded_message: Vec<u8>,
    ///
    pub header: Vec<u8>,
    ///
    pub mmr_leaf: Vec<u8>,
    ///
    pub mmr_proof: Vec<u8>,
}

/// 需要确定MerkleProof的第二个范型类型
pub struct MessageProofWithLightState {
    ///
    pub signed_commitment: Vec<u8>,
    ///
    pub validator_proofs: Vec<MerkleProof<H256, Vec<u8>>>,
    ///
    pub mmr_leaf_for_mmr_root: Vec<u8>,
    ///
    pub mmr_proof_for_mmr_root: Vec<u8>,
    ///
    pub encode_messages: Vec<u8>,
    ///
    pub header: Vec<u8>,
    ///
    pub mmr_leaf_for_header: Vec<u8>,
    ///
    pub mmr_proof_for_header: Vec<u8>,
}

///
pub struct LightClientState {
    ///
    pub signed_commitment: Vec<u8>,
    ///
    pub validator_proofs: Vec<MerkleProof<H256, Vec<u8>>>,
    ///
    pub mmr_leaf: Vec<u8>,
    ///
    pub mmr_proof: Vec<u8>,
}