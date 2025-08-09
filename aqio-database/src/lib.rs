use sqlx::{Pool, Sqlite, SqlitePool};

pub mod domain;
pub mod infrastructure;

// Re-export commonly used types for convenience
pub use aqio_core::*;
pub use domain::{
    errors::{InfrastructureError, InfrastructureResult},
};
pub use infrastructure::persistence::sqlite::{RepositoryFactory, AllRepositories};

#[derive(Clone)]
pub struct Database {
    pool: Pool<Sqlite>,
    factory: RepositoryFactory,
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

        let factory = RepositoryFactory::new(pool.clone());

        Ok(Database { pool, factory })
    }

    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }

    /// Get the repository factory for creating repository instances
    pub fn repositories(&self) -> &RepositoryFactory {
        &self.factory
    }

    /// Create all repositories at once
    pub fn all_repositories(&self) -> AllRepositories {
        self.factory.all_repositories()
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

    #[tokio::test]
    async fn test_database_repository_factory() {
        let db = Database::new(":memory:").await.unwrap();
        
        // Test individual repository creation through factory
        let factory = db.repositories();
        let _user_repo = factory.user_repository();
        let _event_repo = factory.event_repository();
        let _category_repo = factory.event_category_repository();
        let _invitation_repo = factory.invitation_repository();
        let _registration_repo = factory.registration_repository();
        
        // Test all repositories creation
        let all_repos = db.all_repositories();
        let _user = &all_repos.user;
        let _event = &all_repos.event;
        let _category = &all_repos.event_category;
        let _invitation = &all_repos.invitation;
        let _registration = &all_repos.registration;
    }

    #[tokio::test]
    async fn test_database_factory_consistency() {
        let db = Database::new(":memory:").await.unwrap();
        
        // Multiple calls to repositories() should return the same factory
        let factory1 = db.repositories();
        let factory2 = db.repositories();
        
        // Both should create functional repositories
        let _user_repo1 = factory1.user_repository();
        let _user_repo2 = factory2.user_repository();
        
        // Verify they're both functional (they share the same underlying pool)
        assert!(!factory1.pool().is_closed());
        assert!(!factory2.pool().is_closed());
    }
}
