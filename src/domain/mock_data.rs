use crate::domain::CoreError;

#[cfg(feature = "std")]
use bincode::{Decode, Encode};

use core::fmt::Write;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// For std: import std String explicitly
#[cfg(feature = "std")]
use std::string::String;

// For no_std: import heapless String
#[cfg(not(feature = "std"))]
use heapless::String;
// use serde_json_core::heapless::String;


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

#[cfg(not(feature = "std"))]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String<256>,
    pub name: String<64>,
}

impl User {
    pub fn new(name: String<64>) -> Result<Self, CoreError> {
        let uuid = Uuid::new_v4();

        let mut id: String<256> = String::new();

        write!(&mut id, "{}", uuid).map_err(|e| CoreError::UuidFormattingError)?;

        Ok(Self { id, name })
    }
}
