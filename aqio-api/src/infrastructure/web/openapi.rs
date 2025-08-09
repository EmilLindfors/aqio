use utoipa::OpenApi;
use crate::domain::dto::*;
use aqio_core::*;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Aqio Event Management API",
        description = "A comprehensive event management platform built for the Norwegian aquaculture industry",
        version = "1.0.0",
        contact(
            name = "Aqio Support",
            email = "support@aqio.no"
        )
    ),
    paths(
        crate::infrastructure::web::handlers::health_check,
        crate::infrastructure::web::handlers::simple_health,
        crate::infrastructure::web::handlers::list_events,
        crate::infrastructure::web::handlers::get_event,
        crate::infrastructure::web::handlers::create_event,
        crate::infrastructure::web::handlers::update_event,
        crate::infrastructure::web::handlers::delete_event,
        crate::infrastructure::web::handlers::get_my_events,
    ),
    components(
        schemas(
            // Core domain models
            UserRole,
            User,
            IndustryType,
            Company,
            UserProfile,
            EventCategory,
            LocationType,
            EventStatus,
            Event,
            InvitationMethod,
            InvitationStatus,
            EventInvitation,
            RegistrationStatus,
            RegistrationSource,
            EventRegistration,
            ExternalContact,
            EventFilter,
            PaginationParams,
            PaginatedResult<Event>,
            PaginatedResult<User>,
            PaginatedResult<EventRegistration>,
            PaginatedResult<EventInvitation>,
            // API DTOs
            CreateEventRequest,
            EventResponse,
            ListEventsQuery,
            PaginationQuery,
            PaginatedEventResponse,
            PaginationInfo,
            ApiResponse<EventResponse>,
            ApiResponse<UserResponse>,
            ApiResponse<EventCategoryResponse>,
            ApiResponse<InvitationResponse>,
            ApiResponse<RegistrationResponse>,
            CreateUserRequest,
            UpdateUserRequest,
            UserResponse,
            PaginatedUserResponse,
            CreateEventCategoryRequest,
            UpdateEventCategoryRequest,
            EventCategoryResponse,
            HealthResponse,
            HealthServices,
            ServiceHealth,
            CreateInvitationRequest,
            UpdateInvitationStatusRequest,
            InvitationResponse,
            CreateRegistrationRequest,
            UpdateRegistrationRequest,
            UpdateRegistrationStatusRequest,
            RegistrationResponse,
            EventRegistrationStatsResponse,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "events", description = "Event management"),
        (name = "users", description = "User management"),
        (name = "categories", description = "Event category management"),
        (name = "invitations", description = "Invitation management"),
        (name = "registrations", description = "Registration management"),
    )
)]
pub struct ApiDoc;