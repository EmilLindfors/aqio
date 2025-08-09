use sqlx::{Pool, Sqlite};

use super::{
    SqliteEventRepository,
    SqliteUserRepository, 
    SqliteInvitationRepository,
    SqliteEventCategoryRepository,
    SqliteEventRegistrationRepository,
};

/// Central factory for creating repository instances
/// 
/// This factory pattern provides several benefits:
/// - Centralized repository creation and configuration
/// - Consistent pool management across all repositories
/// - Easier testing with mock factories
/// - Single point of configuration for database connections
/// - Support for future enhancements like connection pooling strategies
#[derive(Clone, Debug)]
pub struct RepositoryFactory {
    pool: Pool<Sqlite>,
}

impl RepositoryFactory {
    /// Create a new repository factory with the given database pool
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// Create a user repository instance
    pub fn user_repository(&self) -> SqliteUserRepository {
        SqliteUserRepository::new(self.pool.clone())
    }

    /// Create an event repository instance
    pub fn event_repository(&self) -> SqliteEventRepository {
        SqliteEventRepository::new(self.pool.clone())
    }

    /// Create an event category repository instance
    pub fn event_category_repository(&self) -> SqliteEventCategoryRepository {
        SqliteEventCategoryRepository::new(self.pool.clone())
    }

    /// Create an invitation repository instance
    pub fn invitation_repository(&self) -> SqliteInvitationRepository {
        SqliteInvitationRepository::new(self.pool.clone())
    }

    /// Create an event registration repository instance
    pub fn registration_repository(&self) -> SqliteEventRegistrationRepository {
        SqliteEventRegistrationRepository::new(self.pool.clone())
    }

    /// Get access to the underlying database pool
    /// 
    /// This is useful for custom queries or transactions that span multiple repositories
    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }

    /// Create all repositories at once
    /// 
    /// This is useful when you need multiple repositories and want to create them efficiently
    pub fn all_repositories(&self) -> AllRepositories {
        AllRepositories {
            user: self.user_repository(),
            event: self.event_repository(),
            event_category: self.event_category_repository(),
            invitation: self.invitation_repository(),
            registration: self.registration_repository(),
        }
    }
}

/// Struct containing all repository instances
/// 
/// This provides convenient access to all repositories when needed
pub struct AllRepositories {
    pub user: SqliteUserRepository,
    pub event: SqliteEventRepository,
    pub event_category: SqliteEventCategoryRepository,
    pub invitation: SqliteInvitationRepository,
    pub registration: SqliteEventRegistrationRepository,
}

impl AllRepositories {
    /// Create all repositories from a database pool
    pub fn from_pool(pool: Pool<Sqlite>) -> Self {
        let factory = RepositoryFactory::new(pool);
        factory.all_repositories()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn create_test_pool() -> Pool<Sqlite> {
        SqlitePool::connect(":memory:").await.unwrap()
    }

    #[tokio::test]
    async fn test_repository_factory_creation() {
        let pool = create_test_pool().await;
        let factory = RepositoryFactory::new(pool);

        // Test that all repositories can be created
        let _user_repo = factory.user_repository();
        let _event_repo = factory.event_repository();
        let _category_repo = factory.event_category_repository();
        let _invitation_repo = factory.invitation_repository();
        let _registration_repo = factory.registration_repository();
    }

    #[tokio::test]
    async fn test_factory_clone() {
        let pool = create_test_pool().await;
        let factory1 = RepositoryFactory::new(pool);
        let factory2 = factory1.clone();

        // Both factories should work independently
        let _user_repo1 = factory1.user_repository();
        let _user_repo2 = factory2.user_repository();
    }

    #[tokio::test]
    async fn test_all_repositories() {
        let pool = create_test_pool().await;
        let factory = RepositoryFactory::new(pool.clone());
        
        let all_repos = factory.all_repositories();
        
        // Verify all repositories are created
        // We can't directly test functionality without schema, but we can verify creation
        let _user = &all_repos.user;
        let _event = &all_repos.event;
        let _category = &all_repos.event_category;
        let _invitation = &all_repos.invitation;
        let _registration = &all_repos.registration;
    }

    #[tokio::test]
    async fn test_all_repositories_from_pool() {
        let pool = create_test_pool().await;
        let all_repos = AllRepositories::from_pool(pool);
        
        // Verify all repositories are accessible
        let _user = &all_repos.user;
        let _event = &all_repos.event;
        let _category = &all_repos.event_category;
        let _invitation = &all_repos.invitation;
        let _registration = &all_repos.registration;
    }

    #[tokio::test]
    async fn test_pool_access() {
        let pool = create_test_pool().await;
        let factory = RepositoryFactory::new(pool.clone());
        
        // Should be able to access the underlying pool
        let pool_ref = factory.pool();
        
        // Pool should be accessible and not closed
        assert!(!pool_ref.is_closed());
        assert!(!pool.is_closed());
    }
}