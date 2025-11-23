use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct User {
    pub id: String,
    pub name: String,
}
