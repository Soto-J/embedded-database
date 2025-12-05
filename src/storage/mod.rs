use crate::domain::{DatabaseError, mock_data::User};

#[cfg(feature = "std")]
pub mod std;

pub mod core;

pub trait StorageEngine {
    fn insert(&mut self, user: User) -> Result<(), DatabaseError>;
    fn get(&self, key: &str) -> Result<Option<User>, DatabaseError>;
    fn delete(&mut self, key: &str) -> Result<Option<User>, DatabaseError>;
}
