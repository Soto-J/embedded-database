use crate::domain::CoreError;

#[cfg(not(feature = "std"))]
use heapless::String; // For no_std: heapless String

use core::fmt::Write;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(not(feature = "std"))]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String<256>,
    pub name: String<64>,
}

#[cfg(not(feature = "std"))]
impl User {
    pub fn new(name: String<64>) -> Result<Self, CoreError> {
        let uuid = Uuid::new_v4();

        let mut id: String<256> = String::new();

        write!(&mut id, "{}", uuid).map_err(|e| CoreError::UuidFormattingError)?;

        Ok(Self { id, name })
    }
}

#[cfg(feature = "std")]
use bincode::{Decode, Encode};
#[cfg(feature = "std")]
use std::string::String;

#[cfg(feature = "std")]
#[derive(Debug, Default, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[cfg(feature = "std")]
pub fn create_mock_user() -> User {
    User {
        id: "100".to_string(),
        name: "John".to_string(),
    }
}
