use std::sync::Arc;

use sqlx::{pool::PoolOptions, Pool, Postgres};

pub trait Db {
  async fn new(conn: &str) -> Self;
}

#[derive(Clone)]
pub struct Database<T> {
  pub pool: Arc<T>
}

impl Db for Database<Pool<Postgres>> {
    async fn new(conn: &str) -> Self {
      return Self {
        pool: Arc::new(PoolOptions::new()
          .max_connections(5)
          .connect(conn).await.expect("Coudn't connect do the db"))
      }
    }
}
