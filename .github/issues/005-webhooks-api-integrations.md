# Webhooks & Third-Party API Integrations

**Labels:** `enhancement`, `feature`, `integration`, `api`, `medium-priority`

## Overview
Add comprehensive webhook system and third-party API integrations to enable seamless connectivity with external tools, CRM systems, and automation platforms.

## User Story
As a power user, I want to integrate Aqio with my existing tools (CRM, calendar, Slack) so that event data flows automatically between systems without manual work.

## Acceptance Criteria

### Webhook System
- [ ] Configurable webhook endpoints for organizations
- [ ] Event-driven webhook triggers (registration, cancellation, etc.)
- [ ] Webhook payload customization and filtering
- [ ] Retry logic and failure handling
- [ ] Webhook security (signatures, authentication)

### API Key Management
- [ ] User and organization API key generation
- [ ] Scoped API access (read-only, specific endpoints)
- [ ] API rate limiting and usage tracking
- [ ] Key rotation and revocation
- [ ] Developer documentation and testing tools

### Popular Integrations
- [ ] Calendar systems (Google, Outlook, Apple)
- [ ] Video conferencing (Zoom, Teams, Google Meet)
- [ ] Communication (Slack, Discord, Microsoft Teams)
- [ ] CRM systems (HubSpot, Salesforce, Pipedrive)
- [ ] Marketing (Mailchimp, ConvertKit, ActiveCampaign)

## Database Schema Requirements

### New Tables
```sql
-- API keys for external access
CREATE TABLE api_keys (
    id TEXT PRIMARY KEY,
    key_hash TEXT UNIQUE NOT NULL, -- hashed version of the actual key
    key_prefix TEXT NOT NULL, -- first 8 chars for display (ak_12345678...)
    
    -- Ownership
    user_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    organization_id TEXT REFERENCES organizations(id) ON DELETE CASCADE,
    
    -- Key details
    name TEXT NOT NULL, -- user-friendly name
    description TEXT,
    
    -- Permissions and scoping
    scopes TEXT NOT NULL, -- JSON array: ["events:read", "registrations:write"]
    allowed_ips TEXT, -- JSON array of allowed IP ranges
    
    -- Usage tracking
    last_used_at DATETIME,
    usage_count INTEGER NOT NULL DEFAULT 0,
    
    -- Status and expiration
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    expires_at DATETIME, -- optional expiration
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Webhook endpoints
CREATE TABLE webhooks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    
    -- Webhook configuration
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    secret TEXT, -- for signature verification
    
    -- Event filtering
    events TEXT NOT NULL, -- JSON array: ["event.created", "registration.confirmed"]
    event_filters TEXT, -- JSON: {"event_type": ["conference"], "min_attendees": 10}
    
    -- Delivery settings
    content_type TEXT NOT NULL CHECK(content_type IN ('application/json', 'application/x-www-form-urlencoded')) DEFAULT 'application/json',
    timeout_seconds INTEGER NOT NULL DEFAULT 30,
    retry_count INTEGER NOT NULL DEFAULT 3,
    retry_delay_seconds INTEGER NOT NULL DEFAULT 60,
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    last_triggered_at DATETIME,
    total_deliveries INTEGER NOT NULL DEFAULT 0,
    failed_deliveries INTEGER NOT NULL DEFAULT 0,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Webhook delivery attempts
CREATE TABLE webhook_deliveries (
    id TEXT PRIMARY KEY,
    webhook_id TEXT NOT NULL REFERENCES webhooks(id) ON DELETE CASCADE,
    
    -- Delivery details
    event_type TEXT NOT NULL,
    event_id TEXT, -- ID of the triggering event/registration/etc
    payload TEXT NOT NULL, -- JSON payload sent
    
    -- Request/response
    request_headers TEXT, -- JSON
    response_status_code INTEGER,
    response_headers TEXT, -- JSON
    response_body TEXT,
    
    -- Timing
    attempted_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME,
    duration_ms INTEGER,
    
    -- Status
    status TEXT NOT NULL CHECK(status IN ('pending', 'success', 'failed', 'cancelled')) DEFAULT 'pending',
    error_message TEXT,
    
    -- Retry information
    is_retry BOOLEAN NOT NULL DEFAULT FALSE,
    retry_number INTEGER DEFAULT 0,
    next_retry_at DATETIME
);

-- Third-party integrations configuration
CREATE TABLE integrations (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id TEXT REFERENCES users(id) ON DELETE CASCADE, -- user-specific integrations
    
    -- Integration details
    integration_type TEXT NOT NULL CHECK(integration_type IN (
        'google_calendar', 'outlook_calendar', 'zoom', 'teams', 
        'slack', 'discord', 'hubspot', 'salesforce', 'mailchimp'
    )),
    name TEXT NOT NULL, -- user-friendly name
    
    -- Authentication credentials (encrypted)
    access_token TEXT,
    refresh_token TEXT,
    token_expires_at DATETIME,
    api_key TEXT,
    client_id TEXT,
    
    -- Integration settings
    settings TEXT, -- JSON configuration specific to integration
    sync_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Status
    status TEXT NOT NULL CHECK(status IN ('active', 'error', 'disabled')) DEFAULT 'active',
    last_sync_at DATETIME,
    last_error_message TEXT,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(organization_id, user_id, integration_type)
);

-- Integration sync jobs and logs
CREATE TABLE integration_sync_logs (
    id TEXT PRIMARY KEY,
    integration_id TEXT NOT NULL REFERENCES integrations(id) ON DELETE CASCADE,
    
    -- Sync details
    sync_type TEXT NOT NULL CHECK(sync_type IN ('full', 'incremental', 'manual')),
    direction TEXT NOT NULL CHECK(direction IN ('import', 'export', 'bidirectional')),
    
    -- Results
    records_processed INTEGER NOT NULL DEFAULT 0,
    records_created INTEGER NOT NULL DEFAULT 0,
    records_updated INTEGER NOT NULL DEFAULT 0,
    records_failed INTEGER NOT NULL DEFAULT 0,
    
    -- Timing
    started_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME,
    duration_seconds INTEGER,
    
    -- Status and errors
    status TEXT NOT NULL CHECK(status IN ('running', 'completed', 'failed', 'cancelled')) DEFAULT 'running',
    error_details TEXT, -- JSON array of error messages
    
    -- Sync metadata
    sync_cursor TEXT, -- for incremental syncs
    external_sync_id TEXT -- ID from external system
);
```

## Webhook Event Types
```json
{
  "event_types": [
    "event.created",
    "event.updated", 
    "event.published",
    "event.cancelled",
    "registration.created",
    "registration.confirmed",
    "registration.cancelled",
    "invitation.sent",
    "invitation.accepted",
    "invitation.declined",
    "user.joined_organization",
    "payment.completed",
    "payment.failed"
  ]
}
```

## Webhook Payload Example
```json
{
  "id": "wh_evt_123456",
  "type": "registration.confirmed",
  "created": 1234567890,
  "data": {
    "object": {
      "id": "reg_789",
      "event_id": "evt_456",
      "user_email": "user@example.com",
      "registered_at": "2024-09-15T10:00:00Z",
      "dietary_restrictions": "Vegetarian"
    }
  },
  "organization_id": "org_123"
}
```

## API Endpoints Needed
- `POST /api/webhooks` - Create webhook
- `GET /api/webhooks/{id}/test` - Test webhook delivery
- `GET /api/webhooks/{id}/deliveries` - View delivery history
- `POST /api/api-keys` - Generate new API key
- `POST /api/integrations/{type}/authorize` - OAuth flow for integrations
- `POST /api/integrations/{id}/sync` - Trigger manual sync

## Integration Examples

### Google Calendar Integration
```json
{
  "integration_type": "google_calendar",
  "settings": {
    "calendar_id": "primary",
    "sync_direction": "export", // events created in Aqio -> Google Calendar
    "include_private_events": false,
    "add_attendees": true,
    "send_invitations": false
  }
}
```

### Slack Integration
```json
{
  "integration_type": "slack",
  "settings": {
    "channel": "#events",
    "notifications": {
      "event_created": true,
      "registration_milestone": {"at": [10, 50, 100]},
      "event_reminder": {"hours_before": 24}
    },
    "message_template": "🎉 New registration for {{event.title}}! {{registration_count}}/{{max_attendees}} spots filled."
  }
}
```

### HubSpot CRM Integration
```json
{
  "integration_type": "hubspot",
  "settings": {
    "sync_contacts": true,
    "create_deals_for_paid_events": true,
    "sync_event_attendance": true,
    "contact_property_mapping": {
      "dietary_restrictions": "dietary_preferences",
      "registration_source": "event_source"
    }
  }
}
```

## Security Features
- Webhook signature verification using HMAC
- API key scoping and IP restrictions
- OAuth 2.0 for third-party integrations
- Rate limiting per API key
- Request/response logging for debugging

## API Rate Limiting
```json
{
  "rate_limits": {
    "free": {"requests_per_minute": 60, "requests_per_hour": 1000},
    "pro": {"requests_per_minute": 300, "requests_per_hour": 10000},
    "enterprise": {"requests_per_minute": 1000, "requests_per_hour": 50000}
  }
}
```

## Testing Requirements
- [ ] Webhook delivery reliability tests
- [ ] OAuth flow integration tests
- [ ] Rate limiting enforcement tests
- [ ] API key security tests
- [ ] Integration sync accuracy tests

## Documentation Needed
- Complete API reference documentation
- Webhook setup and testing guide
- Integration setup guides for each platform
- Rate limiting and best practices
- Security recommendations

## Developer Tools
- API testing console
- Webhook testing interface
- Integration logs dashboard
- API usage analytics
- Sample code and SDKs

## Dependencies
- `reqwest` for HTTP requests
- `jsonwebtoken` for JWT handling
- `hmac` and `sha2` for webhook signatures
- OAuth 2.0 client libraries
- Background job processing system

## Estimated Effort
**Large (4-5 weeks)** - Complex integration system with multiple external APIs.

## Priority
**Medium** - Valuable for power users and enterprise customers, but not essential for basic functionality.

## Popular Integration Priorities
1. **Google Calendar** - Most requested for event sync
2. **Slack** - Popular for team notifications  
3. **Zoom** - Essential for virtual/hybrid events
4. **Mailchimp** - Common email marketing platform
5. **HubSpot** - Popular CRM in target market

## Compliance Considerations
- OAuth security best practices
- Data encryption for stored tokens
- Integration permission auditing
- Third-party data handling policies
- GDPR compliance for data sync