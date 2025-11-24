use crate::domain::mock_data::User;

pub mod b_tree;
pub mod hashmap;

pub use b_tree::*;
pub use hashmap::*;

pub fn create_mock_user() -> User {
    User {
        id: "100".to_string(),
        name: "John".to_string(),
    }
}
