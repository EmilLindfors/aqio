use crate::AppContainer;
use dioxus::prelude::*;

#[component]
pub fn EventsPage(container: AppContainer) -> Element {
    let events = use_resource(move || {
        let svc = container.events.clone();
        async move { svc.list().await }
    });

    rsx! {
        div { class: "container",
            h1 { "Events" }
            match &*events.read() {
                Some(Ok(list)) => rsx! {
                    ul {
                        for ev in list.iter() {
                            li { key: "{ev.id}",
                                strong { "{ev.title}" }
                                span { {format!(" – {} @ {}", ev.start_date, ev.location.as_deref().unwrap_or("TBA"))} }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! { p { style: "color:red;", "Error: {e}" } },
                None => rsx! { p { "Loading..." } },
            }
        }
    }
}
