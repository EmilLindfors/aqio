use crate::api::{ApiClient, EventResponse};
use crate::components::navigation::Route;
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, Weekday, Utc, Timelike};
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use std::collections::HashMap;
use aqio_core::models::EventType;

#[derive(Clone, Debug, PartialEq)]
pub enum CalendarView {
    Month,
    Week,
    Day,
}

#[derive(Clone, Debug, PartialEq)]
struct CalendarDay {
    date: NaiveDate,
    is_current_month: bool,
    is_today: bool,
    events: Vec<EventResponse>,
}

#[component]
pub fn EnhancedEventCalendar() -> Element {
    let mut events = use_resource(move || async move {
        let api_client = ApiClient::new();
        api_client.list_events().await
            .map_err(|e| format!("Failed to load events: {}", e))
    });
    
    let mut filtered_events = use_signal(Vec::<EventResponse>::new);
    let mut current_date = use_signal(|| Local::now().date_naive());
    let mut selected_event = use_signal(|| None::<EventResponse>);
    let mut show_event_modal = use_signal(|| false);
    let mut show_create_modal = use_signal(|| false);
    let mut selected_date = use_signal(|| None::<NaiveDate>);
    
    // View controls
    let mut calendar_view = use_signal(|| CalendarView::Month);
    let mut search_query = use_signal(String::new);
    let mut selected_filters = use_signal(|| vec![
        EventType::Conference,
        EventType::Workshop,
        EventType::Networking,
        EventType::Training,
        EventType::Other("".to_string()),
    ]);

    // Filter events based on search and type filters
    use_effect(move || {
        if let Some(Ok(event_list)) = events() {
            let query = search_query().to_lowercase();
            let filters = selected_filters();
            
            let filtered: Vec<EventResponse> = event_list
                .into_iter()
                .filter(|event| {
                    let matches_search = query.is_empty() || 
                        event.title.to_lowercase().contains(&query) ||
                        event.description.to_lowercase().contains(&query) ||
                        event.location.to_lowercase().contains(&query);
                    
                    let matches_filter = filters.iter().any(|f| {
                        match (&event.event_type, f) {
                            (EventType::Conference, EventType::Conference) => true,
                            (EventType::Workshop, EventType::Workshop) => true,
                            (EventType::Networking, EventType::Networking) => true,
                            (EventType::Training, EventType::Training) => true,
                            (EventType::Other(_), EventType::Other(_)) => true,
                            _ => false,
                        }
                    });
                    
                    matches_search && matches_filter
                })
                .collect();
            
            filtered_events.set(filtered);
        }
    });

    let today = Local::now().date_naive();
    let current_year = current_date().year();
    let current_month = current_date().month();
    
    // Navigation handlers
    let prev_period = move |_| {
        match calendar_view() {
            CalendarView::Month => {
                let new_date = current_date() - Duration::days(current_date().day() as i64);
                current_date.set(new_date);
            }
            CalendarView::Week => {
                current_date.set(current_date() - Duration::weeks(1));
            }
            CalendarView::Day => {
                current_date.set(current_date() - Duration::days(1));
            }
        }
    };

    let next_period = move |_| {
        match calendar_view() {
            CalendarView::Month => {
                let days_in_month = days_in_month(current_year, current_month);
                let new_date = current_date() + Duration::days((days_in_month - current_date().day() + 1) as i64);
                current_date.set(new_date);
            }
            CalendarView::Week => {
                current_date.set(current_date() + Duration::weeks(1));
            }
            CalendarView::Day => {
                current_date.set(current_date() + Duration::days(1));
            }
        }
    };

    let go_to_today = move |_| {
        current_date.set(today);
    };

    let month_name = get_month_name(current_month);

    rsx! {
        div { class: "container mx-auto px-4 py-8",
            // Header
            div { class: "mb-6",
                h1 { class: "text-3xl font-bold text-gray-900 mb-2",
                    "🐟 Aquaculture Events Calendar"
                }
                p { class: "text-gray-600",
                    "Manage and view all upcoming events in the Norwegian aquaculture industry"
                }
            }

            // Controls Bar
            div { class: "bg-white rounded-lg shadow-lg p-4 mb-4",
                div { class: "flex flex-wrap items-center justify-between gap-4",
                    // Search Bar
                    div { class: "flex-1 min-w-[200px] max-w-md",
                        input {
                            r#type: "text",
                            placeholder: "Search events...",
                            class: "w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                            value: "{search_query}",
                            oninput: move |e| search_query.set(e.value())
                        }
                    }
                    
                    // View Toggle
                    div { class: "flex bg-gray-100 rounded-lg p-1",
                        button {
                            class: if calendar_view() == CalendarView::Month { 
                                "px-3 py-1 bg-white rounded shadow-sm font-medium" 
                            } else { 
                                "px-3 py-1 text-gray-600 hover:text-gray-900" 
                            },
                            onclick: move |_| calendar_view.set(CalendarView::Month),
                            "Month"
                        }
                        button {
                            class: if calendar_view() == CalendarView::Week { 
                                "px-3 py-1 bg-white rounded shadow-sm font-medium" 
                            } else { 
                                "px-3 py-1 text-gray-600 hover:text-gray-900" 
                            },
                            onclick: move |_| calendar_view.set(CalendarView::Week),
                            "Week"
                        }
                        button {
                            class: if calendar_view() == CalendarView::Day { 
                                "px-3 py-1 bg-white rounded shadow-sm font-medium" 
                            } else { 
                                "px-3 py-1 text-gray-600 hover:text-gray-900" 
                            },
                            onclick: move |_| calendar_view.set(CalendarView::Day),
                            "Day"
                        }
                    }
                    
                    // Add Event Button
                    button {
                        class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors",
                        onclick: move |_| {
                            selected_date.set(Some(current_date()));
                            show_create_modal.set(true);
                        },
                        "➕ Add Event"
                    }
                }
                
                // Filter Pills
                div { class: "flex flex-wrap gap-2 mt-4",
                    span { class: "text-sm font-medium text-gray-700 mr-2", "Filter by type:" }
                    for event_type in [
                        EventType::Conference,
                        EventType::Workshop,
                        EventType::Networking,
                        EventType::Training,
                        EventType::Other("".to_string()),
                    ] {
                        {
                            let type_name = match &event_type {
                                EventType::Conference => "Conference",
                                EventType::Workshop => "Workshop",
                                EventType::Networking => "Networking",
                                EventType::Training => "Training",
                                EventType::Other(_) => "Other",
                            };
                            let is_selected = selected_filters().iter().any(|f| {
                                match (&event_type, f) {
                                    (EventType::Conference, EventType::Conference) => true,
                                    (EventType::Workshop, EventType::Workshop) => true,
                                    (EventType::Networking, EventType::Networking) => true,
                                    (EventType::Training, EventType::Training) => true,
                                    (EventType::Other(_), EventType::Other(_)) => true,
                                    _ => false,
                                }
                            });
                            let event_type_clone = event_type.clone();
                            
                            rsx! {
                                button {
                                    class: if is_selected {
                                        format!("px-3 py-1 rounded-full text-sm font-medium transition-colors {}", 
                                            get_event_color_class(&event_type))
                                    } else {
                                        "px-3 py-1 rounded-full text-sm font-medium bg-gray-200 text-gray-600 hover:bg-gray-300 transition-colors".to_string()
                                    },
                                    onclick: move |_| {
                                        let mut filters = selected_filters();
                                        if is_selected {
                                            filters.retain(|f| !matches!((f, &event_type_clone), 
                                                (EventType::Conference, EventType::Conference) |
                                                (EventType::Workshop, EventType::Workshop) |
                                                (EventType::Networking, EventType::Networking) |
                                                (EventType::Training, EventType::Training) |
                                                (EventType::Other(_), EventType::Other(_))
                                            ));
                                        } else {
                                            filters.push(event_type_clone.clone());
                                        }
                                        selected_filters.set(filters);
                                    },
                                    "{type_name}"
                                }
                            }
                        }
                    }
                }
            }

            // Calendar Container
            div { class: "bg-white rounded-lg shadow-lg p-6",
                // Calendar Navigation
                div { class: "flex items-center justify-between mb-6",
                    button {
                        class: "p-2 hover:bg-gray-100 rounded-lg transition-colors",
                        onclick: prev_period,
                        "←"
                    }
                    
                    div { class: "flex items-center space-x-4",
                        h2 { class: "text-2xl font-bold text-gray-800",
                            {
                                match calendar_view() {
                                    CalendarView::Month => format!("{} {}", month_name, current_year),
                                    CalendarView::Week => {
                                        let week_start = current_date() - Duration::days(current_date().weekday().num_days_from_monday() as i64);
                                        let week_end = week_start + Duration::days(6);
                                        format!("{} - {}", 
                                            week_start.format("%b %d"),
                                            week_end.format("%b %d, %Y"))
                                    },
                                    CalendarView::Day => current_date().format("%B %d, %Y").to_string(),
                                }
                            }
                        }
                        button {
                            class: "px-3 py-1 text-sm bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors",
                            onclick: go_to_today,
                            "Today"
                        }
                    }
                    
                    button {
                        class: "p-2 hover:bg-gray-100 rounded-lg transition-colors",
                        onclick: next_period,
                        "→"
                    }
                }

                match events() {
                    Some(Ok(_)) => {
                        match calendar_view() {
                            CalendarView::Month => rsx! {
                                MonthView { 
                                    current_date: current_date(),
                                    events: filtered_events(),
                                    on_event_click: move |event: EventResponse| {
                                        selected_event.set(Some(event));
                                        show_event_modal.set(true);
                                    },
                                    on_date_click: move |date: NaiveDate| {
                                        selected_date.set(Some(date));
                                        show_create_modal.set(true);
                                    }
                                }
                            },
                            CalendarView::Week => rsx! {
                                WeekView {
                                    current_date: current_date(),
                                    events: filtered_events(),
                                    on_event_click: move |event: EventResponse| {
                                        selected_event.set(Some(event));
                                        show_event_modal.set(true);
                                    },
                                    on_slot_click: move |datetime: NaiveDateTime| {
                                        selected_date.set(Some(datetime.date()));
                                        show_create_modal.set(true);
                                    }
                                }
                            },
                            CalendarView::Day => rsx! {
                                DayView {
                                    current_date: current_date(),
                                    events: filtered_events(),
                                    on_event_click: move |event: EventResponse| {
                                        selected_event.set(Some(event));
                                        show_event_modal.set(true);
                                    },
                                    on_slot_click: move |datetime: NaiveDateTime| {
                                        selected_date.set(Some(datetime.date()));
                                        show_create_modal.set(true);
                                    }
                                }
                            },
                        }
                    },
                    Some(Err(err)) => rsx! {
                        div { class: "bg-red-50 border border-red-200 rounded-lg p-4",
                            div { class: "text-red-800", "{err}" }
                        }
                    },
                    None => rsx! {
                        div { class: "flex justify-center items-center py-12",
                            div { class: "animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600" }
                        }
                    }
                }

                // Stats Bar
                div { class: "mt-6 grid grid-cols-2 md:grid-cols-4 gap-4",
                    div { class: "bg-gray-50 rounded-lg p-3",
                        div { class: "text-2xl font-bold text-gray-900", {format!("{}", filtered_events().len())} }
                        div { class: "text-sm text-gray-600", "Total Events" }
                    }
                    div { class: "bg-blue-50 rounded-lg p-3",
                        div { class: "text-2xl font-bold text-blue-900", 
                            {format!("{}", filtered_events().iter().filter(|e| e.start_date.date_naive() >= today).count())}
                        }
                        div { class: "text-sm text-blue-600", "Upcoming" }
                    }
                    div { class: "bg-green-50 rounded-lg p-3",
                        div { class: "text-2xl font-bold text-green-900",
                            {format!("{}", filtered_events().iter().filter(|e| {
                                let event_date = e.start_date.date_naive();
                                event_date >= today && event_date < today + Duration::days(7)
                            }).count())}
                        }
                        div { class: "text-sm text-green-600", "This Week" }
                    }
                    div { class: "bg-purple-50 rounded-lg p-3",
                        div { class: "text-2xl font-bold text-purple-900",
                            {format!("{}", filtered_events().iter().filter(|e| {
                                let event_date = e.start_date.date_naive();
                                event_date.month() == current_month && event_date.year() == current_year
                            }).count())}
                        }
                        div { class: "text-sm text-purple-600", "This Month" }
                    }
                }
            }

            // Event Modal
            if show_event_modal() {
                EventDetailModal { 
                    event: selected_event(),
                    on_close: move |_| show_event_modal.set(false)
                }
            }

            // Quick Create Modal
            if show_create_modal() {
                QuickCreateModal {
                    selected_date: selected_date(),
                    on_close: move |_| {
                        show_create_modal.set(false);
                        selected_date.set(None);
                    },
                    on_create: move |_| {
                        show_create_modal.set(false);
                        // Refresh events
                        events.restart();
                    }
                }
            }
        }
    }
}

#[component]
fn MonthView(
    current_date: NaiveDate,
    events: Vec<EventResponse>,
    on_event_click: EventHandler<EventResponse>,
    on_date_click: EventHandler<NaiveDate>
) -> Element {
    let calendar_days = calculate_calendar_days(
        current_date.year(), 
        current_date.month(), 
        &events
    );

    rsx! {
        div {
            // Weekday headers
            div { class: "grid grid-cols-7 gap-px bg-gray-200 mb-px",
                for day in ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"] {
                    div { class: "bg-gray-50 p-2 text-center font-semibold text-gray-700",
                        "{day}"
                    }
                }
            }

            // Calendar Grid
            div { class: "grid grid-cols-7 gap-px bg-gray-200",
                for day in calendar_days {
                    CalendarDayCell { 
                        day: day.clone(),
                        on_event_click: on_event_click,
                        on_date_click: on_date_click
                    }
                }
            }
        }
    }
}

#[component]
fn WeekView(
    current_date: NaiveDate,
    events: Vec<EventResponse>,
    on_event_click: EventHandler<EventResponse>,
    on_slot_click: EventHandler<NaiveDateTime>
) -> Element {
    let week_start = current_date - Duration::days(current_date.weekday().num_days_from_monday() as i64);
    let hours = (8..20).collect::<Vec<_>>();
    
    rsx! {
        div { class: "overflow-x-auto",
            div { class: "min-w-[800px]",
                // Header with days
                div { class: "grid grid-cols-8 border-b",
                    div { class: "p-2 text-center font-semibold text-gray-600 border-r", "Time" }
                    for i in 0..7 {
                        {
                            let day = week_start + Duration::days(i);
                            let is_today = day == Local::now().date_naive();
                            rsx! {
                                div { 
                                    class: if is_today { 
                                        "p-2 text-center font-semibold bg-blue-50 border-r" 
                                    } else { 
                                        "p-2 text-center font-semibold border-r" 
                                    },
                                    div { class: "text-sm text-gray-600", 
                                        {["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"][i as usize]}
                                    }
                                    div { class: if is_today { "text-blue-600 font-bold" } else { "" },
                                        "{day.day()}"
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Time slots
                for hour in hours {
                    div { class: "grid grid-cols-8 border-b hover:bg-gray-50",
                        div { class: "p-2 text-sm text-gray-600 border-r",
                            {format!("{:02}:00", hour)}
                        }
                        for i in 0..7 {
                            {
                                let slot_date = week_start + Duration::days(i);
                                let slot_time = NaiveDateTime::new(
                                    slot_date,
                                    chrono::NaiveTime::from_hms_opt(hour, 0, 0).unwrap()
                                );
                                let day_events = events.iter()
                                    .filter(|e| {
                                        let event_start = e.start_date.with_timezone(&Local).naive_local();
                                        event_start.date() == slot_date && 
                                        event_start.hour() == hour
                                    })
                                    .collect::<Vec<_>>();
                                
                                rsx! {
                                    div { 
                                        class: "p-1 min-h-[60px] border-r cursor-pointer hover:bg-blue-50",
                                        onclick: move |_| on_slot_click.call(slot_time),
                                        for event in day_events {
                                            {
                                                let event_clone = (*event).clone();
                                                rsx! {
                                                    div {
                                                        class: "text-xs p-1 mb-1 rounded cursor-pointer hover:opacity-80 {get_event_color_class(&event.event_type)}",
                                                        onclick: move |e| {
                                                            e.stop_propagation();
                                                            on_event_click.call(event_clone.clone());
                                                        },
                                                        "{event.title}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DayView(
    current_date: NaiveDate,
    events: Vec<EventResponse>,
    on_event_click: EventHandler<EventResponse>,
    on_slot_click: EventHandler<NaiveDateTime>
) -> Element {
    let hours = (0..24).collect::<Vec<_>>();
    let day_events = events.iter()
        .filter(|e| e.start_date.date_naive() == current_date)
        .collect::<Vec<_>>();
    
    rsx! {
        div { class: "max-w-4xl mx-auto",
            // All day events
            if day_events.iter().any(|e| {
                let duration = e.end_date - e.start_date;
                duration.num_hours() >= 24
            }) {
                div { class: "mb-4 p-3 bg-gray-50 rounded-lg",
                    div { class: "font-semibold text-gray-700 mb-2", "All Day Events" }
                    div { class: "space-y-2",
                        for event in day_events.iter().filter(|e| {
                            let duration = e.end_date - e.start_date;
                            duration.num_hours() >= 24
                        }) {
                            {
                                let event_clone = (*event).clone();
                                rsx! {
                                    div {
                                        class: "p-2 rounded cursor-pointer {get_event_color_class(&event.event_type)}",
                                        onclick: move |_| on_event_click.call(event_clone.clone()),
                                        div { class: "font-medium", "{event.title}" }
                                        div { class: "text-sm opacity-90", "{event.location}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Hourly schedule
            div { class: "border rounded-lg",
                for hour in hours {
                    {
                        let slot_time = NaiveDateTime::new(
                            current_date,
                            chrono::NaiveTime::from_hms_opt(hour, 0, 0).unwrap()
                        );
                        let hour_events = day_events.iter()
                            .filter(|e| {
                                let event_start = e.start_date.with_timezone(&Local).naive_local();
                                event_start.hour() == hour
                            })
                            .collect::<Vec<_>>();
                        
                        rsx! {
                            div { 
                                class: "flex border-b hover:bg-gray-50",
                                div { class: "w-20 p-3 text-sm text-gray-600 border-r",
                                    {format!("{:02}:00", hour)}
                                }
                                div { 
                                    class: "flex-1 p-2 min-h-[60px] cursor-pointer",
                                    onclick: move |_| on_slot_click.call(slot_time),
                                    for event in hour_events {
                                        {
                                            let event_clone = (*event).clone();
                                            rsx! {
                                                div {
                                                    class: "mb-2 p-2 rounded cursor-pointer {get_event_color_class(&event.event_type)}",
                                                    onclick: move |e| {
                                                        e.stop_propagation();
                                                        on_event_click.call(event_clone.clone());
                                                    },
                                                    div { class: "font-medium", "{event.title}" }
                                                    div { class: "text-sm opacity-90",
                                                        {format!("{} - {}", 
                                                            event.start_date.with_timezone(&Local).format("%H:%M"),
                                                            event.end_date.with_timezone(&Local).format("%H:%M"))}
                                                    }
                                                    div { class: "text-sm opacity-90", "📍 {event.location}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CalendarDayCell(
    day: CalendarDay, 
    on_event_click: EventHandler<EventResponse>,
    on_date_click: EventHandler<NaiveDate>
) -> Element {
    let day_classes = if day.is_today {
        "bg-blue-50 border-2 border-blue-500"
    } else if day.is_current_month {
        "bg-white hover:bg-gray-50"
    } else {
        "bg-gray-50 text-gray-400"
    };

    rsx! {
        div { 
            class: "min-h-[120px] p-2 {day_classes} transition-colors cursor-pointer",
            onclick: move |_| on_date_click.call(day.date),
            
            // Day number
            div { class: "font-semibold text-sm mb-1",
                if day.is_today {
                    span { class: "inline-block bg-blue-500 text-white rounded-full w-6 h-6 text-center",
                        "{day.date.day()}"
                    }
                } else {
                    "{day.date.day()}"
                }
            }

            // Events
            div { class: "space-y-1",
                for event in day.events.iter().take(3) {
                    {
                        let event_clone = event.clone();
                        rsx! {
                            div {
                                key: "{event.id}",
                                class: "text-xs p-1 rounded cursor-pointer hover:opacity-80 truncate {get_event_color_class(&event.event_type)}",
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
                    div { class: "text-xs text-gray-500 font-semibold",
                        "+{day.events.len() - 3} more"
                    }
                }
            }
        }
    }
}

#[component]
fn EventDetailModal(event: Option<EventResponse>, on_close: EventHandler<()>) -> Element {
    let Some(event) = event else {
        return rsx! {};
    };

    let _navigator = use_navigator();
    let start_date = event.start_date.with_timezone(&Local).format("%B %d, %Y at %H:%M").to_string();
    let end_date = event.end_date.with_timezone(&Local).format("%B %d, %Y at %H:%M").to_string();
    
    let event_type_display = match &event.event_type {
        EventType::Conference => "Conference",
        EventType::Workshop => "Workshop",
        EventType::Networking => "Networking",
        EventType::Training => "Training",
        EventType::Other(s) => s,
    };

    rsx! {
        div { 
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50",
            onclick: move |_| on_close.call(()),
            
            div { 
                class: "bg-white rounded-lg max-w-2xl w-full max-h-[90vh] overflow-y-auto",
                onclick: move |e| e.stop_propagation(),
                
                // Header
                div { class: "p-6 border-b",
                    div { class: "flex justify-between items-start",
                        h2 { class: "text-2xl font-bold text-gray-900",
                            "{event.title}"
                        }
                        button {
                            class: "text-gray-400 hover:text-gray-600",
                            onclick: move |_| on_close.call(()),
                            "✕"
                        }
                    }
                    div { class: "mt-2 flex items-center gap-2",
                        span { class: "inline-block px-3 py-1 rounded-full text-sm font-medium {get_event_color_class(&event.event_type)}",
                            "{event_type_display}"
                        }
                    }
                }
                
                // Content
                div { class: "p-6 space-y-4",
                    div {
                        h3 { class: "font-semibold text-gray-700 mb-1", "Description" }
                        p { class: "text-gray-600", "{event.description}" }
                    }
                    
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        div {
                            h3 { class: "font-semibold text-gray-700 mb-1", "📅 Start" }
                            p { class: "text-gray-600", "{start_date}" }
                        }
                        div {
                            h3 { class: "font-semibold text-gray-700 mb-1", "📅 End" }
                            p { class: "text-gray-600", "{end_date}" }
                        }
                        div {
                            h3 { class: "font-semibold text-gray-700 mb-1", "📍 Location" }
                            p { class: "text-gray-600", "{event.location}" }
                        }
                        if let Some(max) = event.max_attendees {
                            div {
                                h3 { class: "font-semibold text-gray-700 mb-1", "👥 Max Attendees" }
                                p { class: "text-gray-600", "{max}" }
                            }
                        }
                    }
                    
                    {event.registration_deadline.map(|deadline| {
                        let deadline_formatted = deadline.with_timezone(&Local).format("%B %d, %Y at %H:%M").to_string();
                        rsx! {
                            div {
                                h3 { class: "font-semibold text-gray-700 mb-1", "⏰ Registration Deadline" }
                                p { class: "text-gray-600", 
                                    "{deadline_formatted}"
                                }
                            }
                        }
                    })}
                }
                
                // Footer
                div { class: "p-6 border-t bg-gray-50 flex gap-3",
                    button {
                        class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors",
                        "Register for Event"
                    }
                    button {
                        class: "px-4 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300 transition-colors",
                        onclick: move |_| {
                            // Export to calendar
                            export_event_to_ical(&event);
                        },
                        "📥 Export to Calendar"
                    }
                }
            }
        }
    }
}

#[component]
fn QuickCreateModal(
    selected_date: Option<NaiveDate>,
    on_close: EventHandler<()>,
    on_create: EventHandler<()>
) -> Element {
    let _navigator = use_navigator();
    let _date_str = selected_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default();
    
    rsx! {
        div { 
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50",
            onclick: move |_| on_close.call(()),
            
            div { 
                class: "bg-white rounded-lg max-w-md w-full",
                onclick: move |e| e.stop_propagation(),
                
                div { class: "p-6 border-b",
                    h2 { class: "text-xl font-bold text-gray-900",
                        "Quick Event Creation"
                    }
                    if let Some(date) = selected_date {
                        p { class: "text-sm text-gray-600 mt-1",
                            "Creating event for {date.format(\"%B %d, %Y\")}"
                        }
                    }
                }
                
                div { class: "p-6",
                    p { class: "text-gray-600 mb-4",
                        "To create a detailed event with all options, please use the full event creation form."
                    }
                    
                    div { class: "flex gap-3",
                        button {
                            class: "flex-1 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors",
                            onclick: move |_| {
                                on_close.call(());
                                navigator().push(Route::CreateEvent {});
                            },
                            "Go to Event Form"
                        }
                        button {
                            class: "px-4 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300 transition-colors",
                            onclick: move |_| on_close.call(()),
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}

// Helper functions
fn calculate_calendar_days(year: i32, month: u32, events: &[EventResponse]) -> Vec<CalendarDay> {
    let mut days = Vec::new();
    
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let first_weekday = first_day.weekday();
    
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
    
    let mut events_by_date: HashMap<NaiveDate, Vec<EventResponse>> = HashMap::new();
    for event in events {
        let event_date = event.start_date.with_timezone(&Local).date_naive();
        events_by_date.entry(event_date).or_insert_with(Vec::new).push(event.clone());
    }
    
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

fn get_event_color_class(event_type: &EventType) -> &'static str {
    match event_type {
        EventType::Conference => "bg-blue-500 text-white",
        EventType::Workshop => "bg-green-500 text-white",
        EventType::Networking => "bg-purple-500 text-white",
        EventType::Training => "bg-orange-500 text-white",
        EventType::Other(_) => "bg-gray-500 text-white",
    }
}

fn export_event_to_ical(event: &EventResponse) {
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
        Utc::now().format("%Y%m%dT%H%M%SZ"),
        event.start_date.format("%Y%m%dT%H%M%SZ"),
        event.end_date.format("%Y%m%dT%H%M%SZ"),
        event.title,
        event.description,
        event.location
    );
    
    // In a real implementation, this would trigger a download
    // For now, we'll just log it
    log::info!("Export event to iCal: {}", event.title);
}