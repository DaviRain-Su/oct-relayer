//! OctRelayer Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

mod appchain;
mod near;
use appchain::AppChianConfig;
use near::NearConfig;

use serde::{Deserialize, Serialize};

/// OctRelayer Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OctRelayerConfig {
    /// An example configuration section
    pub near: NearConfig,
    ///
    pub appchian: AppChianConfig,
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on OctRelayerConfig instead.
impl Default for OctRelayerConfig {
    fn default() -> Self {
        todo!()
    }
}

