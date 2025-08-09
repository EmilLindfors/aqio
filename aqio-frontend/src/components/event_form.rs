use crate::api::{ApiClient, CreateEventRequest};
use aqio_core::models::EventType;
use chrono::{DateTime, Utc, NaiveDateTime};
use dioxus::prelude::*;

#[component]
pub fn EventForm() -> Element {
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut event_type = use_signal(|| EventType::Conference);
    let mut custom_event_type = use_signal(String::new);
    let mut start_date = use_signal(String::new);
    let mut start_time = use_signal(String::new);
    let mut end_date = use_signal(String::new);
    let mut end_time = use_signal(String::new);
    let mut location = use_signal(String::new);
    let mut max_attendees = use_signal(String::new);
    let mut registration_deadline = use_signal(String::new);
    let mut registration_deadline_time = use_signal(String::new);
    let mut header_image = use_signal(|| None::<String>); // Base64 encoded image data
    let mut header_image_name = use_signal(String::new);
    let mut submitting = use_signal(|| false);
    let mut success_message = use_signal(|| None::<String>);
    let mut error_message = use_signal(|| None::<String>);

    let submit_form = move |evt: Event<FormData>| {
        evt.prevent_default();
        spawn(async move {
            submitting.set(true);
            success_message.set(None);
            // Don't clear error_message immediately to avoid flashing

            // Validate required fields
            if title().is_empty() || description().is_empty() || start_date().is_empty() || 
               start_time().is_empty() || end_date().is_empty() || end_time().is_empty() || location().is_empty() {
                error_message.set(Some("Please fill in all required fields".to_string()));
                submitting.set(false);
                return;
            }
            
            // Clear previous errors only after validation passes
            error_message.set(None);

            // Parse dates
            let start_datetime = match parse_datetime(&start_date(), &start_time()) {
                Ok(dt) => dt,
                Err(e) => {
                    error_message.set(Some(format!("Invalid start date/time: {}", e)));
                    submitting.set(false);
                    return;
                }
            };

            let end_datetime = match parse_datetime(&end_date(), &end_time()) {
                Ok(dt) => dt,
                Err(e) => {
                    error_message.set(Some(format!("Invalid end date/time: {}", e)));
                    submitting.set(false);
                    return;
                }
            };

            // Parse optional registration deadline
            let reg_deadline = if !registration_deadline().is_empty() && !registration_deadline_time().is_empty() {
                match parse_datetime(&registration_deadline(), &registration_deadline_time()) {
                    Ok(dt) => Some(dt),
                    Err(e) => {
                        error_message.set(Some(format!("Invalid registration deadline: {}", e)));
                        submitting.set(false);
                        return;
                    }
                }
            } else {
                None
            };

            // Parse max attendees
            let max_att = if max_attendees().is_empty() {
                None
            } else {
                match max_attendees().parse::<i32>() {
                    Ok(n) if n > 0 => Some(n),
                    _ => {
                        error_message.set(Some("Max attendees must be a positive number".to_string()));
                        submitting.set(false);
                        return;
                    }
                }
            };

            // Determine event type
            let final_event_type = match event_type() {
                EventType::Other(_) if !custom_event_type().is_empty() => EventType::Other(custom_event_type()),
                other => other,
            };

            let request = CreateEventRequest {
                title: title(),
                description: description(),
                event_type: final_event_type,
                start_date: start_datetime,
                end_date: end_datetime,
                location: location(),
                max_attendees: max_att,
                registration_deadline: reg_deadline,
            };

            let api_client = ApiClient::new(); // In production, would include auth token
            
            match api_client.create_event(request).await {
                Ok(_) => {
                    success_message.set(Some("Event created successfully!".to_string()));
                    // Reset form
                    title.set(String::new());
                    description.set(String::new());
                    event_type.set(EventType::Conference);
                    custom_event_type.set(String::new());
                    start_date.set(String::new());
                    start_time.set(String::new());
                    end_date.set(String::new());
                    end_time.set(String::new());
                    location.set(String::new());
                    max_attendees.set(String::new());
                    registration_deadline.set(String::new());
                    registration_deadline_time.set(String::new());
                    header_image.set(None);
                    header_image_name.set(String::new());
                }
                Err(e) => {
                    let error_msg = format!("Failed to create event: {}", e);
                    
                    // Debug: Log the full error
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(&format!("❌ Event creation error: {}", error_msg).into());
                    
                    error_message.set(Some(error_msg));
                }
            }
            submitting.set(false);
        });
    };

    rsx! {
        div { 
            style: "max-width: 1200px; margin: 0 auto; padding: 2rem 1rem;",
            div { 
                style: "max-width: 48rem; margin: 0 auto;",
                EventFormHeader {
                    title: "🐟 Create New Event",
                    subtitle: "Organize a new event for the Norwegian aquaculture community"
                }

                if let Some(success) = success_message() {
                    SuccessMessage { message: success }
                }

                if let Some(error) = error_message() {
                    ErrorMessage { message: error }
                }

                EventFormContainer {
                    onsubmit: submit_form,
                    
                    FormField {
                        label: "Event Title *",
                        input {
                            r#type: "text",
                            id: "title",
                            style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem;",
                            placeholder: "e.g., Norwegian Salmon Farming Conference 2024",
                            value: "{title}",
                            oninput: move |e| title.set(e.value())
                        }
                    }

                    FormField {
                        label: "Description *",
                        textarea {
                            id: "description",
                            rows: "4",
                            style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem; resize: vertical;",
                            placeholder: "Describe your event, agenda, and what attendees can expect...",
                            value: "{description}",
                            oninput: move |e| description.set(e.value())
                        }
                    }

                    HeaderImageField {
                        header_image: header_image(),
                        header_image_name: header_image_name()
                    }

                    EventTypeField {
                        event_type: event_type(),
                        custom_event_type: custom_event_type(),
                        on_type_change: move |new_type| event_type.set(new_type),
                        on_custom_change: move |custom| custom_event_type.set(custom)
                    }

                    DateTimeFields {
                        start_date_label: "Start Date *",
                        start_time_label: "Start Time *",
                        start_date: start_date(),
                        start_time: start_time(),
                        on_date_change: move |date| start_date.set(date),
                        on_time_change: move |time| start_time.set(time)
                    }

                    DateTimeFields {
                        start_date_label: "End Date *",
                        start_time_label: "End Time *",
                        start_date: end_date(),
                        start_time: end_time(),
                        on_date_change: move |date| end_date.set(date),
                        on_time_change: move |time| end_time.set(time)
                    }

                    FormField {
                        label: "Location *",
                        input {
                            r#type: "text",
                            id: "location",
                            style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem;",
                            placeholder: "e.g., Bergen Convention Centre, Norway",
                            value: "{location}",
                            oninput: move |e| location.set(e.value())
                        }
                    }

                    FormField {
                        label: "Maximum Attendees (optional)",
                        input {
                            r#type: "number",
                            id: "max_attendees",
                            min: "1",
                            style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem;",
                            placeholder: "e.g., 100",
                            value: "{max_attendees}",
                            oninput: move |e| max_attendees.set(e.value())
                        }
                    }

                    DateTimeFields {
                        start_date_label: "Registration Deadline (optional)",
                        start_time_label: "Deadline Time",
                        start_date: registration_deadline(),
                        start_time: registration_deadline_time(),
                        on_date_change: move |date| registration_deadline.set(date),
                        on_time_change: move |time| registration_deadline_time.set(time)
                    }

                    SubmitButton {
                        submitting: submitting(),
                        creating_text: "Creating Event...",
                        default_text: "🐟 Create Event"
                    }
                }
            }
        }
    }
}

fn parse_datetime(date_str: &str, time_str: &str) -> Result<DateTime<Utc>, String> {
    let datetime_str = format!("{} {}", date_str, time_str);
    let naive = NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M")
        .map_err(|e| e.to_string())?;
    Ok(DateTime::from_naive_utc_and_offset(naive, Utc))
}

#[component]
fn EventFormHeader(title: String, subtitle: String) -> Element {
    rsx! {
        div {
            style: "margin-bottom: 2rem;",
            h1 {
                style: "font-size: 1.875rem; font-weight: bold; color: #111827; margin-bottom: 0.5rem;",
                "{title}"
            }
            p {
                style: "color: #6b7280;",
                "{subtitle}"
            }
        }
    }
}

#[component]
fn SuccessMessage(message: String) -> Element {
    rsx! {
        div {
            style: "background-color: #f0fdf4; border: 1px solid #bbf7d0; border-radius: 0.5rem; padding: 1rem; margin-bottom: 1.5rem;",
            div {
                style: "display: flex; align-items: flex-start;",
                div {
                    style: "flex-shrink: 0;",
                    span {
                        style: "color: #16a34a;",
                        "✅"
                    }
                }
                div {
                    style: "margin-left: 0.75rem;",
                    h3 {
                        style: "font-size: 0.875rem; font-weight: 500; color: #166534;",
                        "{message}"
                    }
                }
            }
        }
    }
}

#[component]
fn ErrorMessage(message: String) -> Element {
    rsx! {
        div {
            style: "background-color: #fef2f2; border: 1px solid #fecaca; border-radius: 0.5rem; padding: 1rem; margin-bottom: 1.5rem;",
            div {
                style: "display: flex; align-items: flex-start;",
                div {
                    style: "flex-shrink: 0;",
                    span {
                        style: "color: #f87171;",
                        "⚠️"
                    }
                }
                div {
                    style: "margin-left: 0.75rem;",
                    h3 {
                        style: "font-size: 0.875rem; font-weight: 500; color: #991b1b;",
                        "{message}"
                    }
                }
            }
        }
    }
}

#[component]
fn EventFormContainer(onsubmit: EventHandler<Event<FormData>>, children: Element) -> Element {
    rsx! {
        form {
            style: "background-color: white; box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05); border-radius: 0.5rem; padding: 1.5rem; display: flex; flex-direction: column; gap: 1.5rem;",
            onsubmit: onsubmit,
            {children}
        }
    }
}

#[component]
fn FormField(label: String, children: Element) -> Element {
    rsx! {
        div {
            label {
                style: "display: block; font-size: 0.875rem; font-weight: 500; color: #374151; margin-bottom: 0.5rem;",
                "{label}"
            }
            {children}
        }
    }
}

#[component]
fn EventTypeField(
    event_type: EventType,
    custom_event_type: String,
    on_type_change: EventHandler<EventType>,
    on_custom_change: EventHandler<String>
) -> Element {
    rsx! {
        FormField {
            label: "Event Type *",
            select {
                id: "event_type",
                style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem;",
                onchange: move |e| {
                    let selected_value = e.value();
                    on_type_change.call(match selected_value.as_str() {
                        "Conference" => EventType::Conference,
                        "Workshop" => EventType::Workshop,
                        "Networking" => EventType::Networking,
                        "Training" => EventType::Training,
                        "Other" => EventType::Other(String::new()),
                        _ => EventType::Conference,
                    });
                },
                option { value: "Conference", "Conference" }
                option { value: "Workshop", "Workshop" }
                option { value: "Networking", "Networking" }
                option { value: "Training", "Training" }
                option { value: "Other", "Other" }
            }
            
            if matches!(event_type, EventType::Other(_)) {
                input {
                    r#type: "text",
                    style: "margin-top: 0.5rem; width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem;",
                    placeholder: "Specify custom event type",
                    value: "{custom_event_type}",
                    oninput: move |e| on_custom_change.call(e.value())
                }
            }
        }
    }
}

#[component]
fn DateTimeFields(
    start_date_label: String,
    start_time_label: String,
    start_date: String,
    start_time: String,
    on_date_change: EventHandler<String>,
    on_time_change: EventHandler<String>
) -> Element {
    rsx! {
        div {
            style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;",
            FormField {
                label: start_date_label,
                input {
                    r#type: "date",
                    style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem;",
                    value: "{start_date}",
                    oninput: move |e| on_date_change.call(e.value())
                }
            }
            FormField {
                label: start_time_label,
                input {
                    r#type: "time",
                    style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem;",
                    value: "{start_time}",
                    oninput: move |e| on_time_change.call(e.value())
                }
            }
        }
    }
}

#[component]
fn SubmitButton(submitting: bool, creating_text: String, default_text: String) -> Element {
    rsx! {
        div {
            button {
                r#type: "submit",
                style: "width: 100%; display: flex; justify-content: center; align-items: center; padding: 0.5rem 1rem; border: none; border-radius: 0.375rem; box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05); font-size: 0.875rem; font-weight: 500; color: white; background-color: #2563eb; cursor: pointer; transition: background-color 0.2s;",
                disabled: submitting,
                if submitting {
                    "{creating_text}"
                } else {
                    "{default_text}"
                }
            }
        }
    }
}

#[component]
fn HeaderImageField(
    header_image: Option<String>,
    header_image_name: String
) -> Element {
    rsx! {
        FormField {
            label: "Header Image (optional)",
            div {
                style: "display: flex; flex-direction: column; gap: 1rem;",
                
                // For now, just show a disabled message
                div {
                    style: "border: 2px dashed #d1d5db; border-radius: 0.5rem; padding: 2rem; text-align: center; background-color: #f9fafb;",
                    div {
                        style: "margin-bottom: 1rem;",
                        span {
                            style: "font-size: 2rem;",
                            "🖼️"
                        }
                    }
                    p {
                        style: "color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;",
                        "Image upload feature coming soon..."
                    }
                    p {
                        style: "color: #9ca3af; font-size: 0.75rem;",
                        "Backend API needs to be updated to support image uploads"
                    }
                }
            }
        }
    }
}

