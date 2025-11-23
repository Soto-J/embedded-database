use crate::storage::json::{MyBTree, hashmap::MyMap};

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Database {
    pub map: MyMap,
    pub b_tree: MyBTree,
}

impl Database {}
