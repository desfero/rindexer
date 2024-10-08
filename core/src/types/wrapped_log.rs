use ethers::prelude::{U256, U64};
pub use ethers::types::{Log as EthersLog};
use serde::{Deserialize, Serialize};

/// TODO: This is a temporary type until we migrate to alloy
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WrappedLog {
    #[serde(flatten)]
    pub inner: EthersLog,

    #[serde(rename = "blockTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_timestamp: Option<U256>,
}