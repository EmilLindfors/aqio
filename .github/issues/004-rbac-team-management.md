# Role-Based Access Control & Team Management

**Labels:** `enhancement`, `feature`, `security`, `rbac`, `high-priority`

## Overview
Implement comprehensive role-based access control (RBAC) and team management functionality to support complex organizational structures and delegation of event management tasks.

## User Story
As an organization admin, I want to assign different roles and permissions to team members so that I can delegate specific tasks while maintaining security and control over sensitive operations.

## Acceptance Criteria

### Role & Permission System
- [ ] Hierarchical role structure with inheritance
- [ ] Granular permissions for all system actions
- [ ] Custom role creation and modification
- [ ] Permission templates for common scenarios
- [ ] Audit logging of permission changes

### Team Management
- [ ] Organization and team hierarchies
- [ ] Team-based event access control
- [ ] Cross-team collaboration features
- [ ] Team invitation and onboarding
- [ ] Team performance analytics

### Multi-Organization Support
- [ ] Support for multiple organizations per user
- [ ] Organization-specific branding and settings
- [ ] Cross-organization event visibility controls
- [ ] Billing and usage per organization

## Database Schema Requirements

### New Tables
```sql
-- Organizations (companies, departments, groups)
CREATE TABLE organizations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL, -- for URLs like aqio.no/org/uib
    description TEXT,
    
    -- Organization settings
    logo_url TEXT,
    brand_colors TEXT, -- JSON: {"primary": "#3B82F6", "secondary": "#10B981"}
    custom_domain TEXT, -- e.g., events.uib.no
    timezone TEXT DEFAULT 'Europe/Oslo',
    
    -- Subscription and billing
    subscription_plan TEXT NOT NULL CHECK(subscription_plan IN ('free', 'pro', 'enterprise')) DEFAULT 'free',
    max_events_per_month INTEGER,
    max_team_members INTEGER,
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Teams within organizations
CREATE TABLE teams (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    
    -- Team settings
    team_type TEXT NOT NULL CHECK(team_type IN ('department', 'project', 'event_specific', 'committee')) DEFAULT 'department',
    is_default BOOLEAN NOT NULL DEFAULT FALSE, -- default team for new members
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(organization_id, name)
);

-- Roles define what actions can be performed
CREATE TABLE roles (
    id TEXT PRIMARY KEY,
    organization_id TEXT REFERENCES organizations(id) ON DELETE CASCADE, -- NULL for system roles
    name TEXT NOT NULL,
    description TEXT,
    
    -- Role hierarchy
    level INTEGER NOT NULL DEFAULT 0, -- 0=lowest, higher numbers = more permissions
    inherits_from_role_id TEXT REFERENCES roles(id),
    
    -- Role settings
    is_system_role BOOLEAN NOT NULL DEFAULT FALSE, -- system roles can't be deleted
    is_custom_role BOOLEAN NOT NULL DEFAULT TRUE,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(organization_id, name)
);

-- Permissions define granular access rights
CREATE TABLE permissions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE, -- e.g., 'event.create', 'user.invite', 'billing.view'
    description TEXT NOT NULL,
    category TEXT NOT NULL, -- 'event', 'user', 'organization', 'billing'
    
    -- Permission metadata
    is_sensitive BOOLEAN NOT NULL DEFAULT FALSE, -- requires extra confirmation
    requires_approval BOOLEAN NOT NULL DEFAULT FALSE, -- requires admin approval
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Role-permission assignments
CREATE TABLE role_permissions (
    role_id TEXT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id TEXT NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    granted_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    granted_by_user_id TEXT REFERENCES users(id),
    
    PRIMARY KEY (role_id, permission_id)
);

-- User memberships in organizations and teams
CREATE TABLE organization_memberships (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    organization_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    role_id TEXT NOT NULL REFERENCES roles(id),
    
    -- Membership details
    joined_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    invited_by_user_id TEXT REFERENCES users(id),
    status TEXT NOT NULL CHECK(status IN ('active', 'invited', 'suspended', 'left')) DEFAULT 'invited',
    
    -- Custom permissions (overrides role permissions)
    additional_permissions TEXT, -- JSON array of permission IDs
    revoked_permissions TEXT, -- JSON array of permission IDs
    
    UNIQUE(user_id, organization_id)
);

CREATE TABLE team_memberships (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    team_id TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    role_in_team TEXT NOT NULL CHECK(role_in_team IN ('member', 'lead', 'admin')) DEFAULT 'member',
    
    joined_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    added_by_user_id TEXT REFERENCES users(id),
    
    UNIQUE(user_id, team_id)
);

-- Event-specific permissions (for granular access control)
CREATE TABLE event_permissions (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    user_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    team_id TEXT REFERENCES teams(id) ON DELETE CASCADE,
    role_id TEXT REFERENCES roles(id) ON DELETE CASCADE,
    
    -- Permission details
    permission_type TEXT NOT NULL CHECK(permission_type IN ('view', 'edit', 'manage', 'admin')),
    granted_by_user_id TEXT NOT NULL REFERENCES users(id),
    granted_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME, -- optional expiration
    
    -- Ensure one target per permission
    CONSTRAINT check_permission_target CHECK (
        (user_id IS NOT NULL AND team_id IS NULL AND role_id IS NULL) OR
        (user_id IS NULL AND team_id IS NOT NULL AND role_id IS NULL) OR
        (user_id IS NULL AND team_id IS NULL AND role_id IS NOT NULL)
    )
);

-- Invitation system for organizations and teams
CREATE TABLE invitations (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    team_id TEXT REFERENCES teams(id) ON DELETE CASCADE,
    invited_email TEXT NOT NULL,
    invited_by_user_id TEXT NOT NULL REFERENCES users(id),
    
    -- Invitation details
    role_id TEXT NOT NULL REFERENCES roles(id),
    personal_message TEXT,
    
    -- Status and expiration
    status TEXT NOT NULL CHECK(status IN ('pending', 'accepted', 'rejected', 'expired')) DEFAULT 'pending',
    expires_at DATETIME NOT NULL DEFAULT (datetime('now', '+7 days')),
    
    -- Tokens for secure acceptance
    invitation_token TEXT UNIQUE NOT NULL,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    accepted_at DATETIME,
    
    UNIQUE(organization_id, invited_email) -- one pending invitation per email per org
);
```

### System Permissions to Insert
```sql
INSERT INTO permissions (id, name, description, category, is_sensitive) VALUES
-- Event permissions
('event.create', 'Create events', 'Create new events', 'event', false),
('event.edit', 'Edit events', 'Modify existing events', 'event', false),
('event.delete', 'Delete events', 'Delete events permanently', 'event', true),
('event.publish', 'Publish events', 'Make events public and send invitations', 'event', false),
('event.manage_registrations', 'Manage registrations', 'View and manage event registrations', 'event', false),

-- User and team permissions
('user.invite', 'Invite users', 'Send invitations to join organization', 'user', false),
('user.manage', 'Manage users', 'Edit user roles and permissions', 'user', true),
('team.create', 'Create teams', 'Create new teams within organization', 'team', false),
('team.manage', 'Manage teams', 'Modify team settings and memberships', 'team', false),

-- Organization permissions
('org.settings', 'Manage organization settings', 'Modify organization settings and branding', 'organization', true),
('org.billing', 'Access billing', 'View and manage billing information', 'organization', true),
('org.analytics', 'View analytics', 'Access organization-wide analytics', 'organization', false);

-- System roles
INSERT INTO roles (id, name, description, level, is_system_role, is_custom_role) VALUES
('system-super-admin', 'Super Admin', 'Full system access across all organizations', 100, true, false),
('system-org-owner', 'Organization Owner', 'Full access within organization', 90, true, false),
('system-org-admin', 'Organization Admin', 'Administrative access within organization', 80, true, false),
('system-team-lead', 'Team Lead', 'Manage team and team events', 60, true, false),
('system-event-manager', 'Event Manager', 'Create and manage events', 40, true, false),
('system-member', 'Member', 'Basic event participation', 10, true, false);
```

## API Endpoints Needed
- `GET /api/organizations/{id}/permissions/check` - Check user permissions
- `POST /api/organizations/{id}/teams` - Create team
- `POST /api/organizations/{id}/roles` - Create custom role
- `POST /api/organizations/{id}/invite` - Invite user to organization
- `PUT /api/users/{id}/permissions` - Update user permissions
- `GET /api/events/{id}/permissions` - Get event-specific permissions

## Permission System Examples

### Permission Checking
```rust
// Check if user can edit specific event
fn can_edit_event(user_id: &str, event_id: &str) -> bool {
    // Check event-specific permissions first
    // Then check role-based permissions
    // Then check organization-level permissions
    // Consider team memberships and inheritance
}

// Permission hierarchy resolution
fn resolve_permissions(user_id: &str, org_id: &str) -> Vec<Permission> {
    // Get base role permissions
    // Add additional permissions
    // Remove revoked permissions
    // Apply team-based permissions
    // Return final permission set
}
```

### Common Permission Patterns
- **Event Creator**: Full control over their events
- **Team Lead**: Manage team members and team events
- **Org Admin**: Manage organization settings, invite users
- **Event Manager**: Create/edit events, manage registrations
- **Member**: View events, register, basic participation

## Security Considerations
- Permission inheritance validation
- Privilege escalation prevention
- Audit logging for sensitive operations
- Session invalidation on role changes
- Rate limiting on permission checks

## UI/UX Features
- Role management interface
- Permission matrix visualization
- Team hierarchy display
- Invitation workflow
- Permission conflict resolution

## Testing Requirements
- [ ] Permission inheritance edge cases
- [ ] Concurrent permission modifications
- [ ] Permission checking performance
- [ ] Security boundary testing
- [ ] Role hierarchy validation

## Documentation Needed
- RBAC system architecture
- Permission reference guide
- Team setup best practices
- Security model documentation
- Migration guide for existing users

## Dependencies
- Enhanced authentication middleware
- Permission checking utilities
- Audit logging system
- JWT token management updates

## Estimated Effort
**Large (4-5 weeks)** - Complex security system with many edge cases to handle.

## Priority
**High** - Essential for enterprise customers and multi-user organizations.

## Compliance Considerations
- GDPR compliance for user data
- Audit trail requirements
- Data retention policies
- Access control documentation
- Privacy by design principles