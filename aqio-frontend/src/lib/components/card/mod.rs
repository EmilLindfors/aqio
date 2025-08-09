use dioxus::prelude::*;
use aqio_core::models::EventType;

#[component]
pub fn Card(
    #[props(default = false)] elevated: bool,
    #[props(default = false)] interactive: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        
        div {
            style: "
                background-color: white;
                border: 1px solid #E2E8F0;
                border-radius: 0.5rem;
                padding: 1.5rem;
                transition: all 150ms ease-in-out;
                cursor: pointer;
            ",
            class: "aqio-card",
            "data-elevated": elevated,
            "data-interactive": interactive,
            ..attributes,
            {children}
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct EventCardModel {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub location: String,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub max_attendees: Option<u32>,
    pub event_type: EventType,
}

#[component] 
pub fn EventCard(
    event: EventCardModel,
    #[props(default)] on_click: EventHandler<EventCardModel>,
) -> Element {
    let event_type_color = match event.event_type {
        EventType::Conference => "blue",
        EventType::Workshop => "green", 
        EventType::Networking => "purple",
        EventType::Training => "orange",
        EventType::Other(_) => "gray",
    };

    let event_type_display = match &event.event_type {
        EventType::Conference => "Conference",
        EventType::Workshop => "Workshop",
        EventType::Networking => "Networking",
        EventType::Training => "Training", 
        EventType::Other(s) => s,
    };

    let start_date = event.start_date.format("%B %d, %Y at %H:%M");
    let event_clone = event.clone();

    rsx! {
        div {
            onclick: move |_| on_click.call(event_clone.clone()),
            Card {
                interactive: true,
                class: "aqio-event-card",
            
            div { class: "aqio-event-card-header",
                div { class: "aqio-event-card-badge aqio-event-badge-{event_type_color}",
                    "{event_type_display}"
                }
                div { class: "aqio-event-card-date",
                    "📅 {start_date}"
                }
            }
            
            div { class: "aqio-event-card-content",
                h3 { class: "aqio-event-card-title",
                    "{event.title}"
                }
                p { class: "aqio-event-card-description",
                    "{event.description}"
                }
            }
            
            div { class: "aqio-event-card-footer",
                div { class: "aqio-event-card-location",
                    "📍 {event.location}"
                }
                if let Some(max) = event.max_attendees {
                    div { class: "aqio-event-card-attendees",
                        "👥 Max: {max}"
                    }
                }
            }
            }
        }
    }
}