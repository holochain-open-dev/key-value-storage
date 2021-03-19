use hc_utils::WrappedEntryHash;
use hdk::prelude::*;

pub mod handlers;

#[hdk_entry(
    id = "store",
    required_validations = 5,
    visibility = "private"
)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Store {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, new)]
#[serde(rename_all = "camelCase")]
pub struct StoreWithHash {
    pub store_hash: WrappedEntryHash,
    pub store: Store,
}

#[derive(Debug, Serialize, Deserialize, Clone, new)]
pub struct StoreInput {
    pub store: String,
}
