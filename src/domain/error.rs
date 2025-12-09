use core::str::Utf8Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error(transparent)]
    Core(#[from] CoreError),

    #[cfg(feature = "std")]
    #[error(transparent)]
    Std(#[from] StdError),

    #[error("user not found")]
    UserNotFound,
}

use heapless::CapacityError;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("UUID formatting failed")]
    UuidFormattingError,
    #[error("failed to insert data")]
    InsertionFailed,
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] Utf8Error),

    // JSON
    #[error("serde_json_core serialization error: {0:?}")]
    JsonSerializationError(#[from] serde_json_core::ser::Error),
    #[error("serde_json_core deserialization error {0:?}")]
    JsonDeserializationError(#[from] serde_json_core::de::Error),

    // Bincode
    #[error("bincode error: {0:?}")]
    CapacityError(#[from] CapacityError),
    #[error("bincode error: {0:?}")]
    BinaryEncodingError(#[from] postcard::Error),
    #[error("failed to decode data: {0:?}")]
    BinaryDecodingError(postcard::Error),
}

#[cfg(feature = "std")]
use bincode::error::{self, DecodeError, EncodeError};
#[cfg(feature = "std")]
use serde_json_core::heapless::String;

#[cfg(feature = "std")]
#[derive(Debug, Error)]
pub enum StdError {
    // JSON
    #[error("failed to serialize data")]
    JsonSerializationError(serde_json::Error),
    #[error("failed to deserialize data")]
    JsonDeserializationError(serde_json::Error),

    // Bincode
    #[error("failed to encode data")]
    BincodeEncodeError(EncodeError),
    #[error("failed to decode data")]
    BincodeDecodeError(DecodeError),
}
