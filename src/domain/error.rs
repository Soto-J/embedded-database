use bincode::error::{self, DecodeError, EncodeError};
use heapless::CapacityError;
use serde_json_core::heapless::String;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error(transparent)]
    Std(#[from] StdError),
    #[error(transparent)]
    Core(#[from] CoreError),

    #[error("user not found")]
    UserNotFound,
}

#[derive(Debug, Error)]
pub enum StdError {
    // JSON
    #[error("failed to serialize data")]
    JsonSerializationError(serde_json::Error),
    #[error("failed to derialize data")]
    JsonDeserializationError(serde_json::Error),

    // Bincode
    #[error("failed to encode data")]
    BincodeEncodeError(EncodeError),
    #[error("failed to decode data")]
    BincodeDecodeError(DecodeError),
}

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("UUID formatting failed")]
    UuidFormattingError,
    #[error("failed to insert data")]
    InsertionFailed,

    // JSON
    #[error("serde_json_core serialization error: {0:?}")]
    JsonSerializationError(#[from] serde_json_core::ser::Error),
    #[error("serde_json_core deserialization error {0:?}")]
    JsonDeserializationError(#[from] serde_json_core::de::Error),

    // Bincode
    #[error("bincode error: {0:?}")]
    CapacityError(#[from] CapacityError),
    #[error("bincode error: {0:?}")]
    BincodeEncodeError(#[from] postcard::Error),
    #[error("failed to decode data: {0:?}")]
    BincodeDecodeError(postcard::Error),
}
