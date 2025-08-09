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
        header { class: "aqio-header",
            div { class: "container aqio-header-inner",
                a { class: "aqio-brand", href: "/", "🐟 AQIO" }
                nav { class: "aqio-nav",
                    Link { class: "aqio-nav-link", to: Route::Home {}, "Events" }
                }
            }
        }
        main { class: "container route-container",
            Router::<Route> {}
        }
        footer { class: "aqio-footer",
            div { class: "container",
                span { class: "aqio-footer-text", "Built with Rust, Dioxus, and Axum" }
            }
        }
    }
}

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "home",
            // Use our component lib primitives to compose the hero section
            crate::lib::components::layout::Stack { gap: crate::lib::components::layout::GapSize::Large,
                crate::lib::components::typography::Heading { level: crate::lib::components::typography::HeadingLevel::H1,
                    "AQIO"
                }
                crate::lib::components::typography::Paragraph { 
                    "Welcome. Browse upcoming events."
                }
                Link { to: Route::Events {}, "Go to Events" }
            }
        }
    }
}

#[component]
pub fn Events() -> Element {
    // Get DI container and pass into page
    let container = use_context::<AppContainer>();
    rsx! { EventsPage { container } }
}
