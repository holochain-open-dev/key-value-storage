//! Utility functions

use hdk::prelude::*;

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(err(reason))
}

pub fn err(reason: &str) -> WasmError {
    WasmError::Guest(String::from(reason))
}

pub fn try_get_and_convert<T: TryFrom<SerializedBytes>>(entry_hash: EntryHash) -> ExternResult<T> {
    match get(entry_hash.clone(), GetOptions::default())? {
        Some(element) => try_from_element(element),
        None => error("Entry not found"),
    }
}

pub fn try_from_element<T: TryFrom<SerializedBytes>>(element: Element) -> ExternResult<T> {
    match element.entry() {
        element::ElementEntry::Present(entry) => try_from_entry::<T>(entry.clone()),
        _ => error("Could not convert element"),
    }
}

pub fn try_from_entry<T: TryFrom<SerializedBytes>>(entry: Entry) -> ExternResult<T> {
    match entry {
        Entry::App(content) => match T::try_from(content.into_sb()) {
            Ok(e) => Ok(e),
            Err(_) => error("Could not convert entry"),
        },
        _ => error("Could not convert entry"),
    }
}
#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
struct StringLinkTag(String);
pub fn link_tag(tag: &str) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = StringLinkTag(tag.into()).try_into()?;
    Ok(LinkTag(sb.bytes().clone()))
}
