use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

// For std: import std String explicitly
#[cfg(feature = "std")]
use std::string::String;

// For no_std: import heapless String
#[cfg(not(feature = "std"))]
use heapless::String;

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
    pub id: String<32>,
    pub name: String<64>,
}
