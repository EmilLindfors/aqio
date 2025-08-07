# Recurring & Series Events Support

**Labels:** `enhancement`, `feature`, `recurring`, `medium-priority`

## Overview
Add support for recurring events and event series to handle weekly/monthly meetings, conference sessions, and workshop series.

## User Story
As an event organizer, I want to create recurring events (weekly team meetings) and event series (multi-day conferences) so that I can efficiently manage repeating events without creating each instance manually.

## Acceptance Criteria

### Recurring Events
- [ ] Support weekly, monthly, yearly recurrence patterns
- [ ] Custom recurrence rules (every 2 weeks, first Monday of month, etc.)
- [ ] Bulk operations on recurring event instances
- [ ] Individual instance modifications without affecting series
- [ ] Exception handling (skip holiday dates, etc.)

### Event Series
- [ ] Multi-event series management (conferences with sessions)
- [ ] Series-level registration with session selection
- [ ] Session prerequisites and scheduling conflicts
- [ ] Series-wide settings inheritance
- [ ] Cross-session attendee analytics

### Registration Management
- [ ] Register for entire series vs individual instances
- [ ] Transfer registrations between instances
- [ ] Series pricing and bulk discounts
- [ ] Waitlist management across series

## Database Schema Requirements

### New Tables
```sql
-- Event series/recurring patterns
CREATE TABLE event_series (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    organizer_id TEXT NOT NULL REFERENCES users(id),
    series_type TEXT NOT NULL CHECK(series_type IN ('recurring', 'series')),
    
    -- Recurring pattern (for recurring events)
    recurrence_pattern TEXT, -- JSON: {"frequency": "weekly", "interval": 1, "days": ["monday"]}
    recurrence_start_date DATETIME,
    recurrence_end_date DATETIME,
    max_occurrences INTEGER,
    
    -- Series settings
    allow_partial_registration BOOLEAN NOT NULL DEFAULT TRUE,
    series_pricing_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Link individual events to series
CREATE TABLE event_series_instances (
    id TEXT PRIMARY KEY,
    series_id TEXT NOT NULL REFERENCES event_series(id) ON DELETE CASCADE,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    instance_number INTEGER NOT NULL, -- 1, 2, 3... or session order
    is_cancelled BOOLEAN NOT NULL DEFAULT FALSE,
    cancellation_reason TEXT,
    
    UNIQUE(series_id, instance_number)
);

-- Series-level registrations
CREATE TABLE series_registrations (
    id TEXT PRIMARY KEY,
    series_id TEXT NOT NULL REFERENCES event_series(id) ON DELETE CASCADE,
    user_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    external_contact_id TEXT REFERENCES external_contacts(id) ON DELETE CASCADE,
    
    -- Registration details
    registration_type TEXT NOT NULL CHECK(registration_type IN ('full_series', 'partial', 'individual')),
    selected_instances TEXT, -- JSON array of instance numbers for partial registration
    
    -- Status
    status TEXT NOT NULL CHECK(status IN ('registered', 'cancelled', 'completed')) DEFAULT 'registered',
    registered_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Ensure single registration per person per series
    CONSTRAINT unique_series_registration UNIQUE(series_id, user_id),
    CONSTRAINT unique_series_external_registration UNIQUE(series_id, external_contact_id)
);

-- Exception dates for recurring events
CREATE TABLE recurrence_exceptions (
    id TEXT PRIMARY KEY,
    series_id TEXT NOT NULL REFERENCES event_series(id) ON DELETE CASCADE,
    exception_date DATE NOT NULL,
    exception_type TEXT NOT NULL CHECK(exception_type IN ('skip', 'reschedule')),
    rescheduled_to_date DATE,
    reason TEXT,
    
    UNIQUE(series_id, exception_date)
);
```

### Modified Tables
```sql
-- Add series reference to events
ALTER TABLE events ADD COLUMN series_id TEXT REFERENCES event_series(id);
ALTER TABLE events ADD COLUMN is_series_instance BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE events ADD COLUMN series_instance_number INTEGER;
```

## API Endpoints Needed
- `POST /api/event-series` - Create new series/recurring pattern
- `GET /api/event-series/{id}/instances` - List all instances in series
- `POST /api/event-series/{id}/instances/{number}/modify` - Modify specific instance
- `DELETE /api/event-series/{id}/instances/{number}` - Cancel specific instance
- `POST /api/event-series/{id}/register` - Register for series
- `GET /api/event-series/{id}/analytics` - Series-wide analytics

## Features in Detail

### Recurring Event Patterns
```json
{
  "frequency": "weekly|monthly|yearly",
  "interval": 1, // every N weeks/months/years
  "days": ["monday", "wednesday"], // for weekly
  "week_of_month": 1, // first week, for monthly
  "day_of_month": 15, // 15th of month
  "months": ["january", "june"], // for yearly
  "end_condition": {
    "type": "date|count|never",
    "end_date": "2025-12-31",
    "max_occurrences": 52
  }
}
```

### Bulk Operations
- Update all future instances
- Update all instances in series
- Cancel remaining instances
- Reschedule series with bulk date shifts

### Series Analytics
- Cross-session attendance patterns
- Drop-off rates throughout series
- Most/least popular sessions
- Series completion rates

## UI/UX Considerations
- Series overview dashboard
- Calendar view showing all instances
- Bulk editing interface
- Exception handling UI
- Registration flow for series vs individual events

## Testing Requirements
- [ ] Recurrence pattern generation tests
- [ ] Exception handling edge cases
- [ ] Bulk operation consistency
- [ ] Cross-instance data integrity
- [ ] Performance tests with large series

## Documentation Needed
- Recurrence pattern examples
- Series setup best practices
- Bulk operations guide
- Migration guide for existing events

## Dependencies
- `chrono` for date calculations
- `serde_json` for pattern storage
- Calendar calculation utilities

## Estimated Effort
**Medium (2-3 weeks)** - Complex date/time logic and new database relationships.

## Priority
**Medium** - Valuable for organizations running regular meetings and multi-session events.