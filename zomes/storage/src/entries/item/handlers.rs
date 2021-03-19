use hdk::prelude::*;

use hc_utils::WrappedHeaderHash;

use crate::{item::*, store::handlers::*, store::*, utils};

#[hdk_extern]
fn validate_create_entry_item(
    validation_data: ValidateData,
) -> ExternResult<ValidateCallbackResult> {
    let element = validation_data.element;
    let item = match element.entry().to_app_option::<Item>() {
        Ok(Some(item)) => item,
        _ => {
            return Ok(ValidateCallbackResult::Invalid(
                "Not an item object.".to_string(),
            ))
        }
    };

    // Check item name length > 0 <= 50
    match item.key.len() {
        len if len == 0 => {
            return Ok(ValidateCallbackResult::Invalid("Min key length is 1.".to_string()))
        }
        len if len > 50 => {
            return Ok(ValidateCallbackResult::Invalid("Max key length is 50.".to_string()))
        }
        _ => ()
    };

    // Check item name length > 0 <= 255
    match item.value.len() {
        len if len == 0 => {
            return Ok(ValidateCallbackResult::Invalid("Min value length is 1.".to_string()))
        }
        len if len > 256 => {
            return Ok(ValidateCallbackResult::Invalid("Max value length is 255.".to_string()))
        }
        _ => return Ok(ValidateCallbackResult::Valid),
    };
}

pub fn set_item(item_input: ItemInput) -> ExternResult<ItemWithHash> {
    // Get store or fail if it doesn't exist
    let store_with_hash: StoreWithHash = get_store(StoreInput::new(item_input.store.clone()))?;

    // Remove old item if exists
    remove_item(KeyInput::new(
        item_input.store.clone(),
        item_input.key.clone(),
    ))?;

    let item = Item {
        key: item_input.key,
        value: item_input.value,
    };

    let entry_hash = hash_entry(&item)?;
    let header_hash = create_entry(&item)?;
    create_link(
        store_with_hash.store_hash.0,
        entry_hash,
        utils::link_tag(&item.key)?,
    )?;

    let result = ItemWithHash {
        item_hash: WrappedHeaderHash(header_hash),
        item,
    };

    Ok(result)
}

pub fn get_item(key_input: KeyInput) -> ExternResult<ItemWithHash> {
    // Get store or fail if it doesn't exist
    let store_with_hash: StoreWithHash = get_store(StoreInput::new(key_input.store))?;

    let links = get_links(
        store_with_hash.store_hash.0,
        Some(utils::link_tag(&key_input.key)?),
    )?;

    match links.into_inner().first() {
        Some(link) => {
            let item: Item = utils::try_get_and_convert(link.target.clone())?;
            let element = get(link.target.clone(), GetOptions::default())?.expect("Unknown error.");
            let item_hash = element.signed_header().as_hash().clone();
            let result = ItemWithHash {
                item_hash: WrappedHeaderHash(item_hash),
                item,
            };
            Ok(result)
        }
        None => utils::error(&format!("Key '{0}' not found.", key_input.key)),
    }
}

pub fn length(store_input: StoreInput) -> ExternResult<Length> {
    // Get store or fail if it doesn't exist
    let store_with_hash: StoreWithHash = get_store(store_input)?;

    Ok(Length::new(
        get_links(store_with_hash.store_hash.0, None)?
            .into_inner()
            .len(),
    ))
}

pub fn keys(store_input: StoreInput) -> ExternResult<KeyList> {
    // Get store or fail if it doesn't exist
    let store_with_hash: StoreWithHash = get_store(store_input)?;

    let links_iter = get_links(store_with_hash.store_hash.0, None)?
        .into_inner()
        .into_iter();

    let mut keys = Vec::with_capacity(links_iter.len());

    for link in links_iter {
        let item: Item = utils::try_get_and_convert(link.target.clone())?;
        let key = Key { key: item.key };
        keys.push(key);
    }
    Ok(keys.into())
}

pub fn key(index_input: IndexInput) -> ExternResult<Key> {
    // Get store or fail if it doesn't exist
    let store_with_hash: StoreWithHash = get_store(StoreInput::new(index_input.store))?;

    let link = &get_links(store_with_hash.store_hash.0, None)?.into_inner()[index_input.index];
    let item: Item = utils::try_get_and_convert(link.target.clone())?;
    Ok(Key::new(item.key))
}

pub fn remove_item(key_input: KeyInput) -> ExternResult<()> {
    // Get store or fail if it doesn't exist
    let store_with_hash: StoreWithHash = get_store(StoreInput::new(key_input.store.clone()))?;

    if let Ok(old_item) = get_item(key_input.clone()) {
        let links = get_links(
            store_with_hash.store_hash.0,
            Some(utils::link_tag(&key_input.key)?),
        )?;

        if let Some(link) = links.into_inner().first() {
            delete_link(link.create_link_hash.clone())?;
        };

        delete_entry(old_item.item_hash.0.clone())?;
    };

    Ok(())
}

pub fn clear(store_input: StoreInput) -> ExternResult<()> {
    // Get store or fail if it doesn't exist
    let store_with_hash: StoreWithHash = get_store(StoreInput::new(store_input.store.clone()))?;

    let links_iter = get_links(store_with_hash.store_hash.0, None)?
        .into_inner()
        .into_iter();

    for link in links_iter {
        let item: Item = utils::try_get_and_convert(link.target.clone())?;
        remove_item(KeyInput::new(store_input.store.clone(), item.key))?;
    }
    Ok(())
}