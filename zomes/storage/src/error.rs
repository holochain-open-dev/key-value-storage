//! General and more zome specific error types.

use hdk3::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error(transparent)]
    Serialization(#[from] SerializedBytesError),
    #[error(transparent)]
    EntryError(#[from] EntryError),
    #[error(transparent)]
    Wasm(#[from] WasmError),
    #[error(transparent)]
    HdkError(#[from] HdkError),
    #[error("Store '{0}' not found.")]
    StoreNotFound(String),
    #[error("Store '{0}' already exists.")]
    StoreAlreadyExists(String),
    #[error("Key '{0}' not found.")]
    ItemKeyNotFound(String),
    #[error("Key not provided.")]
    KeyNotProvided(),
    #[error("Key index not provided.")]
    KeyIndexNotProvided(),
    #[error("Software failure. Guru meditation.")]
    UnknownError,
}

pub type StorageResult<T> = Result<T, StorageError>;
