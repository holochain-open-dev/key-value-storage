use hc_utils::WrappedHeaderHash;
use hdk::prelude::*;

pub mod handlers;

#[hdk_entry(
    id = "item",
    required_validations = 5,
    visibility = "private"
)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Item {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemWithHash {
    pub item_hash: WrappedHeaderHash,
    pub item: Item,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemInput {
    pub store: String,
    pub key: String,
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, new)]
pub struct KeyInput {
    pub store: String,
    pub key: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, new)]
pub struct IndexInput {
    pub store: String,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug, derive_more::From)]
pub struct KeyList {
    pub keys: Vec<Key>,
}

#[derive(Clone, Serialize, Deserialize, Debug, new)]
pub struct Key {
    pub key: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, new)]
pub struct Length {
    pub length: usize,
}
