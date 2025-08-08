use crate::domain::{errors::InfrastructureError, repositories::UserRepository};
use crate::infrastructure::persistence::mapping::{map_user_role, user_role_to_string};
use aqio_core::{DomainResult, PaginatedResult, PaginationParams, User};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Row, Sqlite};
use tracing::{debug, instrument};
use uuid::Uuid;

#[derive(Clone)]
pub struct SqliteUserRepository {
    pool: Pool<Sqlite>,
}

impl SqliteUserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    // Helper method to convert database row to User
    fn row_to_user(row: &sqlx::sqlite::SqliteRow) -> User {
        User {
            id: Uuid::parse_str(&row.try_get::<String, _>("id").unwrap_or_default())
                .unwrap_or_default(),
            keycloak_id: row.try_get("keycloak_id").unwrap_or_default(),
            email: row.try_get("email").unwrap_or_default(),
            name: row.try_get("name").unwrap_or_default(),
            company_id: row
                .try_get::<Option<String>, _>("company_id")
                .ok()
                .flatten()
                .and_then(|s| Uuid::parse_str(&s).ok()),
            role: map_user_role(&row.try_get::<String, _>("role").unwrap_or_default()),
            is_active: row.try_get("is_active").unwrap_or(true),
            created_at: DateTime::from_naive_utc_and_offset(
                row.try_get("created_at").unwrap_or_else(|_| {
                    chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc()
                }),
                Utc,
            ),
            updated_at: DateTime::from_naive_utc_and_offset(
                row.try_get("updated_at").unwrap_or_else(|_| {
                    chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc()
                }),
                Utc,
            ),
        }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    #[instrument(skip(self, user))]
    async fn create(&self, user: &User) -> DomainResult<()> {
        debug!("Creating user with id: {}", user.id);

        let result = sqlx::query(
            "INSERT INTO users (id, keycloak_id, email, name, company_id, role, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(user.id.to_string())
        .bind(&user.keycloak_id)
        .bind(&user.email)
        .bind(&user.name)
        .bind(user.company_id.map(|id| id.to_string()))
        .bind(user_role_to_string(&user.role))
        .bind(user.is_active)
        .bind(user.created_at.naive_utc())
        .bind(user.updated_at.naive_utc())
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => {
                debug!("Successfully created user with id: {}", user.id);
                Ok(())
            }
            Err(e) => {
                let infrastructure_error = InfrastructureError::from(e);
                match infrastructure_error {
                    InfrastructureError::DomainError { source } => Err(source),
                    other => Err(other.into()),
                }
            }
        }
    }

    #[instrument(skip(self, user))]
    async fn update(&self, user: &User) -> DomainResult<()> {
        debug!("Updating user with id: {}", user.id);

        let result = sqlx::query(
            "UPDATE users SET keycloak_id = ?, email = ?, name = ?, company_id = ?, role = ?, is_active = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&user.keycloak_id)
        .bind(&user.email)
        .bind(&user.name)
        .bind(user.company_id.map(|id| id.to_string()))
        .bind(user_role_to_string(&user.role))
        .bind(user.is_active)
        .bind(user.updated_at.naive_utc())
        .bind(user.id.to_string())
        .execute(&self.pool)
        .await;

        match result {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    debug!("User not found for update: {}", user.id);
                    Err(aqio_core::DomainError::not_found("User", user.id))
                } else {
                    debug!("Successfully updated user with id: {}", user.id);
                    Ok(())
                }
            }
            Err(e) => {
                let infrastructure_error = InfrastructureError::from(e);
                match infrastructure_error {
                    InfrastructureError::DomainError { source } => Err(source),
                    other => Err(other.into()),
                }
            }
        }
    }

    #[instrument(skip(self))]
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<User>> {
        debug!("Finding user by id: {}", id);

        let id_string = id.to_string();
        let result = sqlx::query("SELECT id, keycloak_id, email, name, company_id, role, is_active, created_at, updated_at FROM users WHERE id = ?")
            .bind(id_string)
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(Some(row)) => {
                let user = Self::row_to_user(&row);
                debug!("Found user: {}", user.email);
                Ok(Some(user))
            }
            Ok(None) => {
                debug!("User not found with id: {}", id);
                Ok(None)
            }
            Err(e) => {
                let infrastructure_error = InfrastructureError::from(e);
                match infrastructure_error {
                    InfrastructureError::DomainError { source } => Err(source),
                    other => Err(other.into()),
                }
            }
        }
    }

    #[instrument(skip(self))]
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> DomainResult<Option<User>> {
        debug!("Finding user by keycloak_id: {}", keycloak_id);

        let result = sqlx::query("SELECT id, keycloak_id, email, name, company_id, role, is_active, created_at, updated_at FROM users WHERE keycloak_id = ?")
            .bind(keycloak_id)
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(Some(row)) => {
                let user = Self::row_to_user(&row);
                debug!("Found user: {}", user.email);
                Ok(Some(user))
            }
            Ok(None) => {
                debug!("User not found with keycloak_id: {}", keycloak_id);
                Ok(None)
            }
            Err(e) => {
                let infrastructure_error = InfrastructureError::from(e);
                match infrastructure_error {
                    InfrastructureError::DomainError { source } => Err(source),
                    other => Err(other.into()),
                }
            }
        }
    }

    #[instrument(skip(self))]
    async fn find_by_email(&self, email: &str) -> DomainResult<Option<User>> {
        debug!("Finding user by email: {}", email);

        let result = sqlx::query("SELECT id, keycloak_id, email, name, company_id, role, is_active, created_at, updated_at FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(Some(row)) => {
                let user = Self::row_to_user(&row);
                debug!("Found user: {}", user.email);
                Ok(Some(user))
            }
            Ok(None) => {
                debug!("User not found with email: {}", email);
                Ok(None)
            }
            Err(e) => {
                let infrastructure_error = InfrastructureError::from(e);
                match infrastructure_error {
                    InfrastructureError::DomainError { source } => Err(source),
                    other => Err(other.into()),
                }
            }
        }
    }

    #[instrument(skip(self))]
    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        debug!("Deleting user with id: {}", id);

        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await;

        match result {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    debug!("User not found for deletion: {}", id);
                    Err(aqio_core::DomainError::not_found("User", id))
                } else {
                    debug!("Successfully deleted user with id: {}", id);
                    Ok(())
                }
            }
            Err(e) => {
                let infrastructure_error = InfrastructureError::from(e);
                match infrastructure_error {
                    InfrastructureError::DomainError { source } => Err(source),
                    other => Err(other.into()),
                }
            }
        }
    }

    #[instrument(skip(self))]
    async fn list_all(&self, pagination: PaginationParams) -> DomainResult<PaginatedResult<User>> {
        debug!("Listing all users with pagination");

        // Count total users
        let count_result = sqlx::query("SELECT COUNT(*) as count FROM users")
            .fetch_one(&self.pool)
            .await;

        let total_count = match count_result {
            Ok(row) => row.try_get::<i64, _>("count").unwrap_or(0),
            Err(e) => return Err(InfrastructureError::from(e).into()),
        };

        // Fetch the users with pagination
        let result = sqlx::query("SELECT id, keycloak_id, email, name, company_id, role, is_active, created_at, updated_at FROM users ORDER BY created_at DESC LIMIT ? OFFSET ?")
            .bind(pagination.limit)
            .bind(pagination.offset)
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(rows) => {
                let users: Vec<User> = rows.iter().map(|row| Self::row_to_user(row)).collect();
                debug!("Listed {} users out of {} total", users.len(), total_count);
                Ok(PaginatedResult::new(users, total_count, pagination))
            }
            Err(e) => {
                let infrastructure_error = InfrastructureError::from(e);
                match infrastructure_error {
                    InfrastructureError::DomainError { source } => Err(source),
                    other => Err(other.into()),
                }
            }
        }
    }

    #[instrument(skip(self))]
    async fn exists(&self, id: Uuid) -> DomainResult<bool> {
        debug!("Checking if user exists with id: {}", id);

        let result = sqlx::query("SELECT 1 FROM users WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(Some(_)) => {
                debug!("User exists with id: {}", id);
                Ok(true)
            }
            Ok(None) => {
                debug!("User does not exist with id: {}", id);
                Ok(false)
            }
            Err(e) => {
                let infrastructure_error = InfrastructureError::from(e);
                match infrastructure_error {
                    InfrastructureError::DomainError { source } => Err(source),
                    other => Err(other.into()),
                }
            }
        }
    }

    #[instrument(skip(self))]
    async fn email_exists(&self, email: &str) -> DomainResult<bool> {
        debug!("Checking if email exists: {}", email);

        let result = sqlx::query("SELECT 1 FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(Some(_)) => {
                debug!("Email exists: {}", email);
                Ok(true)
            }
            Ok(None) => {
                debug!("Email does not exist: {}", email);
                Ok(false)
            }
            Err(e) => {
                let infrastructure_error = InfrastructureError::from(e);
                match infrastructure_error {
                    InfrastructureError::DomainError { source } => Err(source),
                    other => Err(other.into()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aqio_core::UserRole;
    use chrono::Utc;
    use sqlx::{Pool, Sqlite};
    use uuid::Uuid;

    // Test helper to create an in-memory SQLite database with schema
    async fn create_test_db() -> Pool<Sqlite> {
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();

        // Create companies table first (users reference it)
        sqlx::query(
            r#"
            CREATE TABLE companies (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                org_number TEXT UNIQUE,
                location TEXT,
                industry_type TEXT NOT NULL CHECK(industry_type IN ('Salmon', 'Trout', 'Other')),
                industry_type_other TEXT,
                website TEXT,
                phone TEXT,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
        "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        // Create users table with the full schema
        sqlx::query(r#"
            CREATE TABLE users (
                id TEXT PRIMARY KEY,
                keycloak_id TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                name TEXT NOT NULL,
                company_id TEXT REFERENCES companies(id),
                role TEXT NOT NULL CHECK(role IN ('admin', 'organizer', 'participant')) DEFAULT 'participant',
                is_active BOOLEAN NOT NULL DEFAULT TRUE,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
        "#)
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    // Helper function to create a test user
    fn create_test_user(name: &str, email: &str) -> User {
        let now = Utc::now();
        User {
            id: Uuid::new_v4(),
            keycloak_id: format!("keycloak_{}", name.to_lowercase()),
            email: email.to_string(),
            name: name.to_string(),
            company_id: None,
            role: UserRole::Participant,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    #[tokio::test]
    async fn test_create_and_find_user() {
        let pool = create_test_db().await;
        let repository = SqliteUserRepository::new(pool);
        let user = create_test_user("John Doe", "john@example.com");
        let user_id = user.id;

        // Create the user
        let result = repository.create(&user).await;
        assert!(result.is_ok(), "Failed to create user: {:?}", result);

        // Find the user
        let found_user = repository.find_by_id(user_id).await.unwrap();
        assert!(found_user.is_some(), "User not found after creation");

        let found_user = found_user.unwrap();
        assert_eq!(found_user.name, "John Doe");
        assert_eq!(found_user.email, "john@example.com");
        assert_eq!(found_user.keycloak_id, "keycloak_john doe");
        assert!(matches!(found_user.role, UserRole::Participant));
        assert!(found_user.is_active);
    }

    #[tokio::test]
    async fn test_find_by_keycloak_id() {
        let pool = create_test_db().await;
        let repository = SqliteUserRepository::new(pool);
        let user = create_test_user("Jane Doe", "jane@example.com");

        repository.create(&user).await.unwrap();

        let found_user = repository
            .find_by_keycloak_id(&user.keycloak_id)
            .await
            .unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().email, "jane@example.com");
    }

    #[tokio::test]
    async fn test_find_by_email() {
        let pool = create_test_db().await;
        let repository = SqliteUserRepository::new(pool);
        let user = create_test_user("Bob Smith", "bob@example.com");

        repository.create(&user).await.unwrap();

        let found_user = repository.find_by_email("bob@example.com").await.unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().name, "Bob Smith");
    }

    #[tokio::test]
    async fn test_update_user() {
        let pool = create_test_db().await;
        let repository = SqliteUserRepository::new(pool);
        let mut user = create_test_user("Alice Johnson", "alice@example.com");
        let user_id = user.id;

        // Create user
        repository.create(&user).await.unwrap();

        // Update user
        user.name = "Alice Smith".to_string();
        user.role = UserRole::Organizer;
        user.is_active = false;
        user.updated_at = Utc::now();

        let result = repository.update(&user).await;
        assert!(result.is_ok(), "Failed to update user: {:?}", result);

        // Verify the update
        let found_user = repository.find_by_id(user_id).await.unwrap().unwrap();
        assert_eq!(found_user.name, "Alice Smith");
        assert!(matches!(found_user.role, UserRole::Organizer));
        assert!(!found_user.is_active);
    }

    #[tokio::test]
    async fn test_delete_user() {
        let pool = create_test_db().await;
        let repository = SqliteUserRepository::new(pool);
        let user = create_test_user("Delete Me", "delete@example.com");
        let user_id = user.id;

        // Create user
        repository.create(&user).await.unwrap();

        // Verify it exists
        assert!(repository.exists(user_id).await.unwrap());

        // Delete the user
        let result = repository.delete(user_id).await;
        assert!(result.is_ok(), "Failed to delete user: {:?}", result);

        // Verify it no longer exists
        assert!(!repository.exists(user_id).await.unwrap());
        assert!(repository.find_by_id(user_id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_email_exists() {
        let pool = create_test_db().await;
        let repository = SqliteUserRepository::new(pool);
        let user = create_test_user("Email Test", "email-test@example.com");

        // Check email doesn't exist initially
        assert!(
            !repository
                .email_exists("email-test@example.com")
                .await
                .unwrap()
        );

        // Create user
        repository.create(&user).await.unwrap();

        // Check email now exists
        assert!(
            repository
                .email_exists("email-test@example.com")
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn test_user_roles() {
        let pool = create_test_db().await;
        let repository = SqliteUserRepository::new(pool);

        // Test admin role
        let mut admin_user = create_test_user("Admin User", "admin@example.com");
        admin_user.role = UserRole::Admin;
        repository.create(&admin_user).await.unwrap();

        let found_admin = repository
            .find_by_email("admin@example.com")
            .await
            .unwrap()
            .unwrap();
        assert!(matches!(found_admin.role, UserRole::Admin));

        // Test organizer role
        let mut organizer_user = create_test_user("Organizer User", "organizer@example.com");
        organizer_user.role = UserRole::Organizer;
        repository.create(&organizer_user).await.unwrap();

        let found_organizer = repository
            .find_by_email("organizer@example.com")
            .await
            .unwrap()
            .unwrap();
        assert!(matches!(found_organizer.role, UserRole::Organizer));
    }
}
