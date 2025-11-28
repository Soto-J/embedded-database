#[cfg(feature = "std")]
use embed_db::domain::mock_data::User;

#[cfg(feature = "std")]
pub fn create_mock_data() -> Vec<User> {
    (0..1000)
        .map(|i| User {
            id: i.to_string(),
            name: format!("User {}", i),
        })
        .collect()
}
