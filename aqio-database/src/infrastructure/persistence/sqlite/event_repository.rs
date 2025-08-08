use crate::domain::errors::InfrastructureError;
use aqio_core::{Event, EventFilter, PaginationParams, PaginatedResult, LocationType, EventStatus, DomainResult, EventRepository};
use async_trait::async_trait;
use sqlx::{Pool, Sqlite, Row};
use tracing::{instrument, debug};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct SqliteEventRepository {
    pool: Pool<Sqlite>,
}

impl SqliteEventRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    // Helper method to convert database row to Event
    fn row_to_event(row: &sqlx::sqlite::SqliteRow) -> Event {
        Event {
            id: Uuid::parse_str(&row.try_get::<String, _>("id").unwrap_or_default()).unwrap_or_default(),
            title: row.try_get("title").unwrap_or_default(),
            description: row.try_get("description").unwrap_or_default(),
            category_id: row.try_get("category_id").unwrap_or_default(),
            start_date: DateTime::from_naive_utc_and_offset(row.try_get("start_date").unwrap_or_else(|_| chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc()), Utc),
            end_date: DateTime::from_naive_utc_and_offset(row.try_get("end_date").unwrap_or_else(|_| chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc()), Utc),
            timezone: row.try_get("timezone").unwrap_or_default(),
            location_type: Self::string_to_location_type(&row.try_get::<String, _>("location_type").unwrap_or_default()),
            location_name: row.try_get("location_name").ok(),
            address: row.try_get("address").ok(),
            virtual_link: row.try_get("virtual_link").ok(),
            virtual_access_code: row.try_get("virtual_access_code").ok(),
            organizer_id: Uuid::parse_str(&row.try_get::<String, _>("organizer_id").unwrap_or_default()).unwrap_or_default(),
            co_organizers: serde_json::from_str(&row.try_get::<String, _>("co_organizers").unwrap_or_default()).unwrap_or_default(),
            is_private: row.try_get("is_private").unwrap_or(false),
            requires_approval: row.try_get("requires_approval").unwrap_or(false),
            max_attendees: row.try_get("max_attendees").ok(),
            allow_guests: row.try_get("allow_guests").unwrap_or(false),
            max_guests_per_person: row.try_get("max_guests_per_person").ok(),
            registration_opens: row.try_get("registration_opens").ok().map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            registration_closes: row.try_get("registration_closes").ok().map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            registration_required: row.try_get("registration_required").unwrap_or(false),
            allow_waitlist: row.try_get("allow_waitlist").unwrap_or(false),
            send_reminders: row.try_get("send_reminders").unwrap_or(false),
            collect_dietary_info: row.try_get("collect_dietary_info").unwrap_or(false),
            collect_accessibility_info: row.try_get("collect_accessibility_info").unwrap_or(false),
            image_url: row.try_get("image_url").ok(),
            custom_fields: row.try_get("custom_fields").ok(),
            status: Self::string_to_event_status(&row.try_get::<String, _>("status").unwrap_or_default()),
            created_at: DateTime::from_naive_utc_and_offset(row.try_get("created_at").unwrap_or_else(|_| chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc()), Utc),
            updated_at: DateTime::from_naive_utc_and_offset(row.try_get("updated_at").unwrap_or_else(|_| chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc()), Utc),
        }
    }

    fn location_type_to_string(location_type: &LocationType) -> &'static str {
        match location_type {
            LocationType::Physical => "physical",
            LocationType::Virtual => "virtual",
            LocationType::Hybrid => "hybrid",
        }
    }

    fn string_to_location_type(s: &str) -> LocationType {
        match s {
            "physical" => LocationType::Physical,
            "virtual" => LocationType::Virtual,
            "hybrid" => LocationType::Hybrid,
            _ => LocationType::Physical, // Default
        }
    }

    fn event_status_to_string(status: &EventStatus) -> &'static str {
        match status {
            EventStatus::Draft => "draft",
            EventStatus::Published => "published",
            EventStatus::Cancelled => "cancelled",
            EventStatus::Completed => "completed",
        }
    }

    fn string_to_event_status(s: &str) -> EventStatus {
        match s {
            "draft" => EventStatus::Draft,
            "published" => EventStatus::Published,
            "cancelled" => EventStatus::Cancelled,
            "completed" => EventStatus::Completed,
            _ => EventStatus::Draft, // Default
        }
    }

    #[instrument(skip(self))]
    async fn count_events_with_filter(&self, filter: &EventFilter) -> Result<i64, InfrastructureError> {
        let mut query_builder = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM events WHERE 1=1");
        
        self.apply_filter(&mut query_builder, filter);
        
        let query = query_builder.build_query_scalar::<i64>();
        let count = query.fetch_one(&self.pool).await?;
        
        Ok(count)
    }

    fn apply_filter<'a>(&self, query_builder: &mut sqlx::QueryBuilder<'a, Sqlite>, filter: &'a EventFilter) {
        if let Some(ref title) = filter.title_contains {
            query_builder.push(" AND title LIKE ");
            query_builder.push_bind(format!("%{}%", title));
        }

        if let Some(ref category_id) = filter.category_id {
            query_builder.push(" AND category_id = ");
            query_builder.push_bind(category_id);
        }

        if let Some(organizer_id) = filter.organizer_id {
            query_builder.push(" AND organizer_id = ");
            query_builder.push_bind(organizer_id.to_string());
        }

        if let Some(is_private) = filter.is_private {
            query_builder.push(" AND is_private = ");
            query_builder.push_bind(is_private);
        }

        if let Some(ref status) = filter.status {
            query_builder.push(" AND status = ");
            query_builder.push_bind(Self::event_status_to_string(status));
        }

        if let Some(ref location_type) = filter.location_type {
            query_builder.push(" AND location_type = ");
            query_builder.push_bind(Self::location_type_to_string(location_type));
        }

        if let Some(start_from) = filter.start_date_from {
            query_builder.push(" AND start_date >= ");
            query_builder.push_bind(start_from.naive_utc());
        }

        if let Some(start_to) = filter.start_date_to {
            query_builder.push(" AND start_date <= ");
            query_builder.push_bind(start_to.naive_utc());
        }
    }
}

#[async_trait]
impl EventRepository for SqliteEventRepository {
    #[instrument(skip(self, event))]
    async fn create(&self, event: &Event) -> DomainResult<()> {
        debug!("Creating enhanced event with id: {}", event.id);
        
        let result = sqlx::query(
            "INSERT INTO events (id, title, description, category_id, start_date, end_date, timezone, location_type, location_name, address, virtual_link, virtual_access_code, organizer_id, co_organizers, is_private, requires_approval, max_attendees, allow_guests, max_guests_per_person, registration_opens, registration_closes, registration_required, allow_waitlist, send_reminders, collect_dietary_info, collect_accessibility_info, image_url, custom_fields, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(event.id.to_string())
        .bind(&event.title)
        .bind(&event.description)
        .bind(&event.category_id)
        .bind(event.start_date.naive_utc())
        .bind(event.end_date.naive_utc())
        .bind(&event.timezone)
        .bind(Self::location_type_to_string(&event.location_type))
        .bind(event.location_name.as_deref())
        .bind(event.address.as_deref())
        .bind(event.virtual_link.as_deref())
        .bind(event.virtual_access_code.as_deref())
        .bind(event.organizer_id.to_string())
        .bind(serde_json::to_string(&event.co_organizers).unwrap_or_default())
        .bind(event.is_private)
        .bind(event.requires_approval)
        .bind(event.max_attendees)
        .bind(event.allow_guests)
        .bind(event.max_guests_per_person)
        .bind(event.registration_opens.map(|dt| dt.naive_utc()))
        .bind(event.registration_closes.map(|dt| dt.naive_utc()))
        .bind(event.registration_required)
        .bind(event.allow_waitlist)
        .bind(event.send_reminders)
        .bind(event.collect_dietary_info)
        .bind(event.collect_accessibility_info)
        .bind(event.image_url.as_deref())
        .bind(event.custom_fields.as_deref())
        .bind(Self::event_status_to_string(&event.status))
        .bind(event.created_at.naive_utc())
        .bind(event.updated_at.naive_utc())
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => {
                debug!("Successfully created enhanced event with id: {}", event.id);
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

    #[instrument(skip(self))]
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<Event>> {
        debug!("Finding enhanced event by id: {}", id);

        let id_string = id.to_string();
        let result = sqlx::query("SELECT id, title, description, category_id, start_date, end_date, timezone, location_type, location_name, address, virtual_link, virtual_access_code, organizer_id, co_organizers, is_private, requires_approval, max_attendees, allow_guests, max_guests_per_person, registration_opens, registration_closes, registration_required, allow_waitlist, send_reminders, collect_dietary_info, collect_accessibility_info, image_url, custom_fields, status, created_at, updated_at FROM events WHERE id = ?")
            .bind(id_string)
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(Some(row)) => {
                let event = Self::row_to_event(&row);
                debug!("Found event: {}", event.title);
                Ok(Some(event))
            }
            Ok(None) => {
                debug!("Event not found with id: {}", id);
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

    // TODO: Implement remaining methods...
    #[instrument(skip(self, event))]
    async fn update(&self, event: &Event) -> DomainResult<()> {
        debug!("Updating event with id: {}", event.id);
        
        let result = sqlx::query(
            "UPDATE events SET title = ?, description = ?, category_id = ?, start_date = ?, end_date = ?, timezone = ?, location_type = ?, location_name = ?, address = ?, virtual_link = ?, virtual_access_code = ?, organizer_id = ?, co_organizers = ?, is_private = ?, requires_approval = ?, max_attendees = ?, allow_guests = ?, max_guests_per_person = ?, registration_opens = ?, registration_closes = ?, registration_required = ?, allow_waitlist = ?, send_reminders = ?, collect_dietary_info = ?, collect_accessibility_info = ?, image_url = ?, custom_fields = ?, status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&event.title)
        .bind(&event.description)
        .bind(&event.category_id)
        .bind(event.start_date.naive_utc())
        .bind(event.end_date.naive_utc())
        .bind(&event.timezone)
        .bind(Self::location_type_to_string(&event.location_type))
        .bind(event.location_name.as_deref())
        .bind(event.address.as_deref())
        .bind(event.virtual_link.as_deref())
        .bind(event.virtual_access_code.as_deref())
        .bind(event.organizer_id.to_string())
        .bind(serde_json::to_string(&event.co_organizers).unwrap_or_default())
        .bind(event.is_private)
        .bind(event.requires_approval)
        .bind(event.max_attendees)
        .bind(event.allow_guests)
        .bind(event.max_guests_per_person)
        .bind(event.registration_opens.map(|dt| dt.naive_utc()))
        .bind(event.registration_closes.map(|dt| dt.naive_utc()))
        .bind(event.registration_required)
        .bind(event.allow_waitlist)
        .bind(event.send_reminders)
        .bind(event.collect_dietary_info)
        .bind(event.collect_accessibility_info)
        .bind(event.image_url.as_deref())
        .bind(event.custom_fields.as_deref())
        .bind(Self::event_status_to_string(&event.status))
        .bind(event.updated_at.naive_utc())
        .bind(event.id.to_string())
        .execute(&self.pool)
        .await;

        match result {
            Ok(query_result) => {
                if query_result.rows_affected() == 0 {
                    Err(aqio_core::DomainError::not_found("Event", event.id))
                } else {
                    debug!("Successfully updated event with id: {}", event.id);
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
    async fn find_by_organizer(&self, organizer_id: Uuid, pagination: PaginationParams) -> DomainResult<PaginatedResult<Event>> {
        debug!("Finding events by organizer id: {}", organizer_id);
        
        // Count total events for this organizer
        let organizer_id_string = organizer_id.to_string();
        let count_result = sqlx::query("SELECT COUNT(*) as count FROM events WHERE organizer_id = ?")
            .bind(&organizer_id_string)
            .fetch_one(&self.pool)
            .await;
            
        let total_count = match count_result {
            Ok(row) => row.try_get::<i64, _>("count").unwrap_or(0),
            Err(e) => return Err(InfrastructureError::from(e).into()),
        };
        
        // Fetch the actual events with pagination
        let result = sqlx::query("SELECT id, title, description, category_id, start_date, end_date, timezone, location_type, location_name, address, virtual_link, virtual_access_code, organizer_id, co_organizers, is_private, requires_approval, max_attendees, allow_guests, max_guests_per_person, registration_opens, registration_closes, registration_required, allow_waitlist, send_reminders, collect_dietary_info, collect_accessibility_info, image_url, custom_fields, status, created_at, updated_at FROM events WHERE organizer_id = ? ORDER BY start_date DESC LIMIT ? OFFSET ?")
            .bind(organizer_id_string)
            .bind(pagination.limit)
            .bind(pagination.offset)
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(rows) => {
                let events: Vec<Event> = rows.iter().map(Self::row_to_event).collect();
                debug!("Found {} events for organizer: {}", events.len(), organizer_id);
                Ok(PaginatedResult::new(events, total_count, pagination))
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
    async fn find_by_category(&self, category_id: &str) -> DomainResult<Vec<Event>> {
        debug!("Finding events by category id: {}", category_id);
        
        let result = sqlx::query("SELECT id, title, description, category_id, start_date, end_date, timezone, location_type, location_name, address, virtual_link, virtual_access_code, organizer_id, co_organizers, is_private, requires_approval, max_attendees, allow_guests, max_guests_per_person, registration_opens, registration_closes, registration_required, allow_waitlist, send_reminders, collect_dietary_info, collect_accessibility_info, image_url, custom_fields, status, created_at, updated_at FROM events WHERE category_id = ? ORDER BY start_date DESC")
            .bind(category_id)
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(rows) => {
                let events: Vec<Event> = rows.iter().map(Self::row_to_event).collect();
                debug!("Found {} events for category: {}", events.len(), category_id);
                Ok(events)
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

    #[instrument(skip(self, filter))]
    async fn find_by_filter(
        &self,
        filter: &EventFilter,
        pagination: PaginationParams,
    ) -> DomainResult<PaginatedResult<Event>> {
        debug!("Listing events with filter and pagination");
        
        // Build the main query using the query builder
        let mut query_builder = sqlx::QueryBuilder::new("SELECT id, title, description, category_id, start_date, end_date, timezone, location_type, location_name, address, virtual_link, virtual_access_code, organizer_id, co_organizers, is_private, requires_approval, max_attendees, allow_guests, max_guests_per_person, registration_opens, registration_closes, registration_required, allow_waitlist, send_reminders, collect_dietary_info, collect_accessibility_info, image_url, custom_fields, status, created_at, updated_at FROM events WHERE 1=1");
        
        // Apply filters using the helper method
        self.apply_filter(&mut query_builder, filter);
        
        // Add ordering and pagination
        query_builder.push(" ORDER BY start_date DESC");
        query_builder.push(&format!(" LIMIT {} OFFSET {}", pagination.limit, pagination.offset));
        
        let query = query_builder.build();
        let result = query.fetch_all(&self.pool).await;
        
        match result {
            Ok(rows) => {
                let events: Vec<Event> = rows.iter().map(Self::row_to_event).collect();
                
                // Get total count using the helper method
                let total_count = match self.count_events_with_filter(filter).await {
                    Ok(count) => count,
                    Err(_) => 0,
                };
                
                debug!("Listed {} events (total: {})", events.len(), total_count);
                
                Ok(PaginatedResult::new(events, total_count, PaginationParams { 
                    offset: pagination.offset, 
                    limit: pagination.limit 
                }))
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
    async fn list_all(&self, pagination: PaginationParams) -> DomainResult<PaginatedResult<Event>> {
        debug!("Listing all events with pagination");
        
        // Count total events
        let count_result = sqlx::query("SELECT COUNT(*) as count FROM events")
            .fetch_one(&self.pool)
            .await;
            
        let total_count = match count_result {
            Ok(row) => row.try_get::<i64, _>("count").unwrap_or(0),
            Err(e) => return Err(InfrastructureError::from(e).into()),
        };
        
        // Fetch the events with pagination
        let result = sqlx::query("SELECT id, title, description, category_id, start_date, end_date, timezone, location_type, location_name, address, virtual_link, virtual_access_code, organizer_id, co_organizers, is_private, requires_approval, max_attendees, allow_guests, max_guests_per_person, registration_opens, registration_closes, registration_required, allow_waitlist, send_reminders, collect_dietary_info, collect_accessibility_info, image_url, custom_fields, status, created_at, updated_at FROM events ORDER BY start_date DESC LIMIT ? OFFSET ?")
            .bind(pagination.limit)
            .bind(pagination.offset)
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(rows) => {
                let events: Vec<Event> = rows.iter().map(Self::row_to_event).collect();
                debug!("Listed {} events out of {} total", events.len(), total_count);
                Ok(PaginatedResult::new(events, total_count, pagination))
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
        debug!("Deleting event with id: {}", id);
        
        let id_string = id.to_string();
        let result = sqlx::query("DELETE FROM events WHERE id = ?")
            .bind(id_string)
            .execute(&self.pool)
            .await;

        match result {
            Ok(query_result) => {
                if query_result.rows_affected() == 0 {
                    Err(aqio_core::DomainError::not_found("Event", id))
                } else {
                    debug!("Successfully deleted event with id: {}", id);
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
    async fn exists(&self, id: Uuid) -> DomainResult<bool> {
        debug!("Checking if event exists with id: {}", id);

        let result = sqlx::query("SELECT 1 FROM events WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(Some(_)) => {
                debug!("Event exists with id: {}", id);
                Ok(true)
            }
            Ok(None) => {
                debug!("Event does not exist with id: {}", id);
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
    use aqio_core::{EventFilter, PaginationParams, LocationType, EventStatus};
    use chrono::{Utc, Duration};
    use sqlx::{Pool, Sqlite};
    use uuid::Uuid;

    // Test helper to create an in-memory SQLite database with schema
    async fn create_test_db() -> Pool<Sqlite> {
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        
        // Create event categories table first
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

        // Insert default categories
        sqlx::query(r#"
            INSERT INTO event_categories (id, name, description, color_hex, icon_name) VALUES
            ('conf', 'Conference', 'Large industry conferences and seminars', '#3B82F6', 'presentation'),
            ('workshop', 'Workshop', 'Hands-on training and educational sessions', '#10B981', 'tools'),
            ('networking', 'Networking', 'Social and professional networking events', '#F59E0B', 'users'),
            ('training', 'Training', 'Professional development and skill building', '#8B5CF6', 'academic-cap'),
            ('personal', 'Personal', 'Private celebrations and social gatherings', '#EC4899', 'heart'),
            ('meeting', 'Meeting', 'Business meetings and discussions', '#6B7280', 'clipboard')
        "#)
        .execute(&pool)
        .await
        .unwrap();
        
        // Create the events table with the new sophisticated schema
        sqlx::query(r#"
            CREATE TABLE events (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                category_id TEXT NOT NULL REFERENCES event_categories(id),
                
                start_date DATETIME NOT NULL,
                end_date DATETIME NOT NULL,
                timezone TEXT DEFAULT 'UTC',
                
                location_type TEXT NOT NULL CHECK(location_type IN ('physical', 'virtual', 'hybrid')) DEFAULT 'physical',
                location_name TEXT,
                address TEXT,
                virtual_link TEXT,
                virtual_access_code TEXT,
                
                organizer_id TEXT NOT NULL,
                co_organizers TEXT DEFAULT '[]',
                
                is_private BOOLEAN NOT NULL DEFAULT FALSE,
                requires_approval BOOLEAN NOT NULL DEFAULT FALSE,
                max_attendees INTEGER,
                allow_guests BOOLEAN NOT NULL DEFAULT FALSE,
                max_guests_per_person INTEGER,
                
                registration_opens DATETIME,
                registration_closes DATETIME,
                registration_required BOOLEAN NOT NULL DEFAULT FALSE,
                
                allow_waitlist BOOLEAN NOT NULL DEFAULT FALSE,
                send_reminders BOOLEAN NOT NULL DEFAULT TRUE,
                collect_dietary_info BOOLEAN NOT NULL DEFAULT FALSE,
                collect_accessibility_info BOOLEAN NOT NULL DEFAULT FALSE,
                
                image_url TEXT,
                custom_fields TEXT,
                
                status TEXT NOT NULL CHECK(status IN ('draft', 'published', 'cancelled', 'completed')) DEFAULT 'draft',
                
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
        "#)
        .execute(&pool)
        .await
        .unwrap();
        
        pool
    }
    
    // Helper function to create a test event
    fn create_test_event(title: &str) -> Event {
        let now = Utc::now();
        Event {
            id: Uuid::new_v4(),
            title: title.to_string(),
            description: "Test event description".to_string(),
            category_id: "conf".to_string(),
            start_date: now + Duration::hours(24),
            end_date: now + Duration::hours(26),
            timezone: "UTC".to_string(),
            location_type: LocationType::Physical,
            location_name: Some("Test Location".to_string()),
            address: None,
            virtual_link: None,
            virtual_access_code: None,
            organizer_id: Uuid::new_v4(),
            co_organizers: Vec::new(),
            is_private: false,
            requires_approval: false,
            max_attendees: Some(100),
            allow_guests: false,
            max_guests_per_person: None,
            registration_opens: None,
            registration_closes: None,
            registration_required: false,
            allow_waitlist: false,
            send_reminders: true,
            collect_dietary_info: false,
            collect_accessibility_info: false,
            image_url: None,
            custom_fields: None,
            status: EventStatus::Draft,
            created_at: now,
            updated_at: now,
        }
    }

    #[tokio::test]
    async fn test_create_and_find_event() {
        let pool = create_test_db().await;
        let repository = SqliteEventRepository::new(pool);
        let event = create_test_event("Test Conference");
        let event_id = event.id;

        // Create the event
        let result = repository.create(&event).await;
        assert!(result.is_ok(), "Failed to create event: {:?}", result);

        // Find the event
        let found_event = repository.find_by_id(event_id).await.unwrap();
        assert!(found_event.is_some(), "Event not found after creation");
        
        let found_event = found_event.unwrap();
        assert_eq!(found_event.title, "Test Conference");
        assert_eq!(found_event.category_id, "conf");
        assert_eq!(found_event.location_name, Some("Test Location".to_string()));
    }

    #[tokio::test]
    async fn test_update_event() {
        let pool = create_test_db().await;
        let repository = SqliteEventRepository::new(pool);
        let mut event = create_test_event("Original Title");
        let event_id = event.id;

        // Create the event
        repository.create(&event).await.unwrap();

        // Update the event
        event.title = "Updated Title".to_string();
        event.description = "Updated description".to_string();
        event.updated_at = Utc::now();

        let result = repository.update(&event).await;
        assert!(result.is_ok(), "Failed to update event: {:?}", result);

        // Verify the update
        let found_event = repository.find_by_id(event_id).await.unwrap().unwrap();
        assert_eq!(found_event.title, "Updated Title");
        assert_eq!(found_event.description, "Updated description");
    }

    #[tokio::test]
    async fn test_delete_event() {
        let pool = create_test_db().await;
        let repository = SqliteEventRepository::new(pool);
        let event = create_test_event("Event to Delete");
        let event_id = event.id;

        // Create the event
        repository.create(&event).await.unwrap();

        // Verify it exists
        assert!(repository.exists(event_id).await.unwrap());

        // Delete the event
        let result = repository.delete(event_id).await;
        assert!(result.is_ok(), "Failed to delete event: {:?}", result);

        // Verify it no longer exists
        assert!(!repository.exists(event_id).await.unwrap());
        
        let found_event = repository.find_by_id(event_id).await.unwrap();
        assert!(found_event.is_none());
    }

    #[tokio::test]
    async fn test_delete_nonexistent_event() {
        let pool = create_test_db().await;
        let repository = SqliteEventRepository::new(pool);
        let nonexistent_id = Uuid::new_v4();

        // Try to delete a non-existent event
        let result = repository.delete(nonexistent_id).await;
        assert!(result.is_err(), "Should fail when deleting non-existent event");
    }

    #[tokio::test]
    async fn test_update_nonexistent_event() {
        let pool = create_test_db().await;
        let repository = SqliteEventRepository::new(pool);
        let event = create_test_event("Nonexistent Event");

        // Try to update a non-existent event
        let result = repository.update(&event).await;
        assert!(result.is_err(), "Should fail when updating non-existent event");
    }

    #[tokio::test]
    async fn test_exists() {
        let pool = create_test_db().await;
        let repository = SqliteEventRepository::new(pool);
        let event = create_test_event("Existence Test");
        let event_id = event.id;
        let nonexistent_id = Uuid::new_v4();

        // Check non-existent event
        assert!(!repository.exists(nonexistent_id).await.unwrap());

        // Create event
        repository.create(&event).await.unwrap();

        // Check existing event
        assert!(repository.exists(event_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_find_by_organizer() {
        let pool = create_test_db().await;
        let repository = SqliteEventRepository::new(pool);
        
        let organizer_id = Uuid::new_v4();
        let other_organizer_id = Uuid::new_v4();
        
        let mut event1 = create_test_event("Event 1");
        event1.organizer_id = organizer_id;
        
        let mut event2 = create_test_event("Event 2");
        event2.organizer_id = organizer_id;
        
        let mut event3 = create_test_event("Event 3");
        event3.organizer_id = other_organizer_id;

        // Create all events
        repository.create(&event1).await.unwrap();
        repository.create(&event2).await.unwrap();
        repository.create(&event3).await.unwrap();

        // Find events by organizer
        let pagination = PaginationParams { offset: 0, limit: 10 };
        let events = repository.find_by_organizer(organizer_id, pagination).await.unwrap();
        assert_eq!(events.items.len(), 2);
        
        let titles: Vec<&str> = events.items.iter().map(|e| e.title.as_str()).collect();
        assert!(titles.contains(&"Event 1"));
        assert!(titles.contains(&"Event 2"));
        
        // Find events by other organizer
        let other_pagination = PaginationParams { offset: 0, limit: 10 };
        let other_events = repository.find_by_organizer(other_organizer_id, other_pagination).await.unwrap();
        assert_eq!(other_events.items.len(), 1);
        assert_eq!(other_events.items[0].title, "Event 3");
    }

    #[tokio::test]
    async fn test_find_by_category() {
        let pool = create_test_db().await;
        let repository = SqliteEventRepository::new(pool);
        
        let mut event1 = create_test_event("Conference Event");
        event1.category_id = "conf".to_string();
        
        let mut event2 = create_test_event("Workshop Event");
        event2.category_id = "workshop".to_string();
        
        let mut event3 = create_test_event("Another Conference");
        event3.category_id = "conf".to_string();

        // Create all events
        repository.create(&event1).await.unwrap();
        repository.create(&event2).await.unwrap();
        repository.create(&event3).await.unwrap();

        // Find conference events
        let conference_events = repository.find_by_category("conf").await.unwrap();
        assert_eq!(conference_events.len(), 2);
        
        // Find workshop events
        let workshop_events = repository.find_by_category("workshop").await.unwrap();
        assert_eq!(workshop_events.len(), 1);
        assert_eq!(workshop_events[0].title, "Workshop Event");
        
        // Find non-existent category
        let nonexistent_events = repository.find_by_category("nonexistent").await.unwrap();
        assert_eq!(nonexistent_events.len(), 0);
    }

    #[tokio::test]
    async fn test_list_with_pagination() {
        let pool = create_test_db().await;
        let repository = SqliteEventRepository::new(pool);
        
        // Create multiple events
        for i in 1..=5 {
            let event = create_test_event(&format!("Event {}", i));
            repository.create(&event).await.unwrap();
        }

        // Test pagination
        let filter = EventFilter {
            title_contains: None,
            category_id: None,
            organizer_id: None,
            is_private: None,
            status: None,
            location_type: None,
            start_date_from: None,
            start_date_to: None,
        };
        
        let pagination = PaginationParams { offset: 0, limit: 3 };
        let result = repository.find_by_filter(&filter, pagination).await.unwrap();
        
        assert_eq!(result.items.len(), 3);
        assert_eq!(result.total_count, 5);
        assert_eq!(result.offset, 0);
        assert_eq!(result.limit, 3);
        assert!(result.has_next);

        // Test second page
        let pagination2 = PaginationParams { offset: 3, limit: 3 };
        let result2 = repository.find_by_filter(&filter, pagination2).await.unwrap();
        
        assert_eq!(result2.items.len(), 2);
        assert_eq!(result2.total_count, 5);
        assert_eq!(result2.offset, 3);
        assert_eq!(result2.limit, 3);
        assert!(!result2.has_next);
    }

    #[tokio::test]
    async fn test_list_with_title_filter() {
        let pool = create_test_db().await;
        let repository = SqliteEventRepository::new(pool);
        
        // Create events with different titles
        let event1 = create_test_event("Rust Conference 2024");
        let event2 = create_test_event("Python Workshop");
        let event3 = create_test_event("Advanced Rust Training");
        
        repository.create(&event1).await.unwrap();
        repository.create(&event2).await.unwrap();
        repository.create(&event3).await.unwrap();

        // Filter by title containing "Rust"
        let filter = EventFilter {
            title_contains: Some("Rust".to_string()),
            category_id: None,
            organizer_id: None,
            is_private: None,
            status: None,
            location_type: None,
            start_date_from: None,
            start_date_to: None,
        };
        
        let pagination = PaginationParams { offset: 0, limit: 10 };
        let result = repository.find_by_filter(&filter, pagination).await.unwrap();
        
        assert_eq!(result.items.len(), 2);
        assert_eq!(result.total_count, 2);
        
        let titles: Vec<&str> = result.items.iter().map(|e| e.title.as_str()).collect();
        assert!(titles.contains(&"Rust Conference 2024"));
        assert!(titles.contains(&"Advanced Rust Training"));
    }

    #[tokio::test]
    async fn test_list_with_category_filter() {
        let pool = create_test_db().await;
        let repository = SqliteEventRepository::new(pool);
        
        let mut event1 = create_test_event("Event 1");
        event1.category_id = "conf".to_string();
        
        let mut event2 = create_test_event("Event 2");
        event2.category_id = "workshop".to_string();
        
        let mut event3 = create_test_event("Event 3");
        event3.category_id = "conf".to_string();
        
        repository.create(&event1).await.unwrap();
        repository.create(&event2).await.unwrap();
        repository.create(&event3).await.unwrap();

        // Filter by conference category
        let filter = EventFilter {
            title_contains: None,
            category_id: Some("conf".to_string()),
            organizer_id: None,
            is_private: None,
            status: None,
            location_type: None,
            start_date_from: None,
            start_date_to: None,
        };
        
        let pagination = PaginationParams { offset: 0, limit: 10 };
        let result = repository.find_by_filter(&filter, pagination).await.unwrap();
        
        assert_eq!(result.items.len(), 2);
        assert_eq!(result.total_count, 2);
        
        for event in result.items {
            assert_eq!(event.category_id, "conf");
        }
    }

    #[tokio::test]
    async fn test_list_with_combined_filters() {
        let pool = create_test_db().await;
        let repository = SqliteEventRepository::new(pool);
        
        let mut event1 = create_test_event("Rust Conference");
        event1.category_id = "conf".to_string();
        
        let mut event2 = create_test_event("Rust Workshop");
        event2.category_id = "workshop".to_string();
        
        let mut event3 = create_test_event("Python Conference");
        event3.category_id = "conf".to_string();
        
        repository.create(&event1).await.unwrap();
        repository.create(&event2).await.unwrap();
        repository.create(&event3).await.unwrap();

        // Filter by both title and category
        let filter = EventFilter {
            title_contains: Some("Rust".to_string()),
            category_id: Some("conf".to_string()),
            organizer_id: None,
            is_private: None,
            status: None,
            location_type: None,
            start_date_from: None,
            start_date_to: None,
        };
        
        let pagination = PaginationParams { offset: 0, limit: 10 };
        let result = repository.find_by_filter(&filter, pagination).await.unwrap();
        
        assert_eq!(result.items.len(), 1);
        assert_eq!(result.total_count, 1);
        assert_eq!(result.items[0].title, "Rust Conference");
        assert_eq!(result.items[0].category_id, "conf");
    }

    #[tokio::test]
    async fn test_empty_list() {
        let pool = create_test_db().await;
        let repository = SqliteEventRepository::new(pool);
        
        let filter = EventFilter {
            title_contains: None,
            category_id: None,
            organizer_id: None,
            is_private: None,
            status: None,
            location_type: None,
            start_date_from: None,
            start_date_to: None,
        };
        
        let pagination = PaginationParams { offset: 0, limit: 10 };
        let result = repository.find_by_filter(&filter, pagination).await.unwrap();
        
        assert_eq!(result.items.len(), 0);
        assert_eq!(result.total_count, 0);
        assert!(!result.has_next);
    }

    #[tokio::test]
    async fn test_event_ordering() {
        let pool = create_test_db().await;
        let repository = SqliteEventRepository::new(pool);
        
        let now = Utc::now();
        
        // Create events with different start dates
        let mut event1 = create_test_event("Earliest Event");
        event1.start_date = now + Duration::hours(1);
        
        let mut event2 = create_test_event("Latest Event");
        event2.start_date = now + Duration::hours(3);
        
        let mut event3 = create_test_event("Middle Event");
        event3.start_date = now + Duration::hours(2);
        
        repository.create(&event1).await.unwrap();
        repository.create(&event2).await.unwrap();
        repository.create(&event3).await.unwrap();

        // List events (should be ordered by start_date DESC)
        let filter = EventFilter {
            title_contains: None,
            category_id: None,
            organizer_id: None,
            is_private: None,
            status: None,
            location_type: None,
            start_date_from: None,
            start_date_to: None,
        };
        
        let pagination = PaginationParams { offset: 0, limit: 10 };
        let result = repository.find_by_filter(&filter, pagination).await.unwrap();
        
        assert_eq!(result.items.len(), 3);
        
        // Check ordering (latest first)
        assert_eq!(result.items[0].title, "Latest Event");
        assert_eq!(result.items[1].title, "Middle Event");
        assert_eq!(result.items[2].title, "Earliest Event");
    }
}