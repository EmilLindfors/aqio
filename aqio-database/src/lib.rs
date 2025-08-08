use sqlx::{Pool, Sqlite, SqlitePool};

pub mod domain;
pub mod infrastructure;

// Re-export commonly used types for convenience
pub use aqio_core::*;
pub use domain::{
    errors::{InfrastructureError, InfrastructureResult},
};

#[derive(Clone)]
pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(database_url: &str) -> InfrastructureResult<Self> {
        let pool = SqlitePool::connect(database_url)
            .await
            .map_err(|e| InfrastructureError::ConnectionFailed {
                message: e.to_string(),
            })?;

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| InfrastructureError::MigrationFailed {
                message: e.to_string(),
            })?;

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
