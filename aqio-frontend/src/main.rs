use dioxus::prelude::*;

mod api;
mod components;
mod lib;

use components::navigation::Route;
use lib::theme::{ThemeProvider, AqioTheme};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    // Initialize panic hook for better error messages in WASM
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        
        ThemeProvider {
            theme: AqioTheme::Auto,
            Router::<Route> {}
        }
    }
}