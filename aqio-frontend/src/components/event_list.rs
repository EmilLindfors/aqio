use crate::infrastructure::api_client::{ApiClient, EventResponse};
use crate::lib::components::card::{EventCard, EventCardModel};
use crate::lib::components::button::{Button, ButtonVariant};
use crate::lib::components::layout::{Container, Grid, GridColumns, GapSize, Stack, StackAlign, StackJustify, StackDirection};
use crate::lib::components::typography::{Heading, HeadingLevel, Paragraph, ParagraphSize, TextColor};
use crate::components::navigation::Route;
use dioxus::prelude::*;

#[component]
pub fn EventList() -> Element {
    let navigator = use_navigator();
    let mut events = use_resource(move || async move {
        let api_client = ApiClient::new();
        api_client.list_events().await
            .map_err(|e| format!("Failed to load events: {}", e))
    });

    rsx! {
        Container {
            style: "padding: 2rem 0;",
            
            Stack {
                gap: GapSize::XLarge,
                
                // Header section
                Stack {
                    gap: GapSize::Small,
                    
                    Heading {
                        level: HeadingLevel::H1,
                        "🐟 Norwegian Aquaculture Events"
                    }
                    
                    Paragraph {
                        color: TextColor::Secondary,
                        "Discover upcoming events in the Norwegian aquaculture industry"
                    }
                }

                // Content section
                match events() {
                    Some(Ok(event_list)) => rsx! {
                        if event_list.is_empty() {
                            Stack {
                                align: StackAlign::Center,
                                gap: GapSize::Medium,
                                style: "padding: 3rem 0; text-align: center;",
                                
                                div { 
                                    style: "font-size: 4rem; line-height: 1;",
                                    "🐟" 
                                }
                                
                                Heading {
                                    level: HeadingLevel::H3,
                                    "No events found"
                                }
                                
                                Paragraph {
                                    color: TextColor::Secondary,
                                    "Be the first to create an event for the Norwegian aquaculture community!"
                                }
                            }
                        } else {
                            Grid {
                                columns: GridColumns::One,
                                gap: GapSize::Large,
                                
                                for event in event_list {
                                    let model = EventCardModel {
                                        id: event.id,
                                        title: event.title.clone(),
                                        description: event.description.clone(),
                                        location: event.location.clone(),
                                        start_date: event.start_date,
                                        max_attendees: event.max_attendees,
                                        event_type: event.event_type.clone(),
                                    };
                                    EventCard { 
                                        event: model,
                                        on_click: {
                                            let navigator = navigator.clone();
                                            move |ev: EventCardModel| {
                                                navigator.push(Route::EventDetail { id: ev.id.to_string() });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(err)) => rsx! {
                        Stack {
                            gap: GapSize::Small,
                            style: "
                                background-color: var(--aqio-error-light);
                                border: 1px solid var(--aqio-error);
                                border-radius: var(--aqio-radius-lg);
                                padding: var(--aqio-space-4);
                            ",
                            
                            Stack {
                                direction: StackDirection::Horizontal,
                                gap: GapSize::Small,
                                align: StackAlign::Start,
                                
                                div { 
                                    style: "flex-shrink: 0;",
                                    "⚠️" 
                                }
                                
                                Stack {
                                    gap: GapSize::Small,
                                    
                                    Heading {
                                        level: HeadingLevel::H4,
                                        color: TextColor::Error,
                                        "Error loading events"
                                    }
                                    
                                    Paragraph {
                                        size: ParagraphSize::Small,
                                        color: TextColor::Error,
                                        "{err}"
                                    }
                                }
                            }
                        }
                    },
                    None => rsx! {
                        Stack {
                            align: StackAlign::Center,
                            justify: StackJustify::Center,
                            style: "padding: 3rem 0;",
                            
                            div {
                                style: "
                                    width: 3rem;
                                    height: 3rem;
                                    border: 2px solid transparent;
                                    border-top-color: var(--aqio-blue-primary);
                                    border-radius: 50%;
                                    animation: spin 1s linear infinite;
                                ",
                            }
                            
                            style {
                                "
                                @keyframes spin {{
                                    from {{ transform: rotate(0deg); }}
                                    to {{ transform: rotate(360deg); }}
                                }}
                                "
                            }
                        }
                    }
                }
                
                // Refresh button
                Stack {
                    align: StackAlign::Center,
                    style: "margin-top: 2rem;",
                    
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| events.restart(),
                        "🔄 Refresh Events"
                    }
                }
            }
        }
    }
}