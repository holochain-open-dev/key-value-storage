use hc_utils::WrappedEntryHash;
use hdk3::prelude::*;

pub mod handlers;

#[hdk_entry(id = "store", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug)]
pub struct Store {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, SerializedBytes, Clone, new)]
#[serde(rename_all = "camelCase")]
pub struct StoreWithHash {
    pub store_hash: WrappedEntryHash,
    pub store: Store,
}

#[derive(Debug, Serialize, Deserialize, SerializedBytes, Clone, new)]
pub struct StoreInput {
    pub store: String,
}

