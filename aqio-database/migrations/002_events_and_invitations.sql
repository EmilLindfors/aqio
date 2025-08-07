-- Comprehensive Event Management and Invitation System

-- Event categories for better organization
CREATE TABLE event_categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    color_hex TEXT, -- For UI theming
    icon_name TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert default categories
INSERT INTO event_categories (id, name, description, color_hex, icon_name) VALUES
('conf', 'Conference', 'Large industry conferences and seminars', '#3B82F6', 'presentation'),
('workshop', 'Workshop', 'Hands-on training and educational sessions', '#10B981', 'tools'),
('networking', 'Networking', 'Social and professional networking events', '#F59E0B', 'users'),
('training', 'Training', 'Professional development and skill building', '#8B5CF6', 'academic-cap'),
('personal', 'Personal', 'Private celebrations and social gatherings', '#EC4899', 'heart'),
('meeting', 'Meeting', 'Business meetings and discussions', '#6B7280', 'clipboard');

-- Main events table
CREATE TABLE events (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    category_id TEXT NOT NULL REFERENCES event_categories(id),
    
    -- Timing
    start_date DATETIME NOT NULL,
    end_date DATETIME NOT NULL,
    timezone TEXT DEFAULT 'Europe/Oslo',
    
    -- Location (can be physical or virtual)
    location_type TEXT NOT NULL CHECK(location_type IN ('physical', 'virtual', 'hybrid')) DEFAULT 'physical',
    location_name TEXT, -- Venue name or platform name
    address TEXT, -- Physical address
    virtual_link TEXT, -- Meeting link for virtual events
    virtual_access_code TEXT, -- Meeting ID/password
    
    -- Organizer and permissions
    organizer_id TEXT NOT NULL REFERENCES users(id),
    co_organizers TEXT, -- JSON array of user IDs
    
    -- Event settings
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    requires_approval BOOLEAN NOT NULL DEFAULT FALSE, -- Organizer must approve registrations
    max_attendees INTEGER,
    allow_guests BOOLEAN NOT NULL DEFAULT FALSE, -- Can attendees bring +1s
    max_guests_per_person INTEGER DEFAULT 1,
    
    -- Registration settings
    registration_opens DATETIME,
    registration_closes DATETIME,
    registration_required BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Additional settings
    allow_waitlist BOOLEAN NOT NULL DEFAULT TRUE,
    send_reminders BOOLEAN NOT NULL DEFAULT TRUE,
    collect_dietary_info BOOLEAN NOT NULL DEFAULT FALSE,
    collect_accessibility_info BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- Event image and branding
    image_url TEXT,
    custom_fields TEXT, -- JSON for additional custom form fields
    
    -- Status
    status TEXT NOT NULL CHECK(status IN ('draft', 'published', 'cancelled', 'completed')) DEFAULT 'draft',
    
    -- Metadata
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Event invitations - handles both registered users and external contacts
CREATE TABLE event_invitations (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    
    -- Who is invited (either registered user or external contact)
    invited_user_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    invited_contact_id TEXT REFERENCES external_contacts(id) ON DELETE CASCADE,
    
    -- Manual invitation data (for one-off invites)
    invited_email TEXT,
    invited_name TEXT,
    
    -- Invitation metadata
    inviter_id TEXT NOT NULL REFERENCES users(id),
    invitation_method TEXT NOT NULL CHECK(invitation_method IN ('email', 'sms', 'manual', 'bulk_import')) DEFAULT 'email',
    personal_message TEXT,
    
    -- Status tracking
    status TEXT NOT NULL CHECK(status IN ('pending', 'sent', 'delivered', 'opened', 'accepted', 'declined', 'cancelled')) DEFAULT 'pending',
    sent_at DATETIME,
    opened_at DATETIME,
    responded_at DATETIME,
    
    -- Invitation token for secure RSVP links
    invitation_token TEXT UNIQUE,
    expires_at DATETIME,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Constraints to ensure proper invitation target
    CONSTRAINT check_invitation_target CHECK (
        (invited_user_id IS NOT NULL AND invited_contact_id IS NULL AND invited_email IS NULL AND invited_name IS NULL) OR
        (invited_user_id IS NULL AND invited_contact_id IS NOT NULL AND invited_email IS NULL AND invited_name IS NULL) OR
        (invited_user_id IS NULL AND invited_contact_id IS NULL AND invited_email IS NOT NULL AND invited_name IS NOT NULL)
    ),
    
    -- Prevent duplicate invitations
    UNIQUE(event_id, invited_user_id),
    UNIQUE(event_id, invited_contact_id),
    UNIQUE(event_id, invited_email)
);

-- Event registrations/RSVPs - actual attendance tracking
CREATE TABLE event_registrations (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    invitation_id TEXT REFERENCES event_invitations(id) ON DELETE SET NULL,
    
    -- Registrant information
    user_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    external_contact_id TEXT REFERENCES external_contacts(id) ON DELETE CASCADE,
    
    -- Manual registration data
    registrant_email TEXT,
    registrant_name TEXT,
    registrant_phone TEXT,
    registrant_company TEXT,
    
    -- Registration details
    status TEXT NOT NULL CHECK(status IN ('registered', 'waitlisted', 'cancelled', 'attended', 'no_show')) DEFAULT 'registered',
    registration_source TEXT NOT NULL CHECK(registration_source IN ('invitation', 'direct', 'waitlist_promotion')) DEFAULT 'invitation',
    
    -- Guest information
    guest_count INTEGER NOT NULL DEFAULT 0,
    guest_names TEXT, -- JSON array of guest names
    
    -- Special requirements
    dietary_restrictions TEXT,
    accessibility_needs TEXT,
    special_requests TEXT,
    
    -- Custom field responses
    custom_responses TEXT, -- JSON for responses to custom fields
    
    -- Status tracking
    registered_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    cancelled_at DATETIME,
    checked_in_at DATETIME,
    
    -- Waitlist management
    waitlist_position INTEGER,
    waitlist_added_at DATETIME,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Ensure proper registrant identification
    CONSTRAINT check_registrant_identity CHECK (
        (user_id IS NOT NULL AND external_contact_id IS NULL AND registrant_email IS NULL AND registrant_name IS NULL) OR
        (user_id IS NULL AND external_contact_id IS NOT NULL AND registrant_email IS NULL AND registrant_name IS NULL) OR
        (user_id IS NULL AND external_contact_id IS NULL AND registrant_email IS NOT NULL AND registrant_name IS NOT NULL)
    ),
    
    -- Prevent duplicate registrations
    UNIQUE(event_id, user_id),
    UNIQUE(event_id, external_contact_id),
    UNIQUE(event_id, registrant_email)
);

-- Indexes for performance
CREATE INDEX idx_events_organizer_id ON events(organizer_id);
CREATE INDEX idx_events_category_id ON events(category_id);
CREATE INDEX idx_events_start_date ON events(start_date);
CREATE INDEX idx_events_status ON events(status);
CREATE INDEX idx_events_is_private ON events(is_private);

CREATE INDEX idx_invitations_event_id ON event_invitations(event_id);
CREATE INDEX idx_invitations_invited_user_id ON event_invitations(invited_user_id);
CREATE INDEX idx_invitations_invited_contact_id ON event_invitations(invited_contact_id);
CREATE INDEX idx_invitations_invited_email ON event_invitations(invited_email);
CREATE INDEX idx_invitations_inviter_id ON event_invitations(inviter_id);
CREATE INDEX idx_invitations_status ON event_invitations(status);
CREATE INDEX idx_invitations_token ON event_invitations(invitation_token);

CREATE INDEX idx_registrations_event_id ON event_registrations(event_id);
CREATE INDEX idx_registrations_invitation_id ON event_registrations(invitation_id);
CREATE INDEX idx_registrations_user_id ON event_registrations(user_id);
CREATE INDEX idx_registrations_external_contact_id ON event_registrations(external_contact_id);
CREATE INDEX idx_registrations_status ON event_registrations(status);
CREATE INDEX idx_registrations_waitlist_position ON event_registrations(waitlist_position);