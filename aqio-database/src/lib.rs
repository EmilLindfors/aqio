use anyhow::Result;
use sqlx::{Pool, Sqlite, SqlitePool};

pub mod repository;

#[derive(Clone)]
pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Database { pool })
    }

    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_connection() {
        let db = Database::new(":memory:").await.unwrap();
        assert!(!db.pool.is_closed());
    }
}
