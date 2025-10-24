use std::error::Error;

use crate::modules::public::user::domain::entity::user::User;

pub trait UserRepository<T> {
  async fn create(&self, user: User) -> Result<T, Box<dyn Error>>;
  async fn find_all(&self) -> Result<Vec<User>, Box<dyn Error>>;
}