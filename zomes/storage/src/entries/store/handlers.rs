use hdk3::prelude::*;

use hc_utils::WrappedEntryHash;

use crate::{
    error::{StorageError, StorageResult},
    store::{Store, StoreInput, StoreWithHash},
    utils,
};

pub fn create_store(store_input: StoreInput) -> StorageResult<StoreWithHash> {
    match get_store(store_input.clone()) {
        Ok(_) => return Err(StorageError::StoreAlreadyExists(store_input.store)),
        _ => (),
    };

    let store = Store {
        name: store_input.store.clone(),
    };
    create_entry(&store)?;

    let store_hash = hash_entry(&store)?;

    let agent_info = agent_info()?;
    let agent_address: AnyDhtHash = agent_info.agent_initial_pubkey.clone().into();

    create_link(
        agent_address.into(),
        store_hash.clone(),
        utils::link_tag("store")?,
    )?;

    let result = StoreWithHash {
        store_hash: WrappedEntryHash(store_hash),
        store: store,
    };

    Ok(result)
}

pub fn get_store(store_input: StoreInput) -> StorageResult<StoreWithHash> {
    let agent_info = agent_info()?;
    let agent_address: AnyDhtHash = agent_info.agent_initial_pubkey.clone().into();

    let links = get_links(
        agent_address.clone().into(),
        Some(utils::link_tag("store")?),
    )?;

    for link in links.into_inner().into_iter() {
        let store: Store = utils::try_get_and_convert(link.target.clone())?;
        if store.name == store_input.store {
            let result = StoreWithHash {
                store_hash: WrappedEntryHash(link.target),
                store,
            };
            return Ok(result);
        }
    }

    Err(StorageError::StoreNotFound(store_input.store))
}
