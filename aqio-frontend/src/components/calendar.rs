use crate::api::{ApiClient, EventResponse};
use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use dioxus::prelude::*;
use std::collections::HashMap;


#[component]
pub fn EventCalendar() -> Element {
    let mut events = use_signal(Vec::<EventResponse>::new);
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| None::<String>);
    let mut current_date = use_signal(|| Local::now().date_naive());
    let mut selected_event = use_signal(|| None::<EventResponse>);
    let mut show_event_modal = use_signal(|| false);

    // Load events when component mounts
    use_effect(move || {
        spawn(async move {
            loading.set(true);
            let api_client = ApiClient::new();
            
            match api_client.list_events().await {
                Ok(event_list) => {
                    // Debug: Log event information
                    #[cfg(target_arch = "wasm32")]
                    for event in &event_list {
                        web_sys::console::log_1(&format!(
                            "📅 Event: {} - Date: {} (naive: {})", 
                            event.title,
                            event.start_date,
                            event.start_date.date_naive()
                        ).into());
                    }
                    
                    events.set(event_list);
                    error.set(None);
                }
                Err(e) => {
                    error.set(Some(format!("Failed to load events: {}", e)));
                }
            }
            loading.set(false);
        });
    });

    let today = Local::now().date_naive();

    let current_year = current_date().year();
    let current_month = current_date().month();
    
    // Debug: Log current calendar view
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&format!(
        "📅 Calendar showing: {}-{:02} (events count: {})", 
        current_year, current_month, events().len()
    ).into());
    
    // Calculate calendar days
    let calendar_days = calculate_calendar_days(current_year, current_month, &events());

    // Navigation handlers
    let prev_month = move |_| {
        let new_date = current_date() - Duration::days(current_date().day() as i64);
        current_date.set(new_date);
    };

    let next_month = move |_| {
        let days_in_month = days_in_month(current_year, current_month);
        let new_date = current_date() + Duration::days((days_in_month - current_date().day() + 1) as i64);
        current_date.set(new_date);
    };

    let go_to_today = move |_| {
        current_date.set(today);
    };

    let month_name = get_month_name(current_month);

    rsx! {
        div { 
            style: "max-width: 1200px; margin: 0 auto; padding: 2rem 1rem;",
            
            // Header
            CalendarHeader { 
                title: "🐟 Aquaculture Events Calendar",
                subtitle: "View all upcoming events in the Norwegian aquaculture industry"
            }

            // Calendar Container
            div { 
                style: "background: white; border-radius: 0.5rem; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1); padding: 1.5rem;",
                
                if loading() {
                    LoadingSpinner {}
                } else if let Some(err) = error() {
                    ErrorDisplay { message: err }
                } else {
                    // Calendar Navigation
                    CalendarNavigation {
                        month_name: month_name,
                        year: current_year,
                        on_prev_month: prev_month,
                        on_next_month: next_month,
                        on_today: go_to_today,
                    }

                    // Weekday headers
                    WeekdayHeaders {}

                    // Calendar Grid
                    CalendarGrid {
                        days: calendar_days,
                        on_event_click: move |event: EventResponse| {
                            selected_event.set(Some(event));
                            show_event_modal.set(true);
                        }
                    }

                    // Separator
                    div { 
                        style: "margin: 1.5rem 0; height: 1px; background: #e5e7eb;",
                    }

                    // Legend
                    EventLegend {}
                }
            }

            // Event Modal
            if show_event_modal() {
                EventModal { 
                    event: selected_event(),
                    on_close: move |_| show_event_modal.set(false)
                }
            }
        }

        // CSS for spinner animation is handled inline in the LoadingSpinner component
    }
}

#[derive(Clone, Debug, PartialEq)]
struct CalendarDay {
    date: NaiveDate,
    is_current_month: bool,
    is_today: bool,
    events: Vec<EventResponse>,
}

#[component]
fn CalendarHeader(title: String, subtitle: String) -> Element {
    rsx! {
        div { 
            style: "margin-bottom: 2rem;",
            h1 { 
                style: "font-size: 1.875rem; font-weight: bold; color: #1f2937; margin-bottom: 0.5rem;",
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
fn LoadingSpinner() -> Element {
    rsx! {
        div { 
            style: "display: flex; justify-content: center; align-items: center; padding: 3rem 0;",
            div { 
                style: "width: 3rem; height: 3rem; border: 2px solid #e5e7eb; border-top-color: #2563eb; border-radius: 50%;",
                "⟳"
            }
            span {
                style: "margin-left: 0.5rem; color: #6b7280;",
                "Loading..."
            }
        }
    }
}

#[component]
fn ErrorDisplay(message: String) -> Element {
    rsx! {
        div { 
            style: "background: #fef2f2; border: 1px solid #fecaca; border-radius: 0.5rem; padding: 1rem;",
            div { style: "color: #991b1b;", "{message}" }
        }
    }
}

#[component]
fn CalendarNavigation(
    month_name: &'static str, 
    year: i32,
    on_prev_month: EventHandler<()>,
    on_next_month: EventHandler<()>,
    on_today: EventHandler<()>,
) -> Element {
    rsx! {
        div { 
            style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;",
            
            button {
                style: "padding: 0.5rem; border: none; background: transparent; border-radius: 0.5rem; cursor: pointer; transition: background-color 0.2s; hover: background-color: #f3f4f6;",
                onclick: move |_| on_prev_month.call(()),
                "←"
            }
            
            div {
                style: "display: flex; align-items: center; gap: 1rem;",
                h2 { 
                    style: "font-size: 1.5rem; font-weight: bold; color: #1f2937;",
                    "{month_name} {year}"
                }
                button {
                    style: "padding: 0.25rem 0.75rem; font-size: 0.875rem; background: #2563eb; color: white; border: none; border-radius: 0.375rem; cursor: pointer; transition: background-color 0.2s;",
                    onclick: move |_| on_today.call(()),
                    "Today"
                }
            }
            
            button {
                style: "padding: 0.5rem; border: none; background: transparent; border-radius: 0.5rem; cursor: pointer; transition: background-color 0.2s; hover: background-color: #f3f4f6;",
                onclick: move |_| on_next_month.call(()),
                "→"
            }
        }
    }
}

#[component]
fn WeekdayHeaders() -> Element {
    rsx! {
        div { 
            style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 1px; background: #e5e7eb; margin-bottom: 1px;",
            div { style: "background: #f9fafb; padding: 0.5rem; text-align: center; font-weight: 600; color: #374151;", "Mon" }
            div { style: "background: #f9fafb; padding: 0.5rem; text-align: center; font-weight: 600; color: #374151;", "Tue" }
            div { style: "background: #f9fafb; padding: 0.5rem; text-align: center; font-weight: 600; color: #374151;", "Wed" }
            div { style: "background: #f9fafb; padding: 0.5rem; text-align: center; font-weight: 600; color: #374151;", "Thu" }
            div { style: "background: #f9fafb; padding: 0.5rem; text-align: center; font-weight: 600; color: #374151;", "Fri" }
            div { style: "background: #f9fafb; padding: 0.5rem; text-align: center; font-weight: 600; color: #374151;", "Sat" }
            div { style: "background: #f9fafb; padding: 0.5rem; text-align: center; font-weight: 600; color: #374151;", "Sun" }
        }
    }
}

#[component]
fn CalendarGrid(
    days: Vec<CalendarDay>, 
    on_event_click: EventHandler<EventResponse>
) -> Element {
    rsx! {
        div { 
            style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 1px; background: #e5e7eb;",
            for day in days {
                CalendarDayCell { 
                    day: day.clone(),
                    on_event_click: on_event_click
                }
            }
        }
    }
}

#[component]
fn CalendarDayCell(day: CalendarDay, on_event_click: EventHandler<EventResponse>) -> Element {
    let day_style = if day.is_today {
        "background: #dbeafe; border: 2px solid #2563eb;"
    } else if day.is_current_month {
        "background: white;"
    } else {
        "background: #f9fafb; color: #9ca3af;"
    };

    rsx! {
        div { 
            style: format!(
                "min-height: 7.5rem; padding: 0.5rem; transition: background-color 0.2s; cursor: pointer; {}",
                day_style
            ),
            
            // Day number
            div { 
                style: "font-weight: 600; font-size: 0.875rem; margin-bottom: 0.25rem;",
                if day.is_today {
                    span { 
                        style: "display: inline-block; background: #2563eb; color: white; border-radius: 50%; width: 1.5rem; height: 1.5rem; text-align: center; line-height: 1.5rem;",
                        "{day.date.day()}"
                    }
                } else {
                    "{day.date.day()}"
                }
            }

            // Events
            div { 
                style: "display: flex; flex-direction: column; gap: 0.25rem;",
                for event in day.events.iter().take(3) {
                    {
                        let event_clone = event.clone();
                        rsx! {
                            div {
                                key: "{event.id}",
                                style: format!(
                                    "font-size: 0.75rem; padding: 0.25rem; border-radius: 0.25rem; cursor: pointer; transition: opacity 0.2s; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; {}",
                                    get_event_color_style(&event.event_type)
                                ),
                                onclick: move |e| {
                                    e.stop_propagation();
                                    on_event_click.call(event_clone.clone());
                                },
                                "{event.title}"
                            }
                        }
                    }
                }
                if day.events.len() > 3 {
                    div { 
                        style: "font-size: 0.75rem; color: #6b7280; font-weight: 600;",
                        "+{day.events.len() - 3} more"
                    }
                }
            }
        }
    }
}

#[component]
fn EventLegend() -> Element {
    rsx! {
        div { 
            style: "display: flex; flex-wrap: wrap; gap: 1rem; font-size: 0.875rem;",
            EventLegendItem { color: "#2563eb", label: "Conference" }
            EventLegendItem { color: "#16a34a", label: "Workshop" }
            EventLegendItem { color: "#7c3aed", label: "Networking" }
            EventLegendItem { color: "#ea580c", label: "Training" }
            EventLegendItem { color: "#6b7280", label: "Other" }
        }
    }
}

#[component]
fn EventLegendItem(color: &'static str, label: &'static str) -> Element {
    rsx! {
        div { 
            style: "display: flex; align-items: center; gap: 0.5rem;",
            div { 
                style: format!("width: 0.75rem; height: 0.75rem; background: {}; border-radius: 50%;", color),
            }
            span { "{label}" }
        }
    }
}

#[component]
fn EventModal(event: Option<EventResponse>, on_close: EventHandler<()>) -> Element {
    let Some(event) = event else {
        return rsx! {};
    };

    let start_date = event.start_date.with_timezone(&Local).format("%B %d, %Y at %H:%M").to_string();
    let end_date = event.end_date.with_timezone(&Local).format("%B %d, %Y at %H:%M").to_string();
    
    let event_type_display = match &event.event_type {
        aqio_core::models::EventType::Conference => "Conference",
        aqio_core::models::EventType::Workshop => "Workshop",
        aqio_core::models::EventType::Networking => "Networking",
        aqio_core::models::EventType::Training => "Training",
        aqio_core::models::EventType::Other(s) => s,
    };

    rsx! {
        div { 
            style: "position: fixed; inset: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; padding: 1rem; z-index: 50;",
            onclick: move |_| on_close.call(()),
            
            div { 
                style: "background: white; border-radius: 0.5rem; max-width: 42rem; width: 100%; max-height: 90vh; overflow-y: auto;",
                onclick: move |e| e.stop_propagation(),
                
                // Header
                div { 
                    style: "padding: 1.5rem; border-bottom: 1px solid #e5e7eb;",
                    div { 
                        style: "display: flex; justify-content: space-between; align-items: flex-start;",
                        h2 { 
                            style: "font-size: 1.5rem; font-weight: bold; color: #1f2937;",
                            "{event.title}"
                        }
                        button {
                            style: "color: #9ca3af; cursor: pointer; border: none; background: none; font-size: 1.25rem;",
                            onclick: move |_| on_close.call(()),
                            "✕"
                        }
                    }
                    div { 
                        style: "margin-top: 0.5rem; display: flex; align-items: center; gap: 0.5rem;",
                        span { 
                            style: format!(
                                "display: inline-block; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.875rem; font-weight: 500; {}",
                                get_event_color_style(&event.event_type)
                            ),
                            "{event_type_display}"
                        }
                    }
                }
                
                // Content
                div { 
                    style: "padding: 1.5rem; display: flex; flex-direction: column; gap: 1rem;",
                    div {
                        h3 { 
                            style: "font-weight: 600; color: #374151; margin-bottom: 0.25rem;", 
                            "Description" 
                        }
                        p {
                            style: "color: #6b7280;",
                            "{event.description}"
                        }
                    }
                    
                    div { 
                        style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;",
                        div {
                            h3 { 
                                style: "font-weight: 600; color: #374151; margin-bottom: 0.25rem;", 
                                "📅 Start" 
                            }
                            p { 
                                style: "color: #6b7280;", 
                                "{start_date}" 
                            }
                        }
                        div {
                            h3 { 
                                style: "font-weight: 600; color: #374151; margin-bottom: 0.25rem;", 
                                "📅 End" 
                            }
                            p { 
                                style: "color: #6b7280;", 
                                "{end_date}" 
                            }
                        }
                        div {
                            h3 { 
                                style: "font-weight: 600; color: #374151; margin-bottom: 0.25rem;", 
                                "📍 Location" 
                            }
                            p { 
                                style: "color: #6b7280;", 
                                "{event.location}" 
                            }
                        }
                        if let Some(max) = event.max_attendees {
                            div {
                                h3 { 
                                    style: "font-weight: 600; color: #374151; margin-bottom: 0.25rem;", 
                                    "👥 Max Attendees" 
                                }
                                p { 
                                    style: "color: #6b7280;", 
                                    "{max}" 
                                }
                            }
                        }
                    }
                    
                    {event.registration_deadline.map(|deadline| {
                        let deadline_formatted = deadline.with_timezone(&Local).format("%B %d, %Y at %H:%M").to_string();
                        rsx! {
                            div {
                                h3 { 
                                    style: "font-weight: 600; color: #374151; margin-bottom: 0.25rem;", 
                                    "⏰ Registration Deadline" 
                                }
                                p { 
                                    style: "color: #6b7280;", 
                                    "{deadline_formatted}"
                                }
                            }
                        }
                    })}
                }
                
                // Footer
                div { 
                    style: "padding: 1.5rem; border-top: 1px solid #e5e7eb; background: #f9fafb;",
                    button {
                        style: "padding: 0.5rem 1rem; background: #2563eb; color: white; border: none; border-radius: 0.375rem; cursor: pointer; transition: background-color 0.2s;",
                        "Register for Event"
                    }
                }
            }
        }
    }
}

fn calculate_calendar_days(year: i32, month: u32, events: &[EventResponse]) -> Vec<CalendarDay> {
    let mut days = Vec::new();
    
    // First day of the month
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let first_weekday = first_day.weekday();
    
    // Calculate start of calendar (may include previous month)
    let days_from_monday = match first_weekday {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    };
    
    let calendar_start = first_day - Duration::days(days_from_monday as i64);
    let today = Local::now().date_naive();
    
    // Create event map by date for quick lookup
    let mut events_by_date: HashMap<NaiveDate, Vec<EventResponse>> = HashMap::new();
    for event in events {
        // Use the date portion directly from the UTC timestamp
        // Most events will have dates that don't cross timezone boundaries significantly
        let event_date = event.start_date.date_naive();
        
        // Debug: Log event date mapping
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!(
            "📅 Mapping event '{}' to date: {}", 
            event.title, 
            event_date
        ).into());
        
        events_by_date.entry(event_date).or_insert_with(Vec::new).push(event.clone());
    }
    
    // Generate 6 weeks of days (42 days total)
    for i in 0..42 {
        let date = calendar_start + Duration::days(i);
        let is_current_month = date.month() == month;
        let is_today = date == today;
        let day_events = events_by_date.get(&date).cloned().unwrap_or_default();
        
        days.push(CalendarDay {
            date,
            is_current_month,
            is_today,
            events: day_events,
        });
    }
    
    days
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        12 => NaiveDate::from_ymd_opt(year + 1, 1, 1),
        _ => NaiveDate::from_ymd_opt(year, month + 1, 1),
    }
    .unwrap()
    .pred_opt()
    .unwrap()
    .day()
}

fn get_month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    }
}



fn get_event_color_style(event_type: &aqio_core::models::EventType) -> &'static str {
    use aqio_core::models::EventType;
    match event_type {
        EventType::Conference => "background: #2563eb; color: white;",
        EventType::Workshop => "background: #16a34a; color: white;",
        EventType::Networking => "background: #7c3aed; color: white;",
        EventType::Training => "background: #ea580c; color: white;",
        EventType::Other(_) => "background: #6b7280; color: white;",
    }
}