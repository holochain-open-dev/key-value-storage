use hdk::prelude::*;

use hc_utils::WrappedEntryHash;

use crate::{store::*, utils};

#[hdk_extern]
fn validate_create_entry_store(
    validation_data: ValidateData,
) -> ExternResult<ValidateCallbackResult> {
    let element = validation_data.element;
    let store = match element.entry().to_app_option::<Store>() {
        Ok(Some(store)) => store,
        _ => {
            return Ok(ValidateCallbackResult::Invalid(
                "Not a store object.".to_string(),
            ))
        }
    };

    // Check store name length > 0 <= 50
    let result = match store.name.len() {
        len if len == 0 => {
            ValidateCallbackResult::Invalid("Min store name length is 1.".to_string())
        }
        len if len > 50 => {
            ValidateCallbackResult::Invalid("Max store name length is 50.".to_string())
        }
        _ => ValidateCallbackResult::Valid,
    };
    Ok(result)
}

pub fn create_store(store_input: StoreInput) -> ExternResult<StoreWithHash> {
    match get_store(store_input.clone()) {
        Ok(_) => return utils::error(&format!("Store '{0}' already exists.", store_input.store)),
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

pub fn get_store(store_input: StoreInput) -> ExternResult<StoreWithHash> {
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

    utils::error(&format!("Store '{0}' not found.", store_input.store))
}
