use dioxus::prelude::*;
use std::sync::Arc;

mod application;
mod domain;
mod infrastructure;
mod lib;
mod presentation;

use application::services::EventService;
use infrastructure::{api_client::ApiClient, event_repository::ApiEventRepository};
use lib::theme::{AqioTheme, ThemeProvider};

#[derive(Clone)]
pub struct AppContainer {
    pub events: EventService,
}

impl PartialEq for AppContainer {
    fn eq(&self, other: &Self) -> bool {
        // Services are cheap to clone and stateless wrappers over Arc repos; compare addr
        std::ptr::eq(self as *const _, other as *const _)
    }
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    // Initialize panic hook for better error messages in WASM
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    // Composition root: wire ports -> services -> UI
    let api = ApiClient::new();
    let repo = Arc::new(ApiEventRepository::new(api));
    let events = EventService::new(repo);
    let container = AppContainer { events };

    // Provide DI container to the component tree
    use_context_provider(|| container.clone());

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        ThemeProvider { theme: AqioTheme::Auto,
            presentation::routes::Root {}
        }
    }
}
