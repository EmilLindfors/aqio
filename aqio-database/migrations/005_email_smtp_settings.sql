-- SMTP and Email Configuration System

-- Organization email settings
CREATE TABLE organization_email_settings (
    id TEXT PRIMARY KEY,
    organization_name TEXT NOT NULL,
    
    -- SMTP Configuration
    smtp_host TEXT NOT NULL,
    smtp_port INTEGER NOT NULL DEFAULT 587,
    smtp_username TEXT NOT NULL,
    smtp_password TEXT NOT NULL, -- Should be encrypted in production
    smtp_encryption TEXT NOT NULL CHECK(smtp_encryption IN ('none', 'tls', 'ssl')) DEFAULT 'tls',
    
    -- Email defaults
    default_from_email TEXT NOT NULL,
    default_from_name TEXT NOT NULL,
    default_reply_to_email TEXT,
    
    -- Email branding
    email_signature TEXT,
    email_footer TEXT,
    logo_url TEXT,
    brand_color_hex TEXT DEFAULT '#3B82F6',
    
    -- Rate limiting and sending settings
    max_emails_per_hour INTEGER DEFAULT 100,
    max_emails_per_day INTEGER DEFAULT 1000,
    
    -- Status and validation
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    verification_token TEXT,
    last_verified_at DATETIME,
    
    -- Usage tracking
    emails_sent_today INTEGER NOT NULL DEFAULT 0,
    emails_sent_this_hour INTEGER NOT NULL DEFAULT 0,
    last_reset_date DATE DEFAULT CURRENT_DATE,
    last_reset_hour INTEGER DEFAULT 0,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- User/Company specific email settings (overrides for specific events)
CREATE TABLE user_email_settings (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    company_id TEXT REFERENCES companies(id) ON DELETE CASCADE,
    
    -- Custom SMTP (optional - uses organization default if not set)
    custom_smtp_host TEXT,
    custom_smtp_port INTEGER,
    custom_smtp_username TEXT,
    custom_smtp_password TEXT,
    custom_smtp_encryption TEXT CHECK(custom_smtp_encryption IN ('none', 'tls', 'ssl')),
    
    -- Email identity
    custom_from_email TEXT,
    custom_from_name TEXT,
    custom_reply_to_email TEXT,
    
    -- Personal branding
    custom_signature TEXT,
    custom_footer TEXT,
    
    -- Settings
    use_custom_smtp BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(user_id)
);

-- Email templates with better organization
CREATE TABLE email_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL CHECK(category IN (
        'invitation', 'reminder', 'confirmation', 'cancellation', 
        'update', 'welcome', 'follow_up', 'thank_you'
    )),
    
    -- Template content
    subject_template TEXT NOT NULL,
    html_body_template TEXT NOT NULL,
    text_body_template TEXT,
    
    -- Template variables and customization
    available_variables TEXT, -- JSON array of available placeholders
    is_system_template BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- Branding settings
    include_logo BOOLEAN NOT NULL DEFAULT TRUE,
    include_footer BOOLEAN NOT NULL DEFAULT TRUE,
    custom_css TEXT,
    
    -- Organization/user ownership
    organization_id TEXT,
    created_by_user_id TEXT REFERENCES users(id) ON DELETE SET NULL,
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    version INTEGER NOT NULL DEFAULT 1,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Email sending queue with better tracking
CREATE TABLE email_queue (
    id TEXT PRIMARY KEY,
    
    -- Email details
    to_email TEXT NOT NULL,
    to_name TEXT,
    from_email TEXT NOT NULL,
    from_name TEXT NOT NULL,
    reply_to_email TEXT,
    
    subject TEXT NOT NULL,
    html_body TEXT NOT NULL,
    text_body TEXT,
    
    -- Context
    event_id TEXT REFERENCES events(id) ON DELETE CASCADE,
    invitation_id TEXT REFERENCES event_invitations(id) ON DELETE CASCADE,
    template_id TEXT REFERENCES email_templates(id) ON DELETE SET NULL,
    
    -- SMTP settings to use
    smtp_host TEXT NOT NULL,
    smtp_port INTEGER NOT NULL,
    smtp_username TEXT NOT NULL,
    smtp_password TEXT NOT NULL,
    smtp_encryption TEXT NOT NULL,
    
    -- Priority and scheduling
    priority TEXT NOT NULL CHECK(priority IN ('low', 'normal', 'high', 'urgent')) DEFAULT 'normal',
    scheduled_for DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    -- Status tracking
    status TEXT NOT NULL CHECK(status IN (
        'queued', 'sending', 'sent', 'delivered', 'bounced', 'failed', 'cancelled'
    )) DEFAULT 'queued',
    
    -- Delivery tracking
    sent_at DATETIME,
    delivered_at DATETIME,
    opened_at DATETIME,
    clicked_at DATETIME,
    bounced_at DATETIME,
    failed_at DATETIME,
    
    -- Error handling
    failure_reason TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 3,
    next_retry_at DATETIME,
    
    -- Tracking
    tracking_pixel_url TEXT,
    unsubscribe_token TEXT,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Email opens and clicks tracking
CREATE TABLE email_tracking (
    id TEXT PRIMARY KEY,
    email_queue_id TEXT NOT NULL REFERENCES email_queue(id) ON DELETE CASCADE,
    
    -- Tracking type
    event_type TEXT NOT NULL CHECK(event_type IN ('open', 'click', 'unsubscribe')),
    
    -- Tracking details
    url_clicked TEXT, -- For click events
    user_agent TEXT,
    ip_address TEXT,
    
    tracked_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Additional context
    device_type TEXT,
    browser_name TEXT,
    location_country TEXT,
    location_city TEXT
);

-- Insert default organization settings for aqio.no
INSERT INTO organization_email_settings (
    id, organization_name, smtp_host, smtp_port, smtp_username, smtp_password, 
    default_from_email, default_from_name, default_reply_to_email,
    email_signature, email_footer, brand_color_hex
) VALUES (
    'aqio-default',
    'Aqio Event Management',
    'smtp.aqio.no',
    587,
    'noreply@aqio.no',
    'CHANGE_ME_IN_PRODUCTION',
    'noreply@aqio.no',
    'Aqio Events',
    'support@aqio.no',
    '<p>Best regards,<br>The Aqio Team</p>',
    '<p style="font-size: 12px; color: #666; text-align: center; margin-top: 20px;">
     Sent with ❤️ by <a href="https://aqio.no" style="color: #3B82F6;">Aqio Event Management</a><br>
     <a href="{{unsubscribe_url}}" style="color: #666;">Unsubscribe</a>
     </p>',
    '#3B82F6'
);

-- Insert default email templates
INSERT INTO email_templates (
    id, name, description, category, subject_template, html_body_template, text_body_template, 
    is_system_template, available_variables
) VALUES 
(
    'default-invitation',
    'Default Event Invitation',
    'Standard invitation template for all events',
    'invitation',
    'You''re invited: {{event_title}}',
    '<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{event_title}}</title>
</head>
<body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px;">
    {{#if include_logo}}
    <div style="text-align: center; margin-bottom: 30px;">
        <img src="{{logo_url}}" alt="Logo" style="max-height: 60px;">
    </div>
    {{/if}}
    
    <h1 style="color: {{brand_color}}; text-align: center;">{{event_title}}</h1>
    
    <p>Dear {{recipient_name}},</p>
    
    <p>You''re invited to <strong>{{event_title}}</strong>!</p>
    
    <div style="background: #f8f9fa; padding: 20px; border-radius: 8px; margin: 20px 0;">
        <h3 style="margin-top: 0; color: {{brand_color}};">📅 Event Details</h3>
        <p><strong>When:</strong> {{event_date}} at {{event_time}}</p>
        <p><strong>Where:</strong> {{event_location}}</p>
        {{#if event_description}}
        <p><strong>About:</strong> {{event_description}}</p>
        {{/if}}
    </div>
    
    {{#if personal_message}}
    <div style="background: #fff3cd; padding: 15px; border-left: 4px solid #ffc107; margin: 20px 0;">
        <p style="margin: 0;"><em>{{personal_message}}</em></p>
    </div>
    {{/if}}
    
    <div style="text-align: center; margin: 30px 0;">
        <a href="{{rsvp_url}}" style="background: {{brand_color}}; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; display: inline-block; font-weight: bold;">
            RSVP Now
        </a>
    </div>
    
    <p>We hope to see you there!</p>
    
    {{email_signature}}
    
    {{#if include_footer}}
    {{email_footer}}
    {{/if}}
</body>
</html>',
    'You''re invited to {{event_title}}!

Dear {{recipient_name}},

You''re invited to {{event_title}}!

Event Details:
When: {{event_date}} at {{event_time}}
Where: {{event_location}}

{{#if event_description}}
About: {{event_description}}
{{/if}}

{{#if personal_message}}
Personal message: {{personal_message}}
{{/if}}

Please RSVP: {{rsvp_url}}

We hope to see you there!

Best regards,
{{organizer_name}}',
    TRUE,
    '["recipient_name", "event_title", "event_date", "event_time", "event_location", "event_description", "personal_message", "rsvp_url", "organizer_name", "logo_url", "brand_color", "email_signature", "email_footer", "unsubscribe_url"]'
);

-- Indexes for performance
CREATE INDEX idx_org_email_settings_active ON organization_email_settings(is_active);
CREATE INDEX idx_user_email_settings_user_id ON user_email_settings(user_id);
CREATE INDEX idx_user_email_settings_company_id ON user_email_settings(company_id);

CREATE INDEX idx_email_templates_category ON email_templates(category);
CREATE INDEX idx_email_templates_active ON email_templates(is_active);
CREATE INDEX idx_email_templates_system ON email_templates(is_system_template);

CREATE INDEX idx_email_queue_status ON email_queue(status);
CREATE INDEX idx_email_queue_scheduled_for ON email_queue(scheduled_for);
CREATE INDEX idx_email_queue_priority ON email_queue(priority);
CREATE INDEX idx_email_queue_event_id ON email_queue(event_id);
CREATE INDEX idx_email_queue_invitation_id ON email_queue(invitation_id);

CREATE INDEX idx_email_tracking_queue_id ON email_tracking(email_queue_id);
CREATE INDEX idx_email_tracking_event_type ON email_tracking(event_type);
CREATE INDEX idx_email_tracking_tracked_at ON email_tracking(tracked_at);