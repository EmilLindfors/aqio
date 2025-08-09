use aqio_database::{Database, RepositoryFactory, AllRepositories};
use aqio_core::{User, Event, UserRole};
use chrono::Utc;
use uuid::Uuid;

/// Example demonstrating different ways to use the Repository Factory pattern
/// 
/// This example shows:
/// 1. Basic factory usage
/// 2. All repositories at once
/// 3. Factory sharing across services
/// 4. Error handling patterns

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database with factory
    let db = Database::new("sqlite:example.db").await?;
    
    // Method 1: Individual repository creation through factory
    println!("=== Method 1: Individual Repository Creation ===");
    basic_factory_usage(db.repositories()).await?;
    
    // Method 2: All repositories at once
    println!("\n=== Method 2: All Repositories at Once ===");
    all_repositories_usage(&db).await?;
    
    // Method 3: Factory sharing across services
    println!("\n=== Method 3: Factory Sharing Across Services ===");
    service_layer_usage(db.repositories()).await?;
    
    println!("\n✅ Repository Factory example completed successfully!");
    
    Ok(())
}

/// Basic usage: Create repositories individually as needed
async fn basic_factory_usage(factory: &RepositoryFactory) -> Result<(), Box<dyn std::error::Error>> {
    // Create only the repositories you need
    let user_repo = factory.user_repository();
    let event_repo = factory.event_repository();
    
    println!("📝 Created user and event repositories");
    println!("💾 Database pool status: connected = {}", !factory.pool().is_closed());
    
    // Each repository shares the same underlying database connection pool
    // but provides type-safe access to specific domain entities
    
    Ok(())
}

/// Efficient usage: Get all repositories at once when you need multiple
async fn all_repositories_usage(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    // Get all repositories in one call - more efficient for complex operations
    let repos = db.all_repositories();
    
    println!("📚 Created all repositories:");
    println!("  - User repository: ✓");
    println!("  - Event repository: ✓");  
    println!("  - Event Category repository: ✓");
    println!("  - Invitation repository: ✓");
    println!("  - Registration repository: ✓");
    
    // Use repositories for related operations
    // This is ideal for service layers that need to coordinate between multiple entities
    
    Ok(())
}

/// Service layer usage: How to use the factory in application services
async fn service_layer_usage(factory: &RepositoryFactory) -> Result<(), Box<dyn std::error::Error>> {
    // Simulate application services using the factory
    let user_service = UserService::new(factory);
    let event_service = EventService::new(factory);
    
    println!("🏗️  Created application services with shared factory");
    println!("🔄 Services share database connections efficiently");
    
    // Services can create repositories as needed without direct database coupling
    user_service.example_operation().await?;
    event_service.example_operation().await?;
    
    Ok(())
}

/// Example application service using the repository factory
struct UserService {
    factory: RepositoryFactory,
}

impl UserService {
    fn new(factory: &RepositoryFactory) -> Self {
        Self {
            factory: factory.clone(), // Cheap clone - shares the same pool
        }
    }
    
    async fn example_operation(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create repository when needed
        let user_repo = self.factory.user_repository();
        
        println!("👤 UserService: Using user repository");
        
        // In a real application, you'd perform user operations here
        // let users = user_repo.list_all(pagination).await?;
        
        Ok(())
    }
    
    /// Example of a service method that needs multiple repositories
    async fn complex_user_operation(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get multiple repositories for complex operations
        let user_repo = self.factory.user_repository();
        let event_repo = self.factory.event_repository();
        let registration_repo = self.factory.registration_repository();
        
        println!("🔄 UserService: Complex operation using multiple repositories");
        
        // Coordinate between multiple repositories
        // 1. Validate user exists
        // 2. Check event capacity  
        // 3. Create registration
        // All sharing the same database connection pool
        
        Ok(())
    }
}

/// Example application service for events
struct EventService {
    factory: RepositoryFactory,
}

impl EventService {
    fn new(factory: &RepositoryFactory) -> Self {
        Self {
            factory: factory.clone(),
        }
    }
    
    async fn example_operation(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create repositories as needed
        let event_repo = self.factory.event_repository();
        let category_repo = self.factory.event_category_repository();
        
        println!("📅 EventService: Using event and category repositories");
        
        // Service logic here...
        
        Ok(())
    }
}

/// Example showing how to create a mock factory for testing
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    /// Test helper to create a factory with in-memory database
    async fn create_test_factory() -> RepositoryFactory {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        RepositoryFactory::new(pool)
    }

    #[tokio::test]
    async fn test_service_with_factory() {
        let factory = create_test_factory().await;
        let user_service = UserService::new(&factory);
        
        // Test that service can use the factory
        user_service.example_operation().await.unwrap();
    }
    
    /// Example showing how the factory pattern enables easy testing
    #[tokio::test]
    async fn test_repository_factory_isolation() {
        // Each test gets its own isolated database
        let factory1 = create_test_factory().await;
        let factory2 = create_test_factory().await;
        
        let service1 = UserService::new(&factory1);
        let service2 = UserService::new(&factory2);
        
        // Services are completely isolated
        service1.example_operation().await.unwrap();
        service2.example_operation().await.unwrap();
    }
}