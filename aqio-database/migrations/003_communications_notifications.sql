-- Communication and Notification System

-- Event updates and announcements
CREATE TABLE event_updates (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    author_id TEXT NOT NULL REFERENCES users(id),
    
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    update_type TEXT NOT NULL CHECK(update_type IN (
        'general', 'schedule_change', 'location_change', 'cancellation', 
        'reminder', 'important', 'social'
    )) DEFAULT 'general',
    
    -- Visibility settings
    is_public BOOLEAN NOT NULL DEFAULT TRUE, -- Visible to all attendees
    target_groups TEXT, -- JSON array: ['registered', 'waitlisted', 'invited'] or specific user/contact IDs
    
    -- Notification settings
    send_email BOOLEAN NOT NULL DEFAULT TRUE,
    send_sms BOOLEAN NOT NULL DEFAULT FALSE,
    send_push BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Scheduling
    is_scheduled BOOLEAN NOT NULL DEFAULT FALSE,
    scheduled_for DATETIME,
    sent_at DATETIME,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Comments and discussions on events
CREATE TABLE event_comments (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    parent_comment_id TEXT REFERENCES event_comments(id) ON DELETE CASCADE, -- For replies
    author_id TEXT REFERENCES users(id) ON DELETE SET NULL,
    author_name TEXT, -- For deleted users or external commenters
    
    content TEXT NOT NULL,
    is_private BOOLEAN NOT NULL DEFAULT FALSE, -- Only visible to organizers
    is_pinned BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- Moderation
    is_approved BOOLEAN NOT NULL DEFAULT TRUE,
    is_flagged BOOLEAN NOT NULL DEFAULT FALSE,
    flagged_reason TEXT,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Notification templates for different types of messages
CREATE TABLE notification_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    type TEXT NOT NULL CHECK(type IN (
        'invitation', 'registration_confirmation', 'reminder', 'cancellation',
        'update', 'waitlist_promotion', 'check_in_reminder', 'feedback_request'
    )),
    
    -- Template content for different channels
    email_subject TEXT,
    email_body TEXT,
    sms_body TEXT,
    push_title TEXT,
    push_body TEXT,
    
    -- Template variables (JSON array of available placeholders)
    available_variables TEXT,
    
    -- Settings
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_system_template BOOLEAN NOT NULL DEFAULT FALSE, -- Can't be deleted
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- User notification preferences
CREATE TABLE user_notification_preferences (
    user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    
    -- Global preferences
    email_notifications BOOLEAN NOT NULL DEFAULT TRUE,
    sms_notifications BOOLEAN NOT NULL DEFAULT FALSE,
    push_notifications BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Event-specific preferences
    event_invitations BOOLEAN NOT NULL DEFAULT TRUE,
    event_updates BOOLEAN NOT NULL DEFAULT TRUE,
    event_reminders BOOLEAN NOT NULL DEFAULT TRUE,
    event_cancellations BOOLEAN NOT NULL DEFAULT TRUE,
    waitlist_promotions BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Social features
    comments_on_my_events BOOLEAN NOT NULL DEFAULT TRUE,
    replies_to_my_comments BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Marketing and promotional
    promotional_emails BOOLEAN NOT NULL DEFAULT FALSE,
    event_recommendations BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Communication hours
    quiet_hours_start TIME, -- No notifications during these hours
    quiet_hours_end TIME,
    timezone TEXT DEFAULT 'Europe/Oslo',
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Notification queue and delivery tracking
CREATE TABLE notifications (
    id TEXT PRIMARY KEY,
    recipient_user_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    recipient_contact_id TEXT REFERENCES external_contacts(id) ON DELETE CASCADE,
    recipient_email TEXT, -- For manual recipients
    recipient_phone TEXT, -- For SMS notifications
    
    -- Notification content
    type TEXT NOT NULL CHECK(type IN (
        'invitation', 'registration_confirmation', 'reminder', 'cancellation',
        'update', 'waitlist_promotion', 'check_in_reminder', 'feedback_request', 'custom'
    )),
    channel TEXT NOT NULL CHECK(channel IN ('email', 'sms', 'push', 'in_app')),
    template_id TEXT REFERENCES notification_templates(id),
    
    -- Message content (resolved from template)
    subject TEXT,
    body TEXT NOT NULL,
    
    -- Context
    event_id TEXT REFERENCES events(id) ON DELETE CASCADE,
    related_id TEXT, -- Could reference invitation, registration, comment, etc.
    
    -- Delivery tracking
    status TEXT NOT NULL CHECK(status IN (
        'pending', 'sent', 'delivered', 'opened', 'clicked', 'failed', 'cancelled'
    )) DEFAULT 'pending',
    
    -- Scheduling
    scheduled_for DATETIME,
    sent_at DATETIME,
    delivered_at DATETIME,
    opened_at DATETIME,
    clicked_at DATETIME,
    failed_at DATETIME,
    failure_reason TEXT,
    
    -- Retry mechanism
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 3,
    next_retry_at DATETIME,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Ensure proper recipient identification
    CONSTRAINT check_notification_recipient CHECK (
        (recipient_user_id IS NOT NULL AND recipient_contact_id IS NULL AND recipient_email IS NULL) OR
        (recipient_user_id IS NULL AND recipient_contact_id IS NOT NULL AND recipient_email IS NULL) OR
        (recipient_user_id IS NULL AND recipient_contact_id IS NULL AND recipient_email IS NOT NULL)
    )
);

-- Insert default notification templates
INSERT INTO notification_templates (id, name, type, email_subject, email_body, sms_body, push_title, push_body, is_system_template) VALUES
('invite_email', 'Event Invitation', 'invitation', 
 'You''re invited: {{event_title}}', 
 'Hi {{recipient_name}},\n\nYou''re invited to {{event_title}} on {{event_date}} at {{event_location}}.\n\n{{personal_message}}\n\nPlease RSVP: {{rsvp_link}}',
 'You''re invited to {{event_title}} on {{event_date}}. RSVP: {{rsvp_link}}',
 'Event Invitation',
 'You''re invited to {{event_title}}',
 TRUE),
 
('registration_confirm', 'Registration Confirmed', 'registration_confirmation',
 'Registration confirmed: {{event_title}}',
 'Hi {{recipient_name}},\n\nYour registration for {{event_title}} is confirmed!\n\nEvent Details:\n- Date: {{event_date}}\n- Location: {{event_location}}\n- Time: {{event_time}}\n\nWe look forward to seeing you there!',
 'Registration confirmed for {{event_title}} on {{event_date}}',
 'Registration Confirmed',
 'You''re registered for {{event_title}}',
 TRUE),
 
('reminder_24h', '24 Hour Reminder', 'reminder',
 'Tomorrow: {{event_title}}',
 'Hi {{recipient_name}},\n\nThis is a reminder that {{event_title}} is tomorrow at {{event_time}}.\n\nLocation: {{event_location}}\n\nSee you there!',
 'Reminder: {{event_title}} is tomorrow at {{event_time}}',
 'Event Tomorrow',
 '{{event_title}} is tomorrow at {{event_time}}',
 TRUE);

-- Indexes for performance
CREATE INDEX idx_event_updates_event_id ON event_updates(event_id);
CREATE INDEX idx_event_updates_author_id ON event_updates(author_id);
CREATE INDEX idx_event_updates_scheduled_for ON event_updates(scheduled_for);
CREATE INDEX idx_event_updates_sent_at ON event_updates(sent_at);

CREATE INDEX idx_event_comments_event_id ON event_comments(event_id);
CREATE INDEX idx_event_comments_parent_comment_id ON event_comments(parent_comment_id);
CREATE INDEX idx_event_comments_author_id ON event_comments(author_id);

CREATE INDEX idx_notifications_recipient_user_id ON notifications(recipient_user_id);
CREATE INDEX idx_notifications_recipient_contact_id ON notifications(recipient_contact_id);
CREATE INDEX idx_notifications_recipient_email ON notifications(recipient_email);
CREATE INDEX idx_notifications_event_id ON notifications(event_id);
CREATE INDEX idx_notifications_status ON notifications(status);
CREATE INDEX idx_notifications_scheduled_for ON notifications(scheduled_for);
CREATE INDEX idx_notifications_sent_at ON notifications(sent_at);