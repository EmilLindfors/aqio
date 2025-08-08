use dioxus::prelude::*;

use crate::AppContainer;

use super::pages::events::EventsPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/events")]
    Events {},
}

#[component]
pub fn Root() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "container",
            h1 { "AQIO" }
            Link { to: Route::Events {}, "Go to Events" }
        }
    }
}

#[component]
pub fn Events() -> Element {
    // Get DI container and pass into page
    let container = use_context::<AppContainer>();
    rsx! { EventsPage { container } }
}
