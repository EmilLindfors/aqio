use crate::api::{ApiClient, EventResponse};
use chrono::Local;
use dioxus::prelude::*;
use uuid::Uuid;
use aqio_core::models::EventType;

#[component]
pub fn EventDetailPage(event_id: String) -> Element {
    let event = use_resource(move || {
        let id = event_id.clone();
        async move {
            // Parse the UUID
            let event_uuid = Uuid::parse_str(&id).map_err(|e| format!("Invalid event ID: {}", e))?;
            
            // Fetch the event details
            let api_client = ApiClient::new();
            api_client.get_event(event_uuid).await
                .map_err(|e| format!("Failed to load event: {}", e))
        }
    });

    rsx! {
        div {
            style: "max-width: 800px; margin: 0 auto; padding: 2rem 1rem;",
            
            match event() {
                Some(Ok(event)) => rsx! {
                    EventDetailContent { event: event }
                },
                Some(Err(err)) => rsx! {
                    ErrorDisplay { message: err }
                },
                None => rsx! {
                    LoadingSpinner {}
                }
            }
        }
    }
}

#[component]
fn EventDetailContent(event: EventResponse) -> Element {
    let mut show_registration = use_signal(|| false);
    
    let start_date = event.start_date.with_timezone(&Local).format("%A, %B %d, %Y").to_string();
    let start_time = event.start_date.with_timezone(&Local).format("%H:%M").to_string();
    let end_date = event.end_date.with_timezone(&Local).format("%A, %B %d, %Y").to_string();
    let end_time = event.end_date.with_timezone(&Local).format("%H:%M").to_string();
    
    let event_type_display = match &event.event_type {
        EventType::Conference => ("Conference", "#2563eb"),
        EventType::Workshop => ("Workshop", "#16a34a"),
        EventType::Networking => ("Networking", "#7c3aed"),
        EventType::Training => ("Training", "#ea580c"),
        EventType::Other(s) => (s.as_str(), "#6b7280"),
    };

    let is_past = event.end_date < chrono::Utc::now();
    let registration_deadline_passed = event.registration_deadline
        .map(|deadline| deadline < chrono::Utc::now())
        .unwrap_or(false);

    rsx! {
        // Hero Section
        div {
            style: "background: linear-gradient(135deg, #1e40af 0%, #3730a3 100%); color: white; padding: 3rem 2rem; border-radius: 0.5rem; margin-bottom: 2rem;",
            
            // Event Type Badge
            div {
                style: "margin-bottom: 1rem;",
                span {
                    style: format!(
                        "background: {}; color: white; padding: 0.5rem 1rem; border-radius: 9999px; font-size: 0.875rem; font-weight: 500; opacity: 0.9;",
                        event_type_display.1
                    ),
                    "{event_type_display.0}"
                }
            }
            
            h1 {
                style: "font-size: 2.5rem; font-weight: bold; margin-bottom: 1rem; line-height: 1.2;",
                "{event.title}"
            }
            
            div {
                style: "display: flex; flex-wrap: wrap; gap: 2rem; font-size: 1.125rem; opacity: 0.9;",
                div {
                    style: "display: flex; align-items: center; gap: 0.5rem;",
                    span { style: "font-size: 1.25rem;", "📅" }
                    div {
                        if start_date == end_date {
                            "{start_date}"
                        } else {
                            "{start_date} - {end_date}"
                        }
                        div { style: "font-size: 0.875rem; opacity: 0.8;", "{start_time} - {end_time}" }
                    }
                }
                div {
                    style: "display: flex; align-items: center; gap: 0.5rem;",
                    span { style: "font-size: 1.25rem;", "📍" }
                    "{event.location}"
                }
            }
        }

        // Main Content
        div {
            style: "display: grid; grid-template-columns: 2fr 1fr; gap: 2rem;",
            
            // Left Column - Event Details
            div {
                // Description Section
                EventSection {
                    title: "About This Event",
                    div {
                        style: "font-size: 1.125rem; line-height: 1.7; color: #374151;",
                        "{event.description}"
                    }
                }

                // Event Information
                EventSection {
                    title: "Event Information",
                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem;",
                        
                        EventInfoItem {
                            icon: "📅",
                            label: "Date & Time",
                            value: if start_date == end_date {
                                format!("{}\n{} - {}", start_date, start_time, end_time)
                            } else {
                                format!("{}\n{}\n{}\n{}", start_date, start_time, end_date, end_time)
                            }
                        }
                        
                        EventInfoItem {
                            icon: "📍",
                            label: "Location",
                            value: event.location.clone()
                        }
                        
                        EventInfoItem {
                            icon: "🏷️",
                            label: "Event Type",
                            value: event_type_display.0.to_string()
                        }
                        
                        if let Some(max) = event.max_attendees {
                            EventInfoItem {
                                icon: "👥",
                                label: "Max Attendees",
                                value: format!("{} people", max)
                            }
                        }
                        
                        if let Some(deadline) = event.registration_deadline {
                            EventInfoItem {
                                icon: "⏰",
                                label: "Registration Deadline",
                                value: deadline.with_timezone(&Local).format("%B %d, %Y at %H:%M").to_string()
                            }
                        }
                    }
                }

                // Additional Details (if any)
                EventSection {
                    title: "Additional Information",
                    div {
                        style: "color: #6b7280;",
                        p { "Event ID: {event.id}" }
                        p { "Created: {event.created_at.with_timezone(&Local).format(\"%B %d, %Y\")}" }
                        p { "Last Updated: {event.updated_at.with_timezone(&Local).format(\"%B %d, %Y\")}" }
                    }
                }
            }

            // Right Column - Registration & Actions
            div {
                EventRegistrationCard {
                    event: event.clone(),
                    is_past: is_past,
                    registration_deadline_passed: registration_deadline_passed,
                    show_registration: show_registration,
                }
            }
        }

        // Registration Modal
        if show_registration() {
            EventRegistrationModal {
                event: event.clone(),
                on_close: move |_| show_registration.set(false),
            }
        }
    }
}

#[component]
fn EventSection(title: String, children: Element) -> Element {
    rsx! {
        div {
            style: "margin-bottom: 3rem;",
            h2 {
                style: "font-size: 1.5rem; font-weight: bold; color: #1f2937; margin-bottom: 1.5rem; padding-bottom: 0.5rem; border-bottom: 2px solid #e5e7eb;",
                "{title}"
            }
            {children}
        }
    }
}

#[component]
fn EventInfoItem(icon: String, label: String, value: String) -> Element {
    rsx! {
        div {
            style: "background: #f9fafb; padding: 1.5rem; border-radius: 0.5rem; border: 1px solid #e5e7eb;",
            div {
                style: "display: flex; align-items: center; gap: 0.75rem; margin-bottom: 0.5rem;",
                span { style: "font-size: 1.25rem;", "{icon}" }
                span { style: "font-weight: 600; color: #374151;", "{label}" }
            }
            div {
                style: "color: #6b7280; white-space: pre-line;",
                "{value}"
            }
        }
    }
}

#[component]
fn EventRegistrationCard(
    event: EventResponse, 
    is_past: bool, 
    registration_deadline_passed: bool,
    show_registration: Signal<bool>
) -> Element {
    rsx! {
        div {
            style: "background: white; border-radius: 0.5rem; border: 1px solid #e5e7eb; padding: 2rem; position: sticky; top: 2rem;",
            
            h3 {
                style: "font-size: 1.25rem; font-weight: bold; color: #1f2937; margin-bottom: 1.5rem;",
                "Event Registration"
            }
            
            if is_past {
                div {
                    style: "background: #fef2f2; border: 1px solid #fecaca; border-radius: 0.375rem; padding: 1rem; margin-bottom: 1.5rem;",
                    div { 
                        style: "color: #991b1b; font-weight: 500;", 
                        "This event has already ended" 
                    }
                }
            } else if registration_deadline_passed {
                div {
                    style: "background: #fef3c7; border: 1px solid #fbbf24; border-radius: 0.375rem; padding: 1rem; margin-bottom: 1.5rem;",
                    div { 
                        style: "color: #92400e; font-weight: 500;", 
                        "Registration deadline has passed" 
                    }
                }
            }
            
            // Registration Status
            div {
                style: "margin-bottom: 1.5rem;",
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;",
                    span { style: "color: #6b7280;", "Status:" }
                    if is_past {
                        span { style: "color: #dc2626; font-weight: 500;", "Event Ended" }
                    } else if registration_deadline_passed {
                        span { style: "color: #d97706; font-weight: 500;", "Registration Closed" }
                    } else {
                        span { style: "color: #16a34a; font-weight: 500;", "Registration Open" }
                    }
                }
                
                if let Some(max) = event.max_attendees {
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;",
                        span { style: "color: #6b7280;", "Capacity:" }
                        span { style: "font-weight: 500;", "0 / {max}" }
                    }
                }
                
                if let Some(deadline) = event.registration_deadline {
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center;",
                        span { style: "color: #6b7280;", "Deadline:" }
                        span { 
                            style: "font-weight: 500; font-size: 0.875rem;", 
                            "{deadline.with_timezone(&Local).format(\"%b %d, %Y\")}" 
                        }
                    }
                }
            }
            
            // Registration Button
            if !is_past && !registration_deadline_passed {
                button {
                    style: "width: 100%; background: #2563eb; color: white; font-weight: 500; padding: 0.75rem 1rem; border: none; border-radius: 0.375rem; cursor: pointer; transition: background-color 0.2s; margin-bottom: 1rem;",
                    onclick: move |_| show_registration.set(true),
                    "Register for Event"
                }
            }
            
            // Additional Actions
            div {
                style: "display: flex; flex-direction: column; gap: 0.75rem;",
                {
                    let event_for_calendar = event.clone();
                    rsx! {
                        button {
                            style: "width: 100%; background: #f3f4f6; color: #374151; font-weight: 500; padding: 0.75rem 1rem; border: 1px solid #d1d5db; border-radius: 0.375rem; cursor: pointer; transition: background-color 0.2s;",
                            onclick: move |_| {
                                export_event_to_calendar(&event_for_calendar);
                            },
                            "📅 Add to Calendar"
                        }
                    }
                }
                
                {
                    let event_for_share = event.clone();
                    rsx! {
                        button {
                            style: "width: 100%; background: #f3f4f6; color: #374151; font-weight: 500; padding: 0.75rem 1rem; border: 1px solid #d1d5db; border-radius: 0.375rem; cursor: pointer; transition: background-color 0.2s;",
                            onclick: move |_| {
                                share_event(&event_for_share);
                            },
                            "📤 Share Event"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EventRegistrationModal(event: EventResponse, on_close: EventHandler<()>) -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut phone = use_signal(String::new);
    let mut company = use_signal(String::new);
    let mut dietary_requirements = use_signal(String::new);
    let mut special_requests = use_signal(String::new);
    let mut submitting = use_signal(|| false);

    let event_title = event.title.clone();
    let handle_submit = move |_| {
        let event_title = event_title.clone();
        async move {
            if name().trim().is_empty() || email().trim().is_empty() {
                return;
            }
            
            submitting.set(true);
            
            // Here you would typically send the registration data to your API
            // For now, we'll just simulate a successful registration
            gloo_timers::future::TimeoutFuture::new(1000).await;
            
            submitting.set(false);
            on_close.call(());
            
            // Show success message or redirect
            log::info!("Registration submitted for event: {}", event_title);
        }
    };

    rsx! {
        div {
            style: "position: fixed; inset: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; padding: 1rem; z-index: 50;",
            onclick: move |_| on_close.call(()),
            
            div {
                style: "background: white; border-radius: 0.5rem; max-width: 500px; width: 100%; max-height: 90vh; overflow-y: auto;",
                onclick: move |e| e.stop_propagation(),
                
                // Header
                div {
                    style: "padding: 1.5rem; border-bottom: 1px solid #e5e7eb;",
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center;",
                        h3 {
                            style: "font-size: 1.25rem; font-weight: bold; color: #1f2937;",
                            "Register for Event"
                        }
                        button {
                            style: "color: #6b7280; hover: color: #374151; border: none; background: none; cursor: pointer; font-size: 1.25rem;",
                            onclick: move |_| on_close.call(()),
                            "✕"
                        }
                    }
                    div {
                        style: "margin-top: 0.5rem; color: #6b7280; font-size: 0.875rem;",
                        "{event.title}"
                    }
                }
                
                // Form
                form {
                    style: "padding: 1.5rem;",
                    onsubmit: handle_submit,
                    
                    div {
                        style: "display: grid; gap: 1rem;",
                        
                        RegistrationField {
                            label: "Full Name *",
                            input {
                                r#type: "text",
                                value: "{name}",
                                oninput: move |e| name.set(e.value()),
                                placeholder: "Enter your full name",
                                required: true,
                                style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem;",
                            }
                        }
                        
                        RegistrationField {
                            label: "Email Address *",
                            input {
                                r#type: "email",
                                value: "{email}",
                                oninput: move |e| email.set(e.value()),
                                placeholder: "Enter your email address",
                                required: true,
                                style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem;",
                            }
                        }
                        
                        RegistrationField {
                            label: "Phone Number",
                            input {
                                r#type: "tel",
                                value: "{phone}",
                                oninput: move |e| phone.set(e.value()),
                                placeholder: "Enter your phone number",
                                style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem;",
                            }
                        }
                        
                        RegistrationField {
                            label: "Company/Organization",
                            input {
                                r#type: "text",
                                value: "{company}",
                                oninput: move |e| company.set(e.value()),
                                placeholder: "Enter your company or organization",
                                style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem;",
                            }
                        }
                        
                        RegistrationField {
                            label: "Dietary Requirements",
                            textarea {
                                value: "{dietary_requirements}",
                                oninput: move |e| dietary_requirements.set(e.value()),
                                placeholder: "Any dietary requirements or allergies...",
                                rows: "2",
                                style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem; resize: vertical;",
                            }
                        }
                        
                        RegistrationField {
                            label: "Special Requests",
                            textarea {
                                value: "{special_requests}",
                                oninput: move |e| special_requests.set(e.value()),
                                placeholder: "Any special requests or accommodations...",
                                rows: "3",
                                style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem; resize: vertical;",
                            }
                        }
                    }
                    
                    // Footer
                    div {
                        style: "margin-top: 2rem; display: flex; gap: 1rem;",
                        
                        button {
                            r#type: "submit",
                            disabled: submitting() || name().trim().is_empty() || email().trim().is_empty(),
                            style: format!(
                                "flex: 1; font-weight: 500; padding: 0.75rem 1rem; border: none; border-radius: 0.375rem; cursor: pointer; transition: opacity 0.2s; {}",
                                if submitting() || name().trim().is_empty() || email().trim().is_empty() {
                                    "background: #9ca3af; color: white;"
                                } else {
                                    "background: #2563eb; color: white;"
                                }
                            ),
                            if submitting() {
                                "Submitting..."
                            } else {
                                "Register"
                            }
                        }
                        
                        button {
                            r#type: "button",
                            style: "flex: 1; background: #f3f4f6; color: #374151; font-weight: 500; padding: 0.75rem 1rem; border: 1px solid #d1d5db; border-radius: 0.375rem; cursor: pointer; transition: background-color 0.2s;",
                            onclick: move |_| on_close.call(()),
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RegistrationField(label: String, children: Element) -> Element {
    rsx! {
        div {
            label {
                style: "display: block; font-weight: 500; color: #374151; margin-bottom: 0.5rem;",
                "{label}"
            }
            div {
                style: "width: 100%;",
                {children}
            }
        }
    }
}

#[component]
fn LoadingSpinner() -> Element {
    rsx! {
        div {
            style: "display: flex; justify-content: center; align-items: center; padding: 4rem 0;",
            div {
                style: "display: flex; flex-direction: column; align-items: center; gap: 1rem;",
                div {
                    style: "width: 3rem; height: 3rem; border: 2px solid #e5e7eb; border-top-color: #2563eb; border-radius: 50%;",
                    "⟳"
                }
                span {
                    style: "color: #6b7280;",
                    "Loading event details..."
                }
            }
        }
    }
}

#[component]
fn ErrorDisplay(message: String) -> Element {
    rsx! {
        div {
            style: "background: #fef2f2; border: 1px solid #fecaca; border-radius: 0.5rem; padding: 2rem; text-align: center;",
            div {
                style: "font-size: 2rem; margin-bottom: 1rem;",
                "⚠️"
            }
            h2 {
                style: "font-size: 1.25rem; font-weight: bold; color: #991b1b; margin-bottom: 0.5rem;",
                "Error Loading Event"
            }
            p {
                style: "color: #991b1b;",
                "{message}"
            }
        }
    }
}

// Helper functions
fn export_event_to_calendar(event: &EventResponse) {
    // Create iCal format string
    let _ical_content = format!(
        "BEGIN:VCALENDAR\r\n\
        VERSION:2.0\r\n\
        PRODID:-//AQIO//Norwegian Aquaculture Events//EN\r\n\
        BEGIN:VEVENT\r\n\
        UID:{}@aqio.no\r\n\
        DTSTAMP:{}\r\n\
        DTSTART:{}\r\n\
        DTEND:{}\r\n\
        SUMMARY:{}\r\n\
        DESCRIPTION:{}\r\n\
        LOCATION:{}\r\n\
        END:VEVENT\r\n\
        END:VCALENDAR",
        event.id,
        chrono::Utc::now().format("%Y%m%dT%H%M%SZ"),
        event.start_date.format("%Y%m%dT%H%M%SZ"),
        event.end_date.format("%Y%m%dT%H%M%SZ"),
        event.title,
        event.description,
        event.location
    );
    
    // In a real implementation, this would trigger a download
    log::info!("Export event to iCal: {}", event.title);
    
    // Create a downloadable link
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = window)]
            fn open(url: &str, target: &str);
        }
        
        let data_url = format!("data:text/calendar;charset=utf-8,{}", 
            js_sys::encode_uri_component(&_ical_content));
        
        // This would ideally trigger a download, but for now just log
        log::info!("Would download: {}", data_url);
    }
}

fn share_event(event: &EventResponse) {
    let _share_url = format!("{}/events/{}", 
        web_sys::window().unwrap().location().origin().unwrap(),
        event.id
    );
    
    let _share_text = format!("Check out this event: {} at {}", 
        event.title, 
        event.location
    );
    
    #[cfg(target_arch = "wasm32")]
    {
        // For now, just log the share information
        // Web Share API implementation would require additional web-sys features
        log::info!("Share: {} - {}", _share_text, _share_url);
    }
}