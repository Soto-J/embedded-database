use bincode::error::{DecodeError, EncodeError};

#[derive(Debug)]
pub enum DatabaseError {
    UserNotFound,

    // JSON
    JsonSerializationError(serde_json::Error),
    JsonDeserializationError(serde_json::Error),

    // Bincode
    BincodeEncodeError(EncodeError),
    BincodeDecodeError(DecodeError),
}
