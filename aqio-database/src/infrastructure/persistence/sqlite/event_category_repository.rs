use crate::domain::{
    errors::{InfrastructureError},
    repositories::EventCategoryRepository,
};
use crate::infrastructure::persistence::sqlite::types::{SafeRowGet, RowConversionError};
use aqio_core::{EventCategory, DomainResult};
use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use tracing::{instrument, debug};

#[derive(Clone)]
pub struct SqliteEventCategoryRepository {
    pool: Pool<Sqlite>,
}

impl SqliteEventCategoryRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    // Helper method to convert database row to EventCategory using SafeRowGet
    fn row_to_category(row: &sqlx::sqlite::SqliteRow) -> Result<EventCategory, RowConversionError> {
        Ok(EventCategory {
            id: row.get_string("id")?,
            name: row.get_string("name")?,
            description: row.get_optional_string("description")?,
            color_hex: row.get_optional_string("color_hex")?,
            icon_name: row.get_optional_string("icon_name")?,
            is_active: row.get_bool("is_active")?,
            created_at: row.get_datetime("created_at")?,
        })
    }

    // Helper method to convert RowConversionError to InfrastructureError
    fn conversion_error_to_infrastructure_error(error: RowConversionError) -> InfrastructureError {
        InfrastructureError::from(error)
    }
}

#[async_trait]
impl EventCategoryRepository for SqliteEventCategoryRepository {
    #[instrument(skip(self, category))]
    async fn create(&self, category: &EventCategory) -> DomainResult<()> {
        debug!("Creating event category with id: {}", category.id);
        
        let result = sqlx::query(
            "INSERT INTO event_categories (id, name, description, color_hex, icon_name, is_active, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&category.id)
        .bind(&category.name)
        .bind(category.description.as_deref())
        .bind(category.color_hex.as_deref())
        .bind(category.icon_name.as_deref())
        .bind(category.is_active)
        .bind(category.created_at.naive_utc())
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => {
                debug!("Successfully created event category with id: {}", category.id);
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

    #[instrument(skip(self, category))]
    async fn update(&self, category: &EventCategory) -> DomainResult<()> {
        debug!("Updating event category with id: {}", category.id);
        
        let result = sqlx::query(
            "UPDATE event_categories SET name = ?, description = ?, color_hex = ?, icon_name = ?, is_active = ? WHERE id = ?"
        )
        .bind(&category.name)
        .bind(category.description.as_deref())
        .bind(category.color_hex.as_deref())
        .bind(category.icon_name.as_deref())
        .bind(category.is_active)
        .bind(&category.id)
        .execute(&self.pool)
        .await;

        match result {
            Ok(query_result) => {
                if query_result.rows_affected() == 0 {
                    Err(aqio_core::DomainError::not_found_by_field("EventCategory", "id", &category.id))
                } else {
                    debug!("Successfully updated event category with id: {}", category.id);
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
    async fn find_by_id(&self, id: &str) -> DomainResult<Option<EventCategory>> {
        debug!("Finding event category by id: {}", id);

        let result = sqlx::query("SELECT id, name, description, color_hex, icon_name, is_active, created_at FROM event_categories WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(Some(row)) => {
                match Self::row_to_category(&row) {
                    Ok(category) => {
                        debug!("Found event category: {}", category.name);
                        Ok(Some(category))
                    }
                    Err(conv_error) => {
                        let infrastructure_error = Self::conversion_error_to_infrastructure_error(conv_error);
                        Err(infrastructure_error.into())
                    }
                }
            }
            Ok(None) => {
                debug!("Event category not found with id: {}", id);
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
    async fn list_active(&self) -> DomainResult<Vec<EventCategory>> {
        debug!("Listing active event categories");

        let result = sqlx::query("SELECT id, name, description, color_hex, icon_name, is_active, created_at FROM event_categories WHERE is_active = true ORDER BY name")
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(rows) => {
                let mut categories = Vec::new();
                for row in rows.iter() {
                    match Self::row_to_category(row) {
                        Ok(category) => categories.push(category),
                        Err(conv_error) => {
                            let infrastructure_error = Self::conversion_error_to_infrastructure_error(conv_error);
                            return Err(infrastructure_error.into());
                        }
                    }
                }
                debug!("Found {} active event categories", categories.len());
                Ok(categories)
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
    async fn list_all(&self) -> DomainResult<Vec<EventCategory>> {
        debug!("Listing all event categories");

        let result = sqlx::query("SELECT id, name, description, color_hex, icon_name, is_active, created_at FROM event_categories ORDER BY name")
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(rows) => {
                let mut categories = Vec::new();
                for row in rows.iter() {
                    match Self::row_to_category(row) {
                        Ok(category) => categories.push(category),
                        Err(conv_error) => {
                            let infrastructure_error = Self::conversion_error_to_infrastructure_error(conv_error);
                            return Err(infrastructure_error.into());
                        }
                    }
                }
                debug!("Found {} event categories", categories.len());
                Ok(categories)
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
    async fn delete(&self, id: &str) -> DomainResult<()> {
        debug!("Deleting event category with id: {}", id);
        
        let result = sqlx::query("DELETE FROM event_categories WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await;

        match result {
            Ok(query_result) => {
                if query_result.rows_affected() == 0 {
                    Err(aqio_core::DomainError::not_found_by_field("EventCategory", "id", id))
                } else {
                    debug!("Successfully deleted event category with id: {}", id);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use sqlx::{Pool, Sqlite};

    // Test helper to create an in-memory SQLite database with schema
    async fn create_test_db() -> Pool<Sqlite> {
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        
        // Create the event_categories table
        sqlx::query(r#"
            CREATE TABLE event_categories (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                color_hex TEXT,
                icon_name TEXT,
                is_active BOOLEAN NOT NULL DEFAULT TRUE,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
        "#)
        .execute(&pool)
        .await
        .unwrap();
        
        pool
    }
    
    // Helper function to create a test category
    fn create_test_category(id: &str, name: &str) -> EventCategory {
        let now = Utc::now();
        EventCategory {
            id: id.to_string(),
            name: name.to_string(),
            description: Some(format!("{} category description", name)),
            color_hex: Some("#3B82F6".to_string()),
            icon_name: Some("test-icon".to_string()),
            is_active: true,
            created_at: now,
        }
    }

    #[tokio::test]
    async fn test_create_and_find_category() {
        let pool = create_test_db().await;
        let repository = SqliteEventCategoryRepository::new(pool);
        let category = create_test_category("test", "Test Category");

        // Create the category
        let result = repository.create(&category).await;
        assert!(result.is_ok(), "Failed to create category: {:?}", result);

        // Find the category
        let found_category = repository.find_by_id("test").await.unwrap();
        assert!(found_category.is_some(), "Category not found after creation");
        
        let found_category = found_category.unwrap();
        assert_eq!(found_category.id, "test");
        assert_eq!(found_category.name, "Test Category");
        assert_eq!(found_category.description, Some("Test Category category description".to_string()));
        assert!(found_category.is_active);
    }

    #[tokio::test]
    async fn test_update_category() {
        let pool = create_test_db().await;
        let repository = SqliteEventCategoryRepository::new(pool);
        let mut category = create_test_category("update_test", "Original Name");

        // Create the category
        repository.create(&category).await.unwrap();

        // Update the category
        category.name = "Updated Name".to_string();
        category.description = Some("Updated description".to_string());
        category.color_hex = Some("#FF0000".to_string());
        category.is_active = false;

        let result = repository.update(&category).await;
        assert!(result.is_ok(), "Failed to update category: {:?}", result);

        // Verify the update
        let found_category = repository.find_by_id("update_test").await.unwrap().unwrap();
        assert_eq!(found_category.name, "Updated Name");
        assert_eq!(found_category.description, Some("Updated description".to_string()));
        assert_eq!(found_category.color_hex, Some("#FF0000".to_string()));
        assert!(!found_category.is_active);
    }

    #[tokio::test]
    async fn test_list_active_categories() {
        let pool = create_test_db().await;
        let repository = SqliteEventCategoryRepository::new(pool);

        // Create multiple categories, some active, some inactive
        let mut active_category = create_test_category("active", "Active Category");
        active_category.is_active = true;
        repository.create(&active_category).await.unwrap();

        let mut inactive_category = create_test_category("inactive", "Inactive Category");
        inactive_category.is_active = false;
        repository.create(&inactive_category).await.unwrap();

        let mut another_active = create_test_category("active2", "Another Active");
        another_active.is_active = true;
        repository.create(&another_active).await.unwrap();

        // List active categories
        let active_categories = repository.list_active().await.unwrap();
        assert_eq!(active_categories.len(), 2);
        
        let active_ids: Vec<&str> = active_categories.iter().map(|c| c.id.as_str()).collect();
        assert!(active_ids.contains(&"active"));
        assert!(active_ids.contains(&"active2"));
        assert!(!active_ids.contains(&"inactive"));
    }

    #[tokio::test]
    async fn test_list_all_categories() {
        let pool = create_test_db().await;
        let repository = SqliteEventCategoryRepository::new(pool);

        // Create multiple categories
        let category1 = create_test_category("cat1", "Category 1");
        let category2 = create_test_category("cat2", "Category 2");
        let mut inactive_category = create_test_category("cat3", "Category 3");
        inactive_category.is_active = false;

        repository.create(&category1).await.unwrap();
        repository.create(&category2).await.unwrap();
        repository.create(&inactive_category).await.unwrap();

        // List all categories
        let all_categories = repository.list_all().await.unwrap();
        assert_eq!(all_categories.len(), 3);
        
        let all_ids: Vec<&str> = all_categories.iter().map(|c| c.id.as_str()).collect();
        assert!(all_ids.contains(&"cat1"));
        assert!(all_ids.contains(&"cat2"));
        assert!(all_ids.contains(&"cat3"));
        
        // Verify the inactive one is included
        let inactive_found = all_categories.iter().find(|c| c.id == "cat3").unwrap();
        assert!(!inactive_found.is_active);
    }

    #[tokio::test]
    async fn test_find_nonexistent_category() {
        let pool = create_test_db().await;
        let repository = SqliteEventCategoryRepository::new(pool);

        // Try to find a category that doesn't exist
        let result = repository.find_by_id("nonexistent").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_update_nonexistent_category() {
        let pool = create_test_db().await;
        let repository = SqliteEventCategoryRepository::new(pool);
        let category = create_test_category("nonexistent", "Non-existent Category");

        // Try to update a category that doesn't exist
        let result = repository.update(&category).await;
        assert!(result.is_err(), "Should fail when updating non-existent category");
    }

    #[tokio::test]
    async fn test_empty_lists() {
        let pool = create_test_db().await;
        let repository = SqliteEventCategoryRepository::new(pool);

        // Test empty list_active
        let active_categories = repository.list_active().await.unwrap();
        assert_eq!(active_categories.len(), 0);

        // Test empty list_all
        let all_categories = repository.list_all().await.unwrap();
        assert_eq!(all_categories.len(), 0);
    }

    #[tokio::test]
    async fn test_category_ordering() {
        let pool = create_test_db().await;
        let repository = SqliteEventCategoryRepository::new(pool);

        // Create categories in non-alphabetical order
        let category_z = create_test_category("z", "Z Category");
        let category_a = create_test_category("a", "A Category");
        let category_m = create_test_category("m", "M Category");

        repository.create(&category_z).await.unwrap();
        repository.create(&category_a).await.unwrap();
        repository.create(&category_m).await.unwrap();

        // List all categories (should be ordered by name)
        let all_categories = repository.list_all().await.unwrap();
        assert_eq!(all_categories.len(), 3);
        
        // Check alphabetical ordering by name
        assert_eq!(all_categories[0].name, "A Category");
        assert_eq!(all_categories[1].name, "M Category");
        assert_eq!(all_categories[2].name, "Z Category");
    }
}