use crate::storage::json::{JsonBTree, JsonHashMap};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Database {
    pub json_map: JsonHashMap,
    pub json_btree: JsonBTree,
}

impl Database {}
