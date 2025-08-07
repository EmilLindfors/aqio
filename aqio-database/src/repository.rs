use anyhow::Result;
use aqio_core::models::*;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

pub struct EventRepository {
    pool: Pool<Sqlite>,
}

impl EventRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn create_event(&self, event: &Event) -> Result<()> {
        let (event_type_str, event_type_other) = match &event.event_type {
            EventType::Conference => ("Conference", None),
            EventType::Workshop => ("Workshop", None),
            EventType::Networking => ("Networking", None),
            EventType::Training => ("Training", None),
            EventType::Personal => ("Personal", None),
            EventType::Other(value) => ("Other", Some(value.as_str())),
        };

        let event_id = event.id.to_string();
        let organizer_id = event.organizer_id.to_string();
        let start_date = event.start_date.naive_utc();
        let end_date = event.end_date.naive_utc();
        let registration_deadline = event.registration_deadline.map(|dt| dt.naive_utc());
        let created_at = event.created_at.naive_utc();
        let updated_at = event.updated_at.naive_utc();

        sqlx::query!(
            r#"
            INSERT INTO events (id, title, description, event_type, event_type_other, 
                              start_date, end_date, location, organizer_id, max_attendees, 
                              registration_deadline, is_private, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
            "#,
            event_id,
            event.title,
            event.description,
            event_type_str,
            event_type_other,
            start_date,
            end_date,
            event.location,
            organizer_id,
            event.max_attendees,
            registration_deadline,
            event.is_private,
            created_at,
            updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_event_by_id(&self, id: Uuid) -> Result<Option<Event>> {
        let id_string = id.to_string();
        let row = sqlx::query!("SELECT * FROM events WHERE id = ?", id_string)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let event_type = match row.event_type.as_str() {
                "Conference" => EventType::Conference,
                "Workshop" => EventType::Workshop,
                "Networking" => EventType::Networking,
                "Training" => EventType::Training,
                "Personal" => EventType::Personal,
                "Other" => EventType::Other(row.event_type_other.unwrap_or_default()),
                _ => EventType::Other("Unknown".to_string()),
            };

            Ok(Some(Event {
                id: Uuid::parse_str(row.id.as_deref().unwrap_or(""))?,
                title: row.title,
                description: row.description,
                event_type,
                start_date: DateTime::from_naive_utc_and_offset(row.start_date, Utc),
                end_date: DateTime::from_naive_utc_and_offset(row.end_date, Utc),
                location: row.location,
                organizer_id: Uuid::parse_str(&row.organizer_id)?,
                max_attendees: row.max_attendees.map(|x| x as i32),
                registration_deadline: row
                    .registration_deadline
                    .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                is_private: row.is_private,
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                updated_at: DateTime::from_naive_utc_and_offset(row.updated_at, Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn list_events(&self) -> Result<Vec<Event>> {
        let rows = sqlx::query!("SELECT * FROM events ORDER BY start_date DESC")
            .fetch_all(&self.pool)
            .await?;

        let mut events = Vec::new();
        for row in rows {
            let event_type = match row.event_type.as_str() {
                "Conference" => EventType::Conference,
                "Workshop" => EventType::Workshop,
                "Networking" => EventType::Networking,
                "Training" => EventType::Training,
                "Personal" => EventType::Personal,
                "Other" => EventType::Other(row.event_type_other.unwrap_or_default()),
                _ => EventType::Other("Unknown".to_string()),
            };

            events.push(Event {
                id: Uuid::parse_str(row.id.as_deref().unwrap_or(""))?,
                title: row.title,
                description: row.description,
                event_type,
                start_date: DateTime::from_naive_utc_and_offset(row.start_date, Utc),
                end_date: DateTime::from_naive_utc_and_offset(row.end_date, Utc),
                location: row.location,
                organizer_id: Uuid::parse_str(&row.organizer_id)?,
                max_attendees: row.max_attendees.map(|x| x as i32),
                registration_deadline: row
                    .registration_deadline
                    .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                is_private: row.is_private,
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                updated_at: DateTime::from_naive_utc_and_offset(row.updated_at, Utc),
            });
        }

        Ok(events)
    }
}

pub struct UserRepository {
    pool: Pool<Sqlite>,
}

impl UserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: &User) -> Result<()> {
        let user_id = user.id.to_string();
        let company_id = user.company_id.map(|id| id.to_string());
        let created_at = user.created_at.naive_utc();
        let updated_at = user.updated_at.naive_utc();

        sqlx::query!(
            r#"
            INSERT INTO users (id, keycloak_id, email, name, company_id, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            user_id,
            user.keycloak_id,
            user.email,
            user.name,
            company_id,
            created_at,
            updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>> {
        let row = sqlx::query!("SELECT * FROM users WHERE keycloak_id = ?", keycloak_id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            Ok(Some(User {
                id: Uuid::parse_str(row.id.as_deref().unwrap_or(""))?,
                keycloak_id: row.keycloak_id,
                email: row.email,
                name: row.name,
                company_id: row
                    .company_id
                    .as_ref()
                    .map(|id| Uuid::parse_str(id))
                    .transpose()?,
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                updated_at: DateTime::from_naive_utc_and_offset(row.updated_at, Utc),
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct InvitationRepository {
    pool: Pool<Sqlite>,
}

impl InvitationRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn create_invitation(&self, invitation: &EventInvitation) -> Result<()> {
        let invitation_id = invitation.id.to_string();
        let event_id = invitation.event_id.to_string();
        let invited_user_id = invitation.invited_user_id.map(|id| id.to_string());
        let inviter_id = invitation.inviter_id.to_string();
        let status = match invitation.status {
            InvitationStatus::Pending => "pending",
            InvitationStatus::Accepted => "accepted",
            InvitationStatus::Declined => "declined",
        };
        let invited_at = invitation.invited_at.naive_utc();
        let responded_at = invitation.responded_at.map(|dt| dt.naive_utc());

        sqlx::query!(
            r#"
            INSERT INTO event_invitations (id, event_id, invited_user_id, invited_email, invited_name,
                                         inviter_id, status, invitation_message, invited_at, responded_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
            invitation_id,
            event_id,
            invited_user_id,
            invitation.invited_email,
            invitation.invited_name,
            inviter_id,
            status,
            invitation.invitation_message,
            invited_at,
            responded_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_invitations_for_event(&self, event_id: Uuid) -> Result<Vec<EventInvitation>> {
        let event_id_string = event_id.to_string();
        let rows = sqlx::query!(
            "SELECT * FROM event_invitations WHERE event_id = ? ORDER BY invited_at DESC",
            event_id_string
        )
        .fetch_all(&self.pool)
        .await?;

        let mut invitations = Vec::new();
        for row in rows {
            let status = match row.status.as_str() {
                "pending" => InvitationStatus::Pending,
                "accepted" => InvitationStatus::Accepted,
                "declined" => InvitationStatus::Declined,
                _ => InvitationStatus::Pending,
            };

            invitations.push(EventInvitation {
                id: Uuid::parse_str(row.id.as_deref().unwrap_or(""))?,
                event_id: Uuid::parse_str(&row.event_id)?,
                invited_user_id: row
                    .invited_user_id
                    .as_ref()
                    .map(|id| Uuid::parse_str(id))
                    .transpose()?,
                invited_email: row.invited_email,
                invited_name: row.invited_name,
                inviter_id: Uuid::parse_str(&row.inviter_id)?,
                status,
                invitation_message: row.invitation_message,
                invited_at: DateTime::from_naive_utc_and_offset(row.invited_at, Utc),
                responded_at: row
                    .responded_at
                    .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            });
        }

        Ok(invitations)
    }

    pub async fn get_invitations_for_user(&self, user_id: Uuid) -> Result<Vec<EventInvitation>> {
        let user_id_string = user_id.to_string();
        let rows = sqlx::query!(
            "SELECT * FROM event_invitations WHERE invited_user_id = ? ORDER BY invited_at DESC",
            user_id_string
        )
        .fetch_all(&self.pool)
        .await?;

        let mut invitations = Vec::new();
        for row in rows {
            let status = match row.status.as_str() {
                "pending" => InvitationStatus::Pending,
                "accepted" => InvitationStatus::Accepted,
                "declined" => InvitationStatus::Declined,
                _ => InvitationStatus::Pending,
            };

            invitations.push(EventInvitation {
                id: Uuid::parse_str(row.id.as_deref().unwrap_or(""))?,
                event_id: Uuid::parse_str(&row.event_id)?,
                invited_user_id: row
                    .invited_user_id
                    .as_ref()
                    .map(|id| Uuid::parse_str(id))
                    .transpose()?,
                invited_email: row.invited_email,
                invited_name: row.invited_name,
                inviter_id: Uuid::parse_str(&row.inviter_id)?,
                status,
                invitation_message: row.invitation_message,
                invited_at: DateTime::from_naive_utc_and_offset(row.invited_at, Utc),
                responded_at: row
                    .responded_at
                    .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            });
        }

        Ok(invitations)
    }

    pub async fn get_invitations_by_email(&self, email: &str) -> Result<Vec<EventInvitation>> {
        let rows = sqlx::query!(
            "SELECT * FROM event_invitations WHERE invited_email = ? ORDER BY invited_at DESC",
            email
        )
        .fetch_all(&self.pool)
        .await?;

        let mut invitations = Vec::new();
        for row in rows {
            let status = match row.status.as_str() {
                "pending" => InvitationStatus::Pending,
                "accepted" => InvitationStatus::Accepted,
                "declined" => InvitationStatus::Declined,
                _ => InvitationStatus::Pending,
            };

            invitations.push(EventInvitation {
                id: Uuid::parse_str(row.id.as_deref().unwrap_or(""))?,
                event_id: Uuid::parse_str(&row.event_id)?,
                invited_user_id: row
                    .invited_user_id
                    .as_ref()
                    .map(|id| Uuid::parse_str(id))
                    .transpose()?,
                invited_email: row.invited_email,
                invited_name: row.invited_name,
                inviter_id: Uuid::parse_str(&row.inviter_id)?,
                status,
                invitation_message: row.invitation_message,
                invited_at: DateTime::from_naive_utc_and_offset(row.invited_at, Utc),
                responded_at: row
                    .responded_at
                    .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            });
        }

        Ok(invitations)
    }

    pub async fn update_invitation_status(&self, invitation_id: Uuid, status: InvitationStatus) -> Result<()> {
        let invitation_id_string = invitation_id.to_string();
        let status_str = match status {
            InvitationStatus::Pending => "pending",
            InvitationStatus::Accepted => "accepted",
            InvitationStatus::Declined => "declined",
        };
        let responded_at = chrono::Utc::now().naive_utc();

        sqlx::query!(
            "UPDATE event_invitations SET status = ?, responded_at = ? WHERE id = ?",
            status_str,
            responded_at,
            invitation_id_string
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_invitation_by_id(&self, invitation_id: Uuid) -> Result<Option<EventInvitation>> {
        let invitation_id_string = invitation_id.to_string();
        let row = sqlx::query!(
            "SELECT * FROM event_invitations WHERE id = ?",
            invitation_id_string
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let status = match row.status.as_str() {
                "pending" => InvitationStatus::Pending,
                "accepted" => InvitationStatus::Accepted,
                "declined" => InvitationStatus::Declined,
                _ => InvitationStatus::Pending,
            };

            Ok(Some(EventInvitation {
                id: Uuid::parse_str(row.id.as_deref().unwrap_or(""))?,
                event_id: Uuid::parse_str(&row.event_id)?,
                invited_user_id: row
                    .invited_user_id
                    .as_ref()
                    .map(|id| Uuid::parse_str(id))
                    .transpose()?,
                invited_email: row.invited_email,
                invited_name: row.invited_name,
                inviter_id: Uuid::parse_str(&row.inviter_id)?,
                status,
                invitation_message: row.invitation_message,
                invited_at: DateTime::from_naive_utc_and_offset(row.invited_at, Utc),
                responded_at: row
                    .responded_at
                    .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            }))
        } else {
            Ok(None)
        }
    }
}
