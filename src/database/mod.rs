#[cfg(feature = "std")]
use crate::storage::std::{JsonBTree, JsonHashMap};
use serde::{Deserialize, Serialize};

pub mod index;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Database {
    #[cfg(feature = "std")]
    pub json_map: JsonHashMap,
    #[cfg(feature = "std")]
    pub json_btree: JsonBTree,
}

#[cfg(feature = "std")]
impl Database {}
