use serde::{Deserialize, Serialize};

/// Example configuration section.
///
/// Delete this and replace it with your actual configuration structs.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NearConfig {
    /// near settings config
    pub near_settings: NearSettings,
    /// near smart contract config
    pub contracts: Contracts,
    /// appchain config
    #[serde(default = "Vec::new", skip_serializing_if = "Vec::is_empty")]
    pub appchain_settings: Vec<AppchainSetting>,
    /// relayer near account config
    pub relayer_near_account: RelayerNearAccount,
}

impl Default for NearConfig {
    fn default() -> Self {
        todo!()
    }
}


/// near settings
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NearSettings {
    ///
    pub near_env: String,
    ///
    pub near_node_url: String,
    ///
    pub archival_near_node_url: String,
    ///
    pub wallet_url: String,
    ///
    pub helper_url: String,
}

///
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Contracts {
    ///
    pub registry_contract: String,
    ///
    pub dao_contract_id: String,
    ///
    pub oct_token_contract_id: String,
}

///
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AppchainSetting {
    ///
    pub appchain_name: String,
    ///
    pub appchain_id: String,
    ///
    pub subql_endpoint: String,
    ///
    pub ws_rpc_endpoint: String,
}

///
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct  RelayerNearAccount {
    ///
    pub id: String,
    ///
    pub private_key: String,
}

