use crate::domain::{
    errors::{InfrastructureError},
    repositories::EventInvitationRepository,
};
use aqio_core::{EventInvitation, InvitationStatus, DomainValidation, DomainResult};
use crate::infrastructure::persistence::mapping::{
    map_invitation_status, invitation_status_to_string,
    map_invitation_method, invitation_method_to_string,
    datetime_from_naive, optional_datetime_from_naive,
};
use async_trait::async_trait;
use sqlx::{Pool, Sqlite, Row};
use tracing::{instrument, debug};
use uuid::Uuid;

#[derive(Clone)]
pub struct SqliteInvitationRepository {
    pool: Pool<Sqlite>,
}

impl SqliteInvitationRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    // Helper method to convert database row to EventInvitation
    fn row_to_invitation(row: &sqlx::sqlite::SqliteRow) -> DomainResult<EventInvitation> {
        // Use the simpler approach like other repositories in this codebase
        Ok(EventInvitation {
            id: Uuid::parse_str(&row.try_get::<String, _>("id").unwrap_or_default()).unwrap_or_default(),
            event_id: Uuid::parse_str(&row.try_get::<String, _>("event_id").unwrap_or_default()).unwrap_or_default(),
            invited_user_id: {
                let opt_str: Option<String> = row.try_get("invited_user_id").ok();
                opt_str.and_then(|s| Uuid::parse_str(&s).ok())
            },
            invited_contact_id: {
                let opt_str: Option<String> = row.try_get("invited_contact_id").ok();
                opt_str.and_then(|s| Uuid::parse_str(&s).ok())
            },
            invited_email: row.try_get("invited_email").ok(),
            invited_name: row.try_get("invited_name").ok(),
            inviter_id: Uuid::parse_str(&row.try_get::<String, _>("inviter_id").unwrap_or_default()).unwrap_or_default(),
            invitation_method: map_invitation_method(&row.try_get::<String, _>("invitation_method").unwrap_or_else(|_| "email".to_string())),
            personal_message: row.try_get("personal_message").ok(),
            status: map_invitation_status(&row.try_get::<String, _>("status").unwrap_or_else(|_| "pending".to_string())),
            sent_at: optional_datetime_from_naive(row.try_get("sent_at").ok()),
            opened_at: optional_datetime_from_naive(row.try_get("opened_at").ok()),
            responded_at: optional_datetime_from_naive(row.try_get("responded_at").ok()),
            invitation_token: row.try_get("invitation_token").ok(),
            expires_at: optional_datetime_from_naive(row.try_get("expires_at").ok()),
            created_at: datetime_from_naive(row.try_get("created_at").unwrap_or_else(|_| chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc())),
            updated_at: datetime_from_naive(row.try_get("updated_at").unwrap_or_else(|_| chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc())),
        })
    }
}

#[async_trait]
impl EventInvitationRepository for SqliteInvitationRepository {
    #[instrument(skip(self))]
    async fn create(&self, invitation: &EventInvitation) -> DomainResult<()> {
        debug!("Creating invitation: {}", invitation.id);

        invitation.validate_for_creation()?;

        let query = r#"
            INSERT INTO event_invitations (
                id, event_id, invited_user_id, invited_contact_id, 
                invited_email, invited_name, inviter_id, invitation_method,
                personal_message, status, sent_at, opened_at, responded_at,
                invitation_token, expires_at, created_at, updated_at
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
        "#;

        sqlx::query(query)
            .bind(invitation.id.to_string())
            .bind(invitation.event_id.to_string())
            .bind(invitation.invited_user_id.map(|id| id.to_string()))
            .bind(invitation.invited_contact_id.map(|id| id.to_string()))
            .bind(&invitation.invited_email)
            .bind(&invitation.invited_name)
            .bind(invitation.inviter_id.to_string())
            .bind(invitation_method_to_string(&invitation.invitation_method))
            .bind(&invitation.personal_message)
            .bind(invitation_status_to_string(&invitation.status))
            .bind(invitation.sent_at.map(|dt| dt.naive_utc()))
            .bind(invitation.opened_at.map(|dt| dt.naive_utc()))
            .bind(invitation.responded_at.map(|dt| dt.naive_utc()))
            .bind(&invitation.invitation_token)
            .bind(invitation.expires_at.map(|dt| dt.naive_utc()))
            .bind(invitation.created_at.naive_utc())
            .bind(invitation.updated_at.naive_utc())
            .execute(&self.pool)
            .await
            .map_err(InfrastructureError::from)?;

        debug!("Created invitation successfully: {}", invitation.id);
        Ok(())
    }

    #[instrument(skip(self))]
    async fn update(&self, invitation: &EventInvitation) -> DomainResult<()> {
        debug!("Updating invitation: {}", invitation.id);

        invitation.validate_for_creation()?;

        let query = r#"
            UPDATE event_invitations SET 
                event_id = ?, invited_user_id = ?, invited_contact_id = ?,
                invited_email = ?, invited_name = ?, inviter_id = ?,
                invitation_method = ?, personal_message = ?, status = ?,
                sent_at = ?, opened_at = ?, responded_at = ?,
                invitation_token = ?, expires_at = ?, updated_at = ?
            WHERE id = ?
        "#;

        let result = sqlx::query(query)
            .bind(invitation.event_id.to_string())
            .bind(invitation.invited_user_id.map(|id| id.to_string()))
            .bind(invitation.invited_contact_id.map(|id| id.to_string()))
            .bind(&invitation.invited_email)
            .bind(&invitation.invited_name)
            .bind(invitation.inviter_id.to_string())
            .bind(invitation_method_to_string(&invitation.invitation_method))
            .bind(&invitation.personal_message)
            .bind(invitation_status_to_string(&invitation.status))
            .bind(invitation.sent_at.map(|dt| dt.naive_utc()))
            .bind(invitation.opened_at.map(|dt| dt.naive_utc()))
            .bind(invitation.responded_at.map(|dt| dt.naive_utc()))
            .bind(&invitation.invitation_token)
            .bind(invitation.expires_at.map(|dt| dt.naive_utc()))
            .bind(invitation.updated_at.naive_utc())
            .bind(invitation.id.to_string())
            .execute(&self.pool)
            .await
            .map_err(InfrastructureError::from)?;

        if result.rows_affected() == 0 {
            return Err(aqio_core::DomainError::not_found("EventInvitation", invitation.id));
        }

        debug!("Updated invitation successfully: {}", invitation.id);
        Ok(())
    }

    #[instrument(skip(self))]
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<EventInvitation>> {
        debug!("Finding invitation by id: {}", id);

        let query = "SELECT * FROM event_invitations WHERE id = ?";
        
        let row = sqlx::query(query)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(InfrastructureError::from)?;

        match row {
            Some(row) => {
                let invitation = Self::row_to_invitation(&row)?;
                debug!("Found invitation: {}", id);
                Ok(Some(invitation))
            }
            None => {
                debug!("No invitation found with id: {}", id);
                Ok(None)
            }
        }
    }

    #[instrument(skip(self))]
    async fn find_by_event_id(&self, event_id: Uuid) -> DomainResult<Vec<EventInvitation>> {
        debug!("Finding invitations by event: {}", event_id);

        let query = "SELECT * FROM event_invitations WHERE event_id = ? ORDER BY created_at DESC";
        
        let rows = sqlx::query(query)
            .bind(event_id.to_string())
            .fetch_all(&self.pool)
            .await
            .map_err(InfrastructureError::from)?;

        let mut invitations = Vec::new();
        for row in rows {
            invitations.push(Self::row_to_invitation(&row)?);
        }

        debug!("Found {} invitations for event: {}", invitations.len(), event_id);
        Ok(invitations)
    }

    #[instrument(skip(self))]
    async fn find_by_user_id(&self, user_id: Uuid) -> DomainResult<Vec<EventInvitation>> {
        debug!("Finding invitations by user: {}", user_id);

        let query = "SELECT * FROM event_invitations WHERE invited_user_id = ? ORDER BY created_at DESC";
        
        let rows = sqlx::query(query)
            .bind(user_id.to_string())
            .fetch_all(&self.pool)
            .await
            .map_err(InfrastructureError::from)?;

        let mut invitations = Vec::new();
        for row in rows {
            invitations.push(Self::row_to_invitation(&row)?);
        }

        debug!("Found {} invitations for user: {}", invitations.len(), user_id);
        Ok(invitations)
    }

    #[instrument(skip(self))]
    async fn find_by_token(&self, token: &str) -> DomainResult<Option<EventInvitation>> {
        debug!("Finding invitation by token: {}", token);

        let query = "SELECT * FROM event_invitations WHERE invitation_token = ? LIMIT 1";
        let row_result = sqlx::query(query)
            .bind(token)
            .fetch_optional(&self.pool)
            .await;

        match row_result {
            Ok(Some(row)) => {
                let invitation = Self::row_to_invitation(&row)?;
                Ok(Some(invitation))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(InfrastructureError::from(e).into()),
        }
    }

    #[instrument(skip(self))]
    async fn find_by_email(&self, email: &str) -> DomainResult<Vec<EventInvitation>> {
        debug!("Finding invitations by email: {}", email);

        let query = "SELECT * FROM event_invitations WHERE invited_email = ? ORDER BY created_at DESC";
        
        let rows = sqlx::query(query)
            .bind(email)
            .fetch_all(&self.pool)
            .await
            .map_err(InfrastructureError::from)?;

        let mut invitations = Vec::new();
        for row in rows {
            invitations.push(Self::row_to_invitation(&row)?);
        }

        debug!("Found {} invitations for email: {}", invitations.len(), email);
        Ok(invitations)
    }

    #[instrument(skip(self))]
    async fn update_status(&self, invitation_id: Uuid, status: InvitationStatus) -> DomainResult<()> {
        debug!("Updating invitation status: {} -> {:?}", invitation_id, status);

        let query = r#"
            UPDATE event_invitations 
            SET status = ?, updated_at = CURRENT_TIMESTAMP,
                responded_at = CASE WHEN ? IN ('accepted', 'declined') THEN CURRENT_TIMESTAMP ELSE responded_at END
            WHERE id = ?
        "#;

        let status_str = invitation_status_to_string(&status);
        let result = sqlx::query(query)
            .bind(&status_str)
            .bind(&status_str) // For the CASE WHEN condition
            .bind(invitation_id.to_string())
            .execute(&self.pool)
            .await
            .map_err(InfrastructureError::from)?;

        if result.rows_affected() == 0 {
            return Err(aqio_core::DomainError::not_found("EventInvitation", invitation_id));
        }

        debug!("Updated invitation status successfully: {}", invitation_id);
        Ok(())
    }

    #[instrument(skip(self))]
    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        debug!("Deleting invitation: {}", id);

        let query = "DELETE FROM event_invitations WHERE id = ?";
        
        let result = sqlx::query(query)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(InfrastructureError::from)?;

        if result.rows_affected() == 0 {
            return Err(aqio_core::DomainError::not_found("EventInvitation", id));
        }

        debug!("Deleted invitation successfully: {}", id);
        Ok(())
    }

    #[instrument(skip(self))]
    async fn exists(&self, id: Uuid) -> DomainResult<bool> {
        debug!("Checking if invitation exists: {}", id);

        let query = "SELECT 1 FROM event_invitations WHERE id = ? LIMIT 1";
        
        let row = sqlx::query(query)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(InfrastructureError::from)?;

        let exists = row.is_some();
        debug!("Invitation {} exists: {}", id, exists);
        Ok(exists)
    }

    #[instrument(skip(self))]
    async fn user_invited_to_event(&self, user_id: Uuid, event_id: Uuid) -> DomainResult<bool> {
        debug!("Checking if user {} is invited to event {}", user_id, event_id);

        let query = "SELECT 1 FROM event_invitations WHERE invited_user_id = ? AND event_id = ? LIMIT 1";
        
        let row = sqlx::query(query)
            .bind(user_id.to_string())
            .bind(event_id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(InfrastructureError::from)?;

        let invited = row.is_some();
        debug!("User {} invited to event {}: {}", user_id, event_id, invited);
        Ok(invited)
    }

    #[instrument(skip(self))]
    async fn email_invited_to_event(&self, email: &str, event_id: Uuid) -> DomainResult<bool> {
        debug!("Checking if email {} is invited to event {}", email, event_id);

        let query = "SELECT 1 FROM event_invitations WHERE invited_email = ? AND event_id = ? LIMIT 1";
        
        let row = sqlx::query(query)
            .bind(email)
            .bind(event_id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(InfrastructureError::from)?;

        let invited = row.is_some();
        debug!("Email {} invited to event {}: {}", email, event_id, invited);
        Ok(invited)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aqio_core::{InvitationMethod, InvitationStatus};
    use sqlx::SqlitePool;
    use uuid::Uuid;
    use chrono::Utc;

    async fn setup_test_db() -> Pool<Sqlite> {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        
        // Create required tables for testing (without foreign key constraints for simplicity)
        sqlx::query(r#"
            CREATE TABLE event_invitations (
                id TEXT PRIMARY KEY,
                event_id TEXT NOT NULL,
                invited_user_id TEXT,
                invited_contact_id TEXT,
                invited_email TEXT,
                invited_name TEXT,
                inviter_id TEXT NOT NULL,
                invitation_method TEXT NOT NULL DEFAULT 'email',
                personal_message TEXT,
                status TEXT NOT NULL DEFAULT 'pending',
                sent_at DATETIME,
                opened_at DATETIME,
                responded_at DATETIME,
                invitation_token TEXT UNIQUE,
                expires_at DATETIME,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
        "#)
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_create_and_find_invitation() {
        let pool = setup_test_db().await;
        let repo = SqliteInvitationRepository::new(pool);
        
        let invitation_id = Uuid::new_v4();
        let event_id = Uuid::new_v4();
        let inviter_id = Uuid::new_v4();
        
        let invitation = EventInvitation {
            id: invitation_id,
            event_id,
            invited_user_id: None,
            invited_contact_id: None,
            invited_email: Some("test@example.com".to_string()),
            invited_name: Some("Test User".to_string()),
            inviter_id,
            invitation_method: InvitationMethod::Email,
            personal_message: Some("Please join us!".to_string()),
            status: InvitationStatus::Pending,
            sent_at: None,
            opened_at: None,
            responded_at: None,
            invitation_token: None,
            expires_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Test creation
        repo.create(&invitation).await.unwrap();
        
        // Test finding by id
        let found = repo.find_by_id(invitation_id).await.unwrap().unwrap();
        assert_eq!(found.id, invitation_id);
        assert_eq!(found.invited_email, Some("test@example.com".to_string()));
        
        // Test finding by event
    let event_invitations = repo.find_by_event_id(event_id).await.unwrap();
        assert_eq!(event_invitations.len(), 1);
        assert_eq!(event_invitations[0].id, invitation_id);
        
        // Test finding by email
        let email_invitations = repo.find_by_email("test@example.com").await.unwrap();
        assert_eq!(email_invitations.len(), 1);
        assert_eq!(email_invitations[0].id, invitation_id);
    }

    #[tokio::test]
    async fn test_update_status() {
        let pool = setup_test_db().await;
        let repo = SqliteInvitationRepository::new(pool);
        
        let invitation_id = Uuid::new_v4();
        let event_id = Uuid::new_v4();
        let inviter_id = Uuid::new_v4();
        
        let invitation = EventInvitation {
            id: invitation_id,
            event_id,
            invited_user_id: None,
            invited_contact_id: None,
            invited_email: Some("test@example.com".to_string()),
            invited_name: Some("Test User".to_string()),
            inviter_id,
            invitation_method: InvitationMethod::Email,
            personal_message: None,
            status: InvitationStatus::Pending,
            sent_at: None,
            opened_at: None,
            responded_at: None,
            invitation_token: None,
            expires_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        repo.create(&invitation).await.unwrap();
        
        // Update status to accepted
        repo.update_status(invitation_id, InvitationStatus::Accepted).await.unwrap();
        
        // Verify the update
        let updated = repo.find_by_id(invitation_id).await.unwrap().unwrap();
        assert_eq!(updated.status, InvitationStatus::Accepted);
        assert!(updated.responded_at.is_some());
    }
    
    #[tokio::test]
    async fn test_duplicate_invitations() {
        let pool = setup_test_db().await;
        let repo = SqliteInvitationRepository::new(pool);
        
        let event_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let inviter_id = Uuid::new_v4();
        
        let invitation = EventInvitation {
            id: Uuid::new_v4(),
            event_id,
            invited_user_id: Some(user_id),
            invited_contact_id: None,
            invited_email: None,
            invited_name: None,
            inviter_id,
            invitation_method: InvitationMethod::Email,
            personal_message: None,
            status: InvitationStatus::Pending,
            sent_at: None,
            opened_at: None,
            responded_at: None,
            invitation_token: None,
            expires_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        repo.create(&invitation).await.unwrap();
        
        // Check if user is already invited
        let is_invited = repo.user_invited_to_event(user_id, event_id).await.unwrap();
        assert!(is_invited);
        
        // Check if email is invited (should be false)
        let email_invited = repo.email_invited_to_event("test@example.com", event_id).await.unwrap();
        assert!(!email_invited);
    }
}