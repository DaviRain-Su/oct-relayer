
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AppChianConfig {
    /// appchain id
    pub appchain_id: String,
    /// start block height
    pub start_block_height: u64,
    ///
    pub update_start_min_interval: u64,
}

impl Default for AppChianConfig {
    fn default() -> Self {
        todo!()
    }
}