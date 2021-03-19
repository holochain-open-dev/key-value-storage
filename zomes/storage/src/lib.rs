//! Key-Value Storage is a Holochain zome for persisting key/value pairs - a simple general
//! "database" for Holochain apps. The API is very close to the
//! [Web storage](https://html.spec.whatwg.org/multipage/webstorage.html#webstorage) specifications
//! for `localStorage` and `sessionStorage`.
//!
//! ## Multiple stores
//! You can create multiples stores/"databases".
//! There is no default store, create one before setting the first value using [create_store].
//!
//! ## Limitations
//! - Key values are currently [String] only.

use hdk::prelude::*;

pub mod entries;
pub mod utils;

use entries::item;
use entries::store;

use item::*;
use store::*;

entry_defs![store::Store::entry_def(), item::Item::entry_def()];

#[macro_use]
extern crate derive_new;

#[hdk_extern]
fn validate(_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

/// Creates a named key/value store.
#[hdk_extern]
pub fn create_store(store_input: StoreInput) -> ExternResult<StoreWithHash> {
    store::handlers::create_store(store_input)
}

/// Returns store information, or [error::StorageError::StoreNotFound] if no store with the given
/// name.
#[hdk_extern]
pub fn get_store(store_input: StoreInput) -> ExternResult<StoreWithHash> {
    store::handlers::get_store(store_input)
}

/// Sets the value of the pair identified by key to value, creating a new key/value pair if none
/// existed for key previously.
///
/// @todo - Accepts only strings as value, could this be changed?
#[hdk_extern]
pub fn set_item(item_input: ItemInput) -> ExternResult<ItemWithHash> {
    item::handlers::set_item(item_input)
}

/// Returns the current value associated with the given key, or
/// [error::StorageError::ItemKeyNotFound] if the given key does not exist.
#[hdk_extern]
pub fn get_item(key_input: KeyInput) -> ExternResult<ItemWithHash> {
    item::handlers::get_item(key_input)
}

/// Returns a list with all the keys in the store
#[hdk_extern]
pub fn keys(store_input: StoreInput) -> ExternResult<KeyList> {
    item::handlers::keys(store_input)
}

/// Returns the name of the nth key, or `Error` if n is greater than or equal to the number of key
/// value pairs.
#[hdk_extern]
pub fn key(index_input: IndexInput) -> ExternResult<Key> {
    item::handlers::key(index_input)
}

/// Returns the number of keys in the store
#[hdk_extern]
pub fn length(store_input: StoreInput) -> ExternResult<Length> {
    item::handlers::length(store_input)
}

/// Removes the key/value pair with the given key, if a key/value pair with the given key exists.
#[hdk_extern]
pub fn remove_item(key_input: KeyInput) -> ExternResult<()> {
    item::handlers::remove_item(key_input)
}

/// Removes all key/value pairs, if there are any.
#[hdk_extern]
pub fn clear(store_input: StoreInput) -> ExternResult<()> {
    item::handlers::clear(store_input)
}
