use crate::api::EventResponse;
use aqio_core::models::EventType;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[component]
pub fn EventCard(event: EventResponse) -> Element {
    let event_type_badge_class = match event.event_type {
        EventType::Conference => "bg-blue-100 text-blue-800",
        EventType::Workshop => "bg-green-100 text-green-800", 
        EventType::Networking => "bg-purple-100 text-purple-800",
        EventType::Training => "bg-yellow-100 text-yellow-800",
        EventType::Other(_) => "bg-gray-100 text-gray-800",
    };

    let event_type_display = match &event.event_type {
        EventType::Conference => "Conference",
        EventType::Workshop => "Workshop", 
        EventType::Networking => "Networking",
        EventType::Training => "Training",
        EventType::Other(name) => name,
    };

    let formatted_start_date = format_date_time(event.start_date);
    let formatted_end_date = format_date_time(event.end_date);

    rsx! {
        div { class: "bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow duration-200 border border-gray-200",
            div { class: "p-6",
                div { class: "flex justify-between items-start mb-4",
                    div { class: "flex-1",
                        h3 { class: "text-xl font-semibold text-gray-900 mb-2",
                            "{event.title}"
                        }
                        span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {event_type_badge_class}",
                            "{event_type_display}"
                        }
                    }
                }
                
                p { class: "text-gray-600 mb-4",
                    "{event.description}"
                }
                
                div { class: "space-y-2 text-sm text-gray-500",
                    div { class: "flex items-center",
                        span { class: "font-medium mr-2", "📅 Start:" }
                        "{formatted_start_date}"
                    }
                    div { class: "flex items-center",
                        span { class: "font-medium mr-2", "📅 End:" }
                        "{formatted_end_date}"
                    }
                    div { class: "flex items-center",
                        span { class: "font-medium mr-2", "📍 Location:" }
                        "{event.location}"
                    }
                    if let Some(max_attendees) = event.max_attendees {
                        div { class: "flex items-center",
                            span { class: "font-medium mr-2", "👥 Max attendees:" }
                            "{max_attendees}"
                        }
                    }
                    if let Some(deadline) = event.registration_deadline {
                        div { class: "flex items-center",
                            span { class: "font-medium mr-2", "⏰ Registration deadline:" }
                            "{format_date_time(deadline)}"
                        }
                    }
                }
                
                div { class: "mt-4 pt-4 border-t border-gray-200",
                    button { class: "w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-md transition-colors duration-200",
                        "Register for Event"
                    }
                }
            }
        }
    }
}

fn format_date_time(dt: DateTime<Utc>) -> String {
    dt.format("%B %d, %Y at %H:%M UTC").to_string()
}