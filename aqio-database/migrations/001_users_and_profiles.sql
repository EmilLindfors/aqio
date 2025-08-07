-- Comprehensive User and Profile Management System

-- Companies/Organizations for Norwegian aquaculture industry
CREATE TABLE companies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    org_number TEXT UNIQUE,
    location TEXT,
    industry_type TEXT NOT NULL CHECK(industry_type IN ('Salmon', 'Trout', 'Other')),
    industry_type_other TEXT,
    website TEXT,
    phone TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Registered platform users
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    keycloak_id TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    company_id TEXT REFERENCES companies(id),
    role TEXT NOT NULL CHECK(role IN ('admin', 'organizer', 'participant')) DEFAULT 'participant',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Extended user profiles with additional information
CREATE TABLE user_profiles (
    user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    phone TEXT,
    title TEXT, -- Job title
    bio TEXT,
    profile_image_url TEXT,
    timezone TEXT DEFAULT 'Europe/Oslo',
    language TEXT DEFAULT 'no',
    dietary_restrictions TEXT,
    accessibility_needs TEXT,
    emergency_contact_name TEXT,
    emergency_contact_phone TEXT,
    linkedin_url TEXT,
    twitter_handle TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- External contacts (not registered users) - for invitations
CREATE TABLE external_contacts (
    id TEXT PRIMARY KEY,
    email TEXT,
    first_name TEXT,
    last_name TEXT,
    full_name TEXT, -- For cases where we only have full name
    phone TEXT,
    company_name TEXT,
    notes TEXT, -- Additional info about the contact
    created_by TEXT NOT NULL REFERENCES users(id),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Ensure we have some way to identify the contact
    CONSTRAINT check_contact_identity CHECK (
        email IS NOT NULL OR 
        (first_name IS NOT NULL AND last_name IS NOT NULL) OR 
        full_name IS NOT NULL
    )
);

-- Contact lists for organizing groups of people
CREATE TABLE contact_lists (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    owner_id TEXT NOT NULL REFERENCES users(id),
    is_private BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Many-to-many relationship between contact lists and external contacts
CREATE TABLE contact_list_members (
    id TEXT PRIMARY KEY,
    list_id TEXT NOT NULL REFERENCES contact_lists(id) ON DELETE CASCADE,
    contact_id TEXT NOT NULL REFERENCES external_contacts(id) ON DELETE CASCADE,
    added_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(list_id, contact_id)
);

-- Indexes for performance
CREATE INDEX idx_users_keycloak_id ON users(keycloak_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_company_id ON users(company_id);
CREATE INDEX idx_companies_org_number ON companies(org_number);
CREATE INDEX idx_external_contacts_email ON external_contacts(email);
CREATE INDEX idx_external_contacts_created_by ON external_contacts(created_by);
CREATE INDEX idx_contact_lists_owner_id ON contact_lists(owner_id);
CREATE INDEX idx_contact_list_members_list_id ON contact_list_members(list_id);
CREATE INDEX idx_contact_list_members_contact_id ON contact_list_members(contact_id);