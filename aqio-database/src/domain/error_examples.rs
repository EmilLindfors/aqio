// Example error handling scenarios to demonstrate ergonomic error messages
use crate::domain::errors::InfrastructureError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_constraint_parsing() {
        let sqlite_message = "UNIQUE constraint failed: users.email";
        let (table, field, _value) = InfrastructureError::parse_unique_constraint_error(sqlite_message);
        
        assert_eq!(table, Some("users".to_string()));
        assert_eq!(field, Some("email".to_string()));
        
        // Test user-friendly message creation
        let friendly_message = InfrastructureError::create_user_friendly_constraint_message(
            "users", "email", "unique", sqlite_message
        );
        assert_eq!(friendly_message, "This email address is already registered. Please use a different email or try signing in.");
    }

    #[test]
    fn test_not_null_constraint_parsing() {
        let sqlite_message = "NOT NULL constraint failed: users.name";
        let (table, field) = InfrastructureError::parse_not_null_constraint_error(sqlite_message);
        
        assert_eq!(table, Some("users".to_string()));
        assert_eq!(field, Some("name".to_string()));
        
        // Test user-friendly message creation
        let friendly_message = InfrastructureError::create_user_friendly_constraint_message(
            "users", "name", "not_null", sqlite_message
        );
        assert_eq!(friendly_message, "The field 'name' is required and cannot be empty.");
    }

    #[test]
    fn test_event_category_unique_constraint() {
        let sqlite_message = "UNIQUE constraint failed: event_categories.name";
        let (table, field, _value) = InfrastructureError::parse_unique_constraint_error(sqlite_message);
        
        assert_eq!(table, Some("event_categories".to_string()));
        assert_eq!(field, Some("name".to_string()));
        
        let friendly_message = InfrastructureError::create_user_friendly_constraint_message(
            "event_categories", "name", "unique", sqlite_message
        );
        assert_eq!(friendly_message, "This category name is already taken. Please choose a different name.");
    }

    #[test] 
    fn test_registration_unique_constraint() {
        let sqlite_message = "UNIQUE constraint failed: event_registrations.user_id";
        let (table, field, _value) = InfrastructureError::parse_unique_constraint_error(sqlite_message);
        
        assert_eq!(table, Some("event_registrations".to_string()));
        assert_eq!(field, Some("user_id".to_string()));
        
        let friendly_message = InfrastructureError::create_user_friendly_constraint_message(
            "event_registrations", "user_id", "unique", sqlite_message
        );
        assert_eq!(friendly_message, "You are already registered for this event.");
    }

    #[test]
    fn test_check_constraint_parsing() {
        let sqlite_message = "CHECK constraint failed: users.role IN ('admin', 'organizer', 'participant')";
        let constraint = InfrastructureError::parse_check_constraint_error(sqlite_message);
        
        assert_eq!(constraint, Some("users.role IN ('admin', 'organizer', 'participant')".to_string()));
    }

    #[test]
    fn test_generic_field_fallback() {
        let sqlite_message = "UNIQUE constraint failed: custom_table.custom_field";
        let (table, field, _value) = InfrastructureError::parse_unique_constraint_error(sqlite_message);
        
        assert_eq!(table, Some("custom_table".to_string()));
        assert_eq!(field, Some("custom_field".to_string()));
        
        // Should fall back to generic message
        let friendly_message = InfrastructureError::create_user_friendly_constraint_message(
            "custom_table", "custom_field", "unique", sqlite_message
        );
        assert_eq!(friendly_message, "This custom field is already taken. Please choose a different value.");
    }
}

// Examples of how errors would appear to users with the new ergonomic handling:
// 
// Instead of:
// - "UNIQUE constraint failed: users.email"
// 
// Users now see:
// - "This email address is already registered. Please use a different email or try signing in."
//
// Instead of:
// - "NOT NULL constraint failed: users.name"
// 
// Users now see:  
// - "The field 'name' is required and cannot be empty."
//
// Instead of:
// - "UNIQUE constraint failed: event_categories.name"
//
// Users now see:
// - "This category name is already taken. Please choose a different name."
//
// The errors are now:
// 1. ✅ **User-friendly**: Clear, actionable messages
// 2. ✅ **Context-aware**: Specific to the domain (users, events, registrations)
// 3. ✅ **Actionable**: Tell users what they can do to fix the issue
// 4. ✅ **Type-safe**: Proper error types with field information for programmatic handling
// 5. ✅ **Fallback-ready**: Generic messages for unknown tables/fields