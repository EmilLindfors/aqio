use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::domain::errors::{InfrastructureError, SqliteForeignKeyDiagnostic};
use aqio_core::{
    DomainError, DomainResult, EventRegistration, EventRegistrationRepository,
    RegistrationSource, RegistrationStatus
};

#[derive(Clone)]
pub struct SqliteEventRegistrationRepository {
    pool: SqlitePool,
}

impl SqliteEventRegistrationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // Helper function to convert NaiveDateTime to DateTime<Utc>
    fn naive_to_utc(naive: NaiveDateTime) -> DateTime<Utc> {
        naive.and_utc()
    }

    // Helper function to convert Optional NaiveDateTime to Optional DateTime<Utc>
    fn optional_naive_to_utc(naive: Option<NaiveDateTime>) -> Option<DateTime<Utc>> {
        naive.map(|n| n.and_utc())
    }

    // Helper function to map string to RegistrationStatus
    fn map_registration_status(status_str: &str) -> RegistrationStatus {
        match status_str.to_lowercase().as_str() {
            "registered" => RegistrationStatus::Registered,
            "waitlisted" => RegistrationStatus::Waitlisted,
            "cancelled" => RegistrationStatus::Cancelled,
            "attended" => RegistrationStatus::Attended,
            "no_show" => RegistrationStatus::NoShow,
            _ => RegistrationStatus::Registered, // Default fallback
        }
    }

    // Helper function to map RegistrationStatus to string
    fn status_to_string(status: &RegistrationStatus) -> &'static str {
        match status {
            RegistrationStatus::Registered => "registered",
            RegistrationStatus::Waitlisted => "waitlisted",
            RegistrationStatus::Cancelled => "cancelled",
            RegistrationStatus::Attended => "attended",
            RegistrationStatus::NoShow => "no_show",
        }
    }

    // Helper function to map string to RegistrationSource
    fn map_registration_source(source_str: &str) -> RegistrationSource {
        match source_str.to_lowercase().as_str() {
            "invitation" => RegistrationSource::Invitation,
            "direct" => RegistrationSource::Direct,
            "waitlist_promotion" => RegistrationSource::WaitlistPromotion,
            _ => RegistrationSource::Direct, // Default fallback
        }
    }

    // Helper function to map RegistrationSource to string
    fn source_to_string(source: &RegistrationSource) -> &'static str {
        match source {
            RegistrationSource::Invitation => "invitation",
            RegistrationSource::Direct => "direct",
            RegistrationSource::WaitlistPromotion => "waitlist_promotion",
        }
    }

    // Helper function to build EventRegistration from database row
    // Note: Some fields are NOT NULL in database so they come as String/i64/NaiveDateTime
    // Others are nullable so they come as Option<T>
    fn build_registration_from_row(
        id: String,                        // NOT NULL
        event_id: String,                  // NOT NULL
        invitation_id: Option<String>,
        user_id: Option<String>,
        external_contact_id: Option<String>,
        registrant_email: Option<String>,
        registrant_name: Option<String>,
        registrant_phone: Option<String>,
        registrant_company: Option<String>,
        status: String,                    // NOT NULL
        registration_source: String,       // NOT NULL
        guest_count: i64,                  // NOT NULL
        guest_names: Option<String>,
        dietary_restrictions: Option<String>,
        accessibility_needs: Option<String>,
        special_requests: Option<String>,
        custom_responses: Option<String>,
        registered_at: NaiveDateTime,      // NOT NULL
        cancelled_at: Option<NaiveDateTime>,
        checked_in_at: Option<NaiveDateTime>,
        waitlist_position: Option<i64>,
        waitlist_added_at: Option<NaiveDateTime>,
        created_at: NaiveDateTime,         // NOT NULL
        updated_at: NaiveDateTime,         // NOT NULL
    ) -> DomainResult<EventRegistration> {
        let registration_id = Uuid::parse_str(&id)
            .map_err(|e| DomainError::business_rule(&format!("Invalid UUID format for registration ID: {}", e)))?;
        let event_id = Uuid::parse_str(&event_id)
            .map_err(|e| DomainError::business_rule(&format!("Invalid UUID format for event ID: {}", e)))?;
        
        let invitation_id = match invitation_id {
            Some(id_str) => Some(Uuid::parse_str(&id_str)
                .map_err(|e| DomainError::business_rule(&format!("Invalid UUID format for invitation ID: {}", e)))?),
            None => None,
        };
        
        let user_id = match user_id {
            Some(id_str) => Some(Uuid::parse_str(&id_str)
                .map_err(|e| DomainError::business_rule(&format!("Invalid UUID format for user ID: {}", e)))?),
            None => None,
        };
        
        let external_contact_id = match external_contact_id {
            Some(id_str) => Some(Uuid::parse_str(&id_str)
                .map_err(|e| DomainError::business_rule(&format!("Invalid UUID format for external contact ID: {}", e)))?),
            None => None,
        };

        // Parse guest_names JSON
        let guest_names: Vec<String> = match guest_names {
            Some(names_json) => serde_json::from_str(&names_json).unwrap_or_default(),
            None => vec![],
        };

        Ok(EventRegistration {
            id: registration_id,
            event_id,
            invitation_id,
            user_id,
            external_contact_id,
            registrant_email,
            registrant_name,
            registrant_phone,
            registrant_company,
            status: Self::map_registration_status(&status),
            registration_source: Self::map_registration_source(&registration_source),
            guest_count: guest_count as i32,
            guest_names,
            dietary_restrictions,
            accessibility_needs,
            special_requests,
            custom_responses,
            registered_at: Self::naive_to_utc(registered_at),
            cancelled_at: Self::optional_naive_to_utc(cancelled_at),
            checked_in_at: Self::optional_naive_to_utc(checked_in_at),
            waitlist_position: waitlist_position.map(|pos| pos as i32),
            waitlist_added_at: Self::optional_naive_to_utc(waitlist_added_at),
            created_at: Self::naive_to_utc(created_at),
            updated_at: Self::naive_to_utc(updated_at),
        })
    }

    // Diagnose which foreign key constraint is failing by checking if referenced entities exist
    async fn diagnose_foreign_key_violation(&self, registration: &EventRegistration, _db_message: &str) -> aqio_core::DomainError {
        let diagnostic = SqliteForeignKeyDiagnostic::new(self.pool.clone());

        // Check if event exists
        let event_exists = diagnostic.check_event_exists(registration.event_id).await;
        if !event_exists {
            return SqliteForeignKeyDiagnostic::create_user_friendly_foreign_key_error(
                "Registration",
                "event_id",
                &registration.event_id.to_string(),
            );
        }

        // Check if user exists (if provided)
        if let Some(user_id) = registration.user_id {
            let user_exists = diagnostic.check_user_exists(user_id).await;
            if !user_exists {
                return SqliteForeignKeyDiagnostic::create_user_friendly_foreign_key_error(
                    "Registration",
                    "user_id",
                    &user_id.to_string(),
                );
            }
        }

        // Check if invitation exists (if provided)
        if let Some(invitation_id) = registration.invitation_id {
            let invitation_exists = diagnostic.check_invitation_exists(invitation_id).await;
            if !invitation_exists {
                return SqliteForeignKeyDiagnostic::create_user_friendly_foreign_key_error(
                    "Registration",
                    "invitation_id",
                    &invitation_id.to_string(),
                );
            }
        }

        // If we get here, it's some other foreign key constraint we don't know about
        aqio_core::DomainError::BusinessRuleViolation {
            message: "Foreign key constraint violation: Unknown referenced entity does not exist".to_string(),
        }
    }
}

#[async_trait]
impl EventRegistrationRepository for SqliteEventRegistrationRepository {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<EventRegistration>> {
        let id_str = id.to_string();
        let result = sqlx::query!(
            r#"
            SELECT 
                id, event_id, invitation_id, user_id, external_contact_id,
                registrant_email, registrant_name, registrant_phone, registrant_company,
                status, registration_source,
                guest_count, guest_names,
                dietary_restrictions, accessibility_needs, special_requests, custom_responses,
                registered_at, cancelled_at, checked_in_at,
                waitlist_position, waitlist_added_at,
                created_at, updated_at
            FROM event_registrations 
            WHERE id = ?
            "#,
            id_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::business_rule(&format!("Failed to fetch registration: {}", e)))?;

        match result {
            Some(row) => {
                let registration = Self::build_registration_from_row(
                    row.id.unwrap_or_else(|| "".to_string()),
                    row.event_id,
                    row.invitation_id,
                    row.user_id,
                    row.external_contact_id,
                    row.registrant_email,
                    row.registrant_name,
                    row.registrant_phone,
                    row.registrant_company,
                    row.status,
                    row.registration_source,
                    row.guest_count,
                    row.guest_names,
                    row.dietary_restrictions,
                    row.accessibility_needs,
                    row.special_requests,
                    row.custom_responses,
                    row.registered_at,
                    row.cancelled_at,
                    row.checked_in_at,
                    row.waitlist_position,
                    row.waitlist_added_at,
                    row.created_at,
                    row.updated_at,
                )?;
                Ok(Some(registration))
            }
            None => Ok(None),
        }
    }

    async fn find_by_event_id(&self, event_id: Uuid) -> DomainResult<Vec<EventRegistration>> {
        let event_id_str = event_id.to_string();
        let results = sqlx::query!(
            r#"
            SELECT 
                id, event_id, invitation_id, user_id, external_contact_id,
                registrant_email, registrant_name, registrant_phone, registrant_company,
                status, registration_source,
                guest_count, guest_names,
                dietary_restrictions, accessibility_needs, special_requests, custom_responses,
                registered_at, cancelled_at, checked_in_at,
                waitlist_position, waitlist_added_at,
                created_at, updated_at
            FROM event_registrations 
            WHERE event_id = ?
            ORDER BY registered_at ASC
            "#,
            event_id_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::business_rule(&format!("Failed to fetch registrations by event: {}", e)))?;

        let mut registrations = Vec::new();
        for row in results {
            let registration = Self::build_registration_from_row(
                row.id.unwrap_or_else(|| "".to_string()),
                row.event_id,
                row.invitation_id,
                row.user_id,
                row.external_contact_id,
                row.registrant_email,
                row.registrant_name,
                row.registrant_phone,
                row.registrant_company,
                row.status,
                row.registration_source,
                row.guest_count,
                row.guest_names,
                row.dietary_restrictions,
                row.accessibility_needs,
                row.special_requests,
                row.custom_responses,
                row.registered_at,
                row.cancelled_at,
                row.checked_in_at,
                row.waitlist_position,
                row.waitlist_added_at,
                row.created_at,
                row.updated_at,
            )?;
            registrations.push(registration);
        }

        Ok(registrations)
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> DomainResult<Vec<EventRegistration>> {
        let user_id_str = user_id.to_string();
        let results = sqlx::query!(
            r#"
            SELECT 
                id, event_id, invitation_id, user_id, external_contact_id,
                registrant_email, registrant_name, registrant_phone, registrant_company,
                status, registration_source,
                guest_count, guest_names,
                dietary_restrictions, accessibility_needs, special_requests, custom_responses,
                registered_at, cancelled_at, checked_in_at,
                waitlist_position, waitlist_added_at,
                created_at, updated_at
            FROM event_registrations 
            WHERE user_id = ?
            ORDER BY registered_at DESC
            "#,
            user_id_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::business_rule(&format!("Failed to fetch registrations by user: {}", e)))?;

        let mut registrations = Vec::new();
        for row in results {
            let registration = Self::build_registration_from_row(
                row.id.unwrap_or_else(|| "".to_string()),
                row.event_id,
                row.invitation_id,
                row.user_id,
                row.external_contact_id,
                row.registrant_email,
                row.registrant_name,
                row.registrant_phone,
                row.registrant_company,
                row.status,
                row.registration_source,
                row.guest_count,
                row.guest_names,
                row.dietary_restrictions,
                row.accessibility_needs,
                row.special_requests,
                row.custom_responses,
                row.registered_at,
                row.cancelled_at,
                row.checked_in_at,
                row.waitlist_position,
                row.waitlist_added_at,
                row.created_at,
                row.updated_at,
            )?;
            registrations.push(registration);
        }

        Ok(registrations)
    }

    async fn find_by_event_and_user(&self, event_id: Uuid, user_id: Uuid) -> DomainResult<Option<EventRegistration>> {
        let event_id_str = event_id.to_string();
        let user_id_str = user_id.to_string();
        let result = sqlx::query!(
            r#"
            SELECT 
                id, event_id, invitation_id, user_id, external_contact_id,
                registrant_email, registrant_name, registrant_phone, registrant_company,
                status, registration_source,
                guest_count, guest_names,
                dietary_restrictions, accessibility_needs, special_requests, custom_responses,
                registered_at, cancelled_at, checked_in_at,
                waitlist_position, waitlist_added_at,
                created_at, updated_at
            FROM event_registrations 
            WHERE event_id = ? AND user_id = ?
            "#,
            event_id_str,
            user_id_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::business_rule(&format!("Failed to fetch registration by event and user: {}", e)))?;

        match result {
            Some(row) => {
                let registration = Self::build_registration_from_row(
                    row.id.unwrap_or_else(|| "".to_string()),
                    row.event_id,
                    row.invitation_id,
                    row.user_id,
                    row.external_contact_id,
                    row.registrant_email,
                    row.registrant_name,
                    row.registrant_phone,
                    row.registrant_company,
                    row.status,
                    row.registration_source,
                    row.guest_count,
                    row.guest_names,
                    row.dietary_restrictions,
                    row.accessibility_needs,
                    row.special_requests,
                    row.custom_responses,
                    row.registered_at,
                    row.cancelled_at,
                    row.checked_in_at,
                    row.waitlist_position,
                    row.waitlist_added_at,
                    row.created_at,
                    row.updated_at,
                )?;
                Ok(Some(registration))
            }
            None => Ok(None),
        }
    }

    async fn create(&self, registration: &EventRegistration) -> DomainResult<()> {
        let guest_names_json = serde_json::to_string(&registration.guest_names)
            .map_err(|e| DomainError::business_rule(&format!("Failed to serialize guest names: {}", e)))?;

        // Convert values to proper types and create owned strings for lifetimes
        let id_str = registration.id.to_string();
        let event_id_str = registration.event_id.to_string();
        let invitation_id_str = registration.invitation_id.as_ref().map(|id| id.to_string());
        let user_id_str = registration.user_id.as_ref().map(|id| id.to_string());
        let external_contact_id_str = registration.external_contact_id.as_ref().map(|id| id.to_string());
        let status_str = Self::status_to_string(&registration.status).to_string();
        let source_str = Self::source_to_string(&registration.registration_source).to_string();
        let guest_count_i64 = registration.guest_count as i64;
        let registered_at_naive = registration.registered_at.naive_utc();
        let cancelled_at_naive = registration.cancelled_at.map(|dt| dt.naive_utc());
        let checked_in_at_naive = registration.checked_in_at.map(|dt| dt.naive_utc());
        let waitlist_position_i64 = registration.waitlist_position.map(|pos| pos as i64);
        let waitlist_added_at_naive = registration.waitlist_added_at.map(|dt| dt.naive_utc());
        let created_at_naive = registration.created_at.naive_utc();
        let updated_at_naive = registration.updated_at.naive_utc();

        let result = sqlx::query!(
            r#"
            INSERT INTO event_registrations (
                id, event_id, invitation_id, user_id, external_contact_id,
                registrant_email, registrant_name, registrant_phone, registrant_company,
                status, registration_source,
                guest_count, guest_names,
                dietary_restrictions, accessibility_needs, special_requests, custom_responses,
                registered_at, cancelled_at, checked_in_at,
                waitlist_position, waitlist_added_at,
                created_at, updated_at
            ) VALUES (
                ?, ?, ?, ?, ?,
                ?, ?, ?, ?,
                ?, ?,
                ?, ?,
                ?, ?, ?, ?,
                ?, ?, ?,
                ?, ?,
                ?, ?
            )
            "#,
            id_str,
            event_id_str,
            invitation_id_str,
            user_id_str,
            external_contact_id_str,
            registration.registrant_email,
            registration.registrant_name,
            registration.registrant_phone,
            registration.registrant_company,
            status_str,
            source_str,
            guest_count_i64,
            guest_names_json,
            registration.dietary_restrictions,
            registration.accessibility_needs,
            registration.special_requests,
            registration.custom_responses,
            registered_at_naive,
            cancelled_at_naive,
            checked_in_at_naive,
            waitlist_position_i64,
            waitlist_added_at_naive,
            created_at_naive,
            updated_at_naive,
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                let infrastructure_error = InfrastructureError::from(e);
                match infrastructure_error {
                    InfrastructureError::DomainError { source } => Err(source),
                    InfrastructureError::ForeignKeyConstraintViolation { message } => {
                        // We have context about what we were trying to insert
                        let specific_error = self.diagnose_foreign_key_violation(&registration, &message).await;
                        Err(specific_error)
                    }
                    other => Err(other.into()),
                }
            }
        }
    }

    async fn update(&self, registration: &EventRegistration) -> DomainResult<()> {
        let guest_names_json = serde_json::to_string(&registration.guest_names)
            .map_err(|e| DomainError::business_rule(&format!("Failed to serialize guest names: {}", e)))?;

        // Convert values to proper types and create owned strings for lifetimes
        let id_str = registration.id.to_string();
        let invitation_id_str = registration.invitation_id.as_ref().map(|id| id.to_string());
        let user_id_str = registration.user_id.as_ref().map(|id| id.to_string());
        let external_contact_id_str = registration.external_contact_id.as_ref().map(|id| id.to_string());
        let status_str = Self::status_to_string(&registration.status).to_string();
        let source_str = Self::source_to_string(&registration.registration_source).to_string();
        let guest_count_i64 = registration.guest_count as i64;
        let registered_at_naive = registration.registered_at.naive_utc();
        let cancelled_at_naive = registration.cancelled_at.map(|dt| dt.naive_utc());
        let checked_in_at_naive = registration.checked_in_at.map(|dt| dt.naive_utc());
        let waitlist_position_i64 = registration.waitlist_position.map(|pos| pos as i64);
        let waitlist_added_at_naive = registration.waitlist_added_at.map(|dt| dt.naive_utc());
        let updated_at_naive = registration.updated_at.naive_utc();

        let result = sqlx::query!(
            r#"
            UPDATE event_registrations SET
                invitation_id = ?, user_id = ?, external_contact_id = ?,
                registrant_email = ?, registrant_name = ?, registrant_phone = ?, registrant_company = ?,
                status = ?, registration_source = ?,
                guest_count = ?, guest_names = ?,
                dietary_restrictions = ?, accessibility_needs = ?, special_requests = ?, custom_responses = ?,
                registered_at = ?, cancelled_at = ?, checked_in_at = ?,
                waitlist_position = ?, waitlist_added_at = ?,
                updated_at = ?
            WHERE id = ?
            "#,
            invitation_id_str,
            user_id_str,
            external_contact_id_str,
            registration.registrant_email,
            registration.registrant_name,
            registration.registrant_phone,
            registration.registrant_company,
            status_str,
            source_str,
            guest_count_i64,
            guest_names_json,
            registration.dietary_restrictions,
            registration.accessibility_needs,
            registration.special_requests,
            registration.custom_responses,
            registered_at_naive,
            cancelled_at_naive,
            checked_in_at_naive,
            waitlist_position_i64,
            waitlist_added_at_naive,
            updated_at_naive,
            id_str,
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(rows_affected) => {
                if rows_affected.rows_affected() == 0 {
                    Err(DomainError::not_found("EventRegistration", registration.id))
                } else {
                    Ok(())
                }
            }
            Err(e) => {
                let infrastructure_error = InfrastructureError::from(e);
                match infrastructure_error {
                    InfrastructureError::DomainError { source } => Err(source),
                    InfrastructureError::ForeignKeyConstraintViolation { message } => {
                        // We have context about what we were trying to update
                        let specific_error = self.diagnose_foreign_key_violation(&registration, &message).await;
                        Err(specific_error)
                    }
                    other => Err(other.into()),
                }
            }
        }
    }

    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        let id_str = id.to_string();
        let rows_affected = sqlx::query!(
            "DELETE FROM event_registrations WHERE id = ?",
            id_str
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::business_rule(&format!("Failed to delete registration: {}", e)))?;

        if rows_affected.rows_affected() == 0 {
            return Err(DomainError::not_found("EventRegistration", id));
        }

        Ok(())
    }
}