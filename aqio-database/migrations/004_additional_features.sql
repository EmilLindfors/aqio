-- Additional Features: Check-ins, Feedback, Files, Analytics

-- Event check-in management
CREATE TABLE event_check_ins (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    registration_id TEXT NOT NULL REFERENCES event_registrations(id) ON DELETE CASCADE,
    
    -- Check-in details
    checked_in_by TEXT REFERENCES users(id), -- Who performed the check-in (organizer/volunteer)
    check_in_method TEXT NOT NULL CHECK(check_in_method IN (
        'manual', 'qr_code', 'nfc', 'self_service', 'mobile_app'
    )) DEFAULT 'manual',
    
    -- Location tracking (useful for large venues)
    check_in_location TEXT, -- e.g., "Main Entrance", "Registration Desk A"
    
    -- Guest check-ins (for +1s)
    guests_checked_in INTEGER NOT NULL DEFAULT 0,
    guest_names TEXT, -- JSON array of actual guest names who showed up
    
    -- Timing
    checked_in_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    checked_out_at DATETIME, -- Optional for events that track exits
    
    -- Additional data
    notes TEXT,
    device_info TEXT, -- JSON with check-in device details
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(registration_id) -- One check-in per registration
);

-- Post-event surveys and feedback
CREATE TABLE event_surveys (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    
    -- Survey configuration
    questions TEXT NOT NULL, -- JSON array of survey questions and their types
    is_anonymous BOOLEAN NOT NULL DEFAULT FALSE,
    is_public BOOLEAN NOT NULL DEFAULT FALSE, -- Are results publicly visible
    
    -- Timing
    opens_at DATETIME,
    closes_at DATETIME,
    
    -- Settings
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    send_reminder BOOLEAN NOT NULL DEFAULT TRUE,
    
    created_by TEXT NOT NULL REFERENCES users(id),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Survey responses
CREATE TABLE survey_responses (
    id TEXT PRIMARY KEY,
    survey_id TEXT NOT NULL REFERENCES event_surveys(id) ON DELETE CASCADE,
    registration_id TEXT REFERENCES event_registrations(id) ON DELETE CASCADE,
    
    -- Respondent info (for anonymous surveys, only registration_id might be null)
    respondent_name TEXT,
    respondent_email TEXT,
    
    -- Response data
    responses TEXT NOT NULL, -- JSON with question IDs as keys and responses as values
    
    -- Ratings and scores
    overall_rating INTEGER, -- 1-5 star rating
    would_recommend BOOLEAN,
    likelihood_to_attend_again INTEGER, -- 1-10 scale
    
    -- Additional feedback
    comments TEXT,
    suggestions TEXT,
    
    submitted_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(survey_id, registration_id) -- One response per registrant per survey
);

-- File and photo sharing for events
CREATE TABLE event_files (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    uploader_id TEXT REFERENCES users(id) ON DELETE SET NULL,
    uploader_name TEXT, -- For external uploaders or deleted users
    
    -- File details
    filename TEXT NOT NULL,
    original_filename TEXT NOT NULL,
    file_path TEXT NOT NULL, -- Storage path
    file_size INTEGER NOT NULL, -- Size in bytes
    mime_type TEXT NOT NULL,
    file_hash TEXT, -- For duplicate detection
    
    -- File metadata
    title TEXT,
    description TEXT,
    category TEXT CHECK(category IN (
        'presentation', 'document', 'photo', 'video', 'audio', 'other'
    )) DEFAULT 'other',
    tags TEXT, -- JSON array of tags
    
    -- Permissions
    is_public BOOLEAN NOT NULL DEFAULT FALSE, -- Public to all attendees
    allowed_groups TEXT, -- JSON array: ['registered', 'organizers'] or specific user IDs
    
    -- Download tracking
    download_count INTEGER NOT NULL DEFAULT 0,
    last_downloaded_at DATETIME,
    
    uploaded_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Ensure uploader identification
    CONSTRAINT check_uploader_identity CHECK (
        uploader_id IS NOT NULL OR uploader_name IS NOT NULL
    )
);

-- Photo albums for events
CREATE TABLE event_albums (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    cover_photo_id TEXT REFERENCES event_files(id) ON DELETE SET NULL,
    
    -- Settings
    is_collaborative BOOLEAN NOT NULL DEFAULT TRUE, -- Can attendees add photos
    requires_approval BOOLEAN NOT NULL DEFAULT FALSE, -- Organizer must approve photos
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    
    created_by TEXT NOT NULL REFERENCES users(id),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Link photos to albums
CREATE TABLE album_photos (
    album_id TEXT NOT NULL REFERENCES event_albums(id) ON DELETE CASCADE,
    file_id TEXT NOT NULL REFERENCES event_files(id) ON DELETE CASCADE,
    sort_order INTEGER NOT NULL DEFAULT 0,
    added_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    PRIMARY KEY (album_id, file_id)
);

-- Event analytics and metrics
CREATE TABLE event_analytics (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    
    -- Registration metrics
    total_invitations INTEGER NOT NULL DEFAULT 0,
    total_registrations INTEGER NOT NULL DEFAULT 0,
    total_attended INTEGER NOT NULL DEFAULT 0,
    total_no_shows INTEGER NOT NULL DEFAULT 0,
    total_waitlisted INTEGER NOT NULL DEFAULT 0,
    
    -- Response rates by channel
    invitation_open_rate DECIMAL(5,2), -- Percentage
    invitation_response_rate DECIMAL(5,2),
    registration_conversion_rate DECIMAL(5,2),
    attendance_rate DECIMAL(5,2),
    
    -- Demographics (if available)
    attendee_demographics TEXT, -- JSON with company, role, location breakdowns
    
    -- Engagement metrics
    total_comments INTEGER NOT NULL DEFAULT 0,
    total_file_downloads INTEGER NOT NULL DEFAULT 0,
    survey_response_rate DECIMAL(5,2),
    average_survey_rating DECIMAL(3,1),
    
    -- Calculated at
    calculated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(event_id) -- One analytics record per event
);

-- Calendar integration settings
CREATE TABLE calendar_integrations (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Integration type
    provider TEXT NOT NULL CHECK(provider IN ('google', 'outlook', 'apple', 'ical')),
    calendar_id TEXT, -- External calendar ID
    
    -- Credentials (encrypted)
    access_token TEXT,
    refresh_token TEXT,
    token_expires_at DATETIME,
    
    -- Settings
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    sync_my_events BOOLEAN NOT NULL DEFAULT TRUE, -- Sync events I organize
    sync_attending_events BOOLEAN NOT NULL DEFAULT TRUE, -- Sync events I'm attending
    auto_add_attendees BOOLEAN NOT NULL DEFAULT FALSE, -- Add attendees to calendar events
    
    last_sync_at DATETIME,
    sync_status TEXT CHECK(sync_status IN ('success', 'error', 'pending')) DEFAULT 'pending',
    sync_error_message TEXT,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Audit log for tracking important changes
CREATE TABLE audit_logs (
    id TEXT PRIMARY KEY,
    
    -- What changed
    table_name TEXT NOT NULL,
    record_id TEXT NOT NULL,
    action TEXT NOT NULL CHECK(action IN ('insert', 'update', 'delete')),
    
    -- Who made the change
    user_id TEXT REFERENCES users(id) ON DELETE SET NULL,
    user_name TEXT, -- For deleted users
    user_email TEXT,
    
    -- Change details
    old_values TEXT, -- JSON of old field values
    new_values TEXT, -- JSON of new field values
    changed_fields TEXT, -- JSON array of field names that changed
    
    -- Context
    event_id TEXT REFERENCES events(id) ON DELETE SET NULL,
    ip_address TEXT,
    user_agent TEXT,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX idx_check_ins_event_id ON event_check_ins(event_id);
CREATE INDEX idx_check_ins_registration_id ON event_check_ins(registration_id);
CREATE INDEX idx_check_ins_checked_in_at ON event_check_ins(checked_in_at);

CREATE INDEX idx_surveys_event_id ON event_surveys(event_id);
CREATE INDEX idx_surveys_created_by ON event_surveys(created_by);
CREATE INDEX idx_survey_responses_survey_id ON survey_responses(survey_id);
CREATE INDEX idx_survey_responses_registration_id ON survey_responses(registration_id);

CREATE INDEX idx_event_files_event_id ON event_files(event_id);
CREATE INDEX idx_event_files_uploader_id ON event_files(uploader_id);
CREATE INDEX idx_event_files_category ON event_files(category);
CREATE INDEX idx_event_files_uploaded_at ON event_files(uploaded_at);

CREATE INDEX idx_event_albums_event_id ON event_albums(event_id);
CREATE INDEX idx_event_albums_created_by ON event_albums(created_by);

CREATE INDEX idx_calendar_integrations_user_id ON calendar_integrations(user_id);
CREATE INDEX idx_calendar_integrations_provider ON calendar_integrations(provider);
CREATE INDEX idx_calendar_integrations_last_sync_at ON calendar_integrations(last_sync_at);

CREATE INDEX idx_audit_logs_table_record ON audit_logs(table_name, record_id);
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_event_id ON audit_logs(event_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);