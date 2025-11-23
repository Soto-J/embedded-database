use bincode::{
    Encode,
    error::{DecodeError, EncodeError},
};

#[derive(Debug)]
pub enum DatabaseError {
    // JSON
    JsonSerializationError(serde_json::Error),
    JsonDeserializationError(serde_json::Error),

    // Bincode
    BincodeEncodeError(EncodeError),
    BincodeDecodeError(DecodeError),
}
