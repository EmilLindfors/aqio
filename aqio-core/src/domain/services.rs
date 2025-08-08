use crate::domain::{
    DomainResult, Event, EventInvitation, EventRegistration, User,
    EventDomainValidation, InvitationDomainValidation, DomainValidation,
    InvitationStatus, RegistrationStatus
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// Core domain services (business logic without infrastructure dependencies)

#[derive(Debug, Clone)]
pub struct EventService;

impl EventService {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_event(&self, event: &Event) -> DomainResult<()> {
        event.validate_for_creation()?;
        Ok(())
    }

    pub fn can_register_for_event(&self, event: &Event) -> DomainResult<()> {
        let now = Utc::now();
        event.can_be_registered_for(now)?;
        Ok(())
    }

    pub fn calculate_available_spots(&self, event: &Event, current_registrations: usize) -> Option<i32> {
        event.max_attendees.map(|max| (max as usize).saturating_sub(current_registrations) as i32)
    }

    pub fn should_add_to_waitlist(&self, event: &Event, current_registrations: usize) -> bool {
        if !event.allow_waitlist {
            return false;
        }

        match event.max_attendees {
            Some(max) => current_registrations >= max as usize,
            None => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InvitationService;

impl InvitationService {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_invitation(&self, invitation: &EventInvitation) -> DomainResult<()> {
        invitation.validate_for_creation()?;
        Ok(())
    }

    pub fn can_respond_to_invitation(&self, invitation: &EventInvitation) -> DomainResult<()> {
        let now = Utc::now();
        invitation.can_respond(now)?;
        
        // Check if invitation has expired
        if let Some(expires_at) = invitation.expires_at {
            if now > expires_at {
                return Err(crate::domain::DomainError::business_rule(
                    "Invitation has expired"
                ));
            }
        }
        
        Ok(())
    }

    pub fn generate_invitation_token(&self) -> String {
        Uuid::new_v4().to_string()
    }

    pub fn calculate_expiry(&self, days_from_now: i64) -> DateTime<Utc> {
        Utc::now() + chrono::Duration::days(days_from_now)
    }

    pub fn mark_as_sent(&self, invitation: &mut EventInvitation) {
        invitation.status = InvitationStatus::Sent;
        invitation.sent_at = Some(Utc::now());
        invitation.updated_at = Utc::now();
    }

    pub fn mark_as_opened(&self, invitation: &mut EventInvitation) {
        if invitation.status == InvitationStatus::Sent || invitation.status == InvitationStatus::Delivered {
            invitation.status = InvitationStatus::Opened;
            invitation.opened_at = Some(Utc::now());
            invitation.updated_at = Utc::now();
        }
    }

    pub fn accept_invitation(&self, invitation: &mut EventInvitation) -> DomainResult<()> {
        self.can_respond_to_invitation(invitation)?;
        
        invitation.status = InvitationStatus::Accepted;
        invitation.responded_at = Some(Utc::now());
        invitation.updated_at = Utc::now();
        
        Ok(())
    }

    pub fn decline_invitation(&self, invitation: &mut EventInvitation) -> DomainResult<()> {
        self.can_respond_to_invitation(invitation)?;
        
        invitation.status = InvitationStatus::Declined;
        invitation.responded_at = Some(Utc::now());
        invitation.updated_at = Utc::now();
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RegistrationService;

impl RegistrationService {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_waitlist_position(&self, existing_waitlist_count: usize) -> i32 {
        (existing_waitlist_count + 1) as i32
    }

    pub fn register_for_event(
        &self,
        registration: &mut EventRegistration,
        should_waitlist: bool,
    ) {
        if should_waitlist {
            registration.status = RegistrationStatus::Waitlisted;
            registration.waitlist_added_at = Some(Utc::now());
        } else {
            registration.status = RegistrationStatus::Registered;
        }
        
        registration.registered_at = Utc::now();
        registration.updated_at = Utc::now();
    }

    pub fn cancel_registration(&self, registration: &mut EventRegistration) -> DomainResult<()> {
        if registration.status == RegistrationStatus::Cancelled {
            return Err(crate::domain::DomainError::business_rule(
                "Registration is already cancelled"
            ));
        }

        registration.status = RegistrationStatus::Cancelled;
        registration.cancelled_at = Some(Utc::now());
        registration.updated_at = Utc::now();

        Ok(())
    }

    pub fn promote_from_waitlist(&self, registration: &mut EventRegistration) -> DomainResult<()> {
        if registration.status != RegistrationStatus::Waitlisted {
            return Err(crate::domain::DomainError::business_rule(
                "Only waitlisted registrations can be promoted"
            ));
        }

        registration.status = RegistrationStatus::Registered;
        registration.waitlist_position = None;
        registration.waitlist_added_at = None;
        registration.updated_at = Utc::now();

        Ok(())
    }

    pub fn check_in(&self, registration: &mut EventRegistration) -> DomainResult<()> {
        if registration.status != RegistrationStatus::Registered {
            return Err(crate::domain::DomainError::business_rule(
                "Only registered participants can be checked in"
            ));
        }

        registration.status = RegistrationStatus::Attended;
        registration.checked_in_at = Some(Utc::now());
        registration.updated_at = Utc::now();

        Ok(())
    }

    pub fn mark_no_show(&self, registration: &mut EventRegistration) -> DomainResult<()> {
        if registration.status != RegistrationStatus::Registered {
            return Err(crate::domain::DomainError::business_rule(
                "Only registered participants can be marked as no-show"
            ));
        }

        registration.status = RegistrationStatus::NoShow;
        registration.updated_at = Utc::now();

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_user(&self, user: &User) -> DomainResult<()> {
        user.validate_for_creation()?;
        Ok(())
    }

    pub fn deactivate_user(&self, user: &mut User) {
        user.is_active = false;
        user.updated_at = Utc::now();
    }

    pub fn activate_user(&self, user: &mut User) {
        user.is_active = true;
        user.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{EventStatus, LocationType, InvitationMethod, RegistrationSource};

    fn create_test_event() -> Event {
        Event {
            id: Uuid::new_v4(),
            title: "Test Event".to_string(),
            description: "A test event".to_string(),
            category_id: "test-category".to_string(),
            start_date: Utc::now() + chrono::Duration::hours(24),
            end_date: Utc::now() + chrono::Duration::hours(26),
            timezone: "UTC".to_string(),
            location_type: LocationType::Virtual,
            location_name: None,
            address: None,
            virtual_link: Some("https://example.com".to_string()),
            virtual_access_code: None,
            organizer_id: Uuid::new_v4(),
            co_organizers: vec![],
            is_private: false,
            requires_approval: false,
            max_attendees: Some(10),
            allow_guests: false,
            max_guests_per_person: None,
            registration_opens: None,
            registration_closes: None,
            registration_required: true,
            allow_waitlist: true,
            send_reminders: true,
            collect_dietary_info: false,
            collect_accessibility_info: false,
            image_url: None,
            custom_fields: None,
            status: EventStatus::Published,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_event_service_calculate_available_spots() {
        let service = EventService::new();
        let event = create_test_event();
        
        assert_eq!(service.calculate_available_spots(&event, 5), Some(5));
        assert_eq!(service.calculate_available_spots(&event, 10), Some(0));
        assert_eq!(service.calculate_available_spots(&event, 15), Some(0));
    }

    #[test]
    fn test_event_service_should_add_to_waitlist() {
        let service = EventService::new();
        let event = create_test_event();
        
        assert!(!service.should_add_to_waitlist(&event, 5));
        assert!(service.should_add_to_waitlist(&event, 10));
        assert!(service.should_add_to_waitlist(&event, 15));
    }

    #[test]
    fn test_invitation_service_generate_token() {
        let service = InvitationService::new();
        let token1 = service.generate_invitation_token();
        let token2 = service.generate_invitation_token();
        
        assert_ne!(token1, token2);
        assert!(!token1.is_empty());
        assert!(!token2.is_empty());
    }

    #[test]
    fn test_registration_service_calculate_waitlist_position() {
        let service = RegistrationService::new();
        
        assert_eq!(service.calculate_waitlist_position(0), 1);
        assert_eq!(service.calculate_waitlist_position(5), 6);
    }
}