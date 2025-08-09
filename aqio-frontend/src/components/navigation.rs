use dioxus::prelude::*;

use crate::components::event_list::EventList;
use crate::components::event_form::EventForm;
use crate::components::event_detail::EventDetailPage;
use crate::components::calendar::EventCalendar;
use crate::components::auth::{LoginModal, use_auth_state, use_auth_login, use_auth_logout};
use crate::lib::components::navigation::{
    Navbar, NavbarBrand, NavbarSubtitle, NavbarLinks, NavLink, NavButton, Footer,
    Stack, StackDirection, StackAlign, StackGap,
    NavButtonVariant, NavButtonSize
};

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/calendar")]
    Calendar {},
    #[route("/events/:id")]
    EventDetail { id: String },
    #[route("/create")]
    CreateEvent {},
}

#[component]
fn Layout() -> Element {
    let mut show_login = use_signal(|| false);
    let handle_login = use_auth_login();
    let handle_logout = use_auth_logout();

    rsx! {
        div { 
            id: "main",
            style: "
                min-height: 100vh;
                background-color: var(--aqio-surface);
                display: flex;
                flex-direction: column;
            ",
            
            Navigation { 
                on_login: move |_| show_login.set(true),
                on_logout: move |_| handle_logout()
            }
            
            main { 
                class: "route-container",
                style: "flex: 1; padding: 1rem 0;",
                Outlet::<Route> {}
            }
            
            Footer {
                Stack {
                    gap: StackGap::Small,
                    
                    div {
                        class: "aqio-footer-text",
                        "🐟 AQIO - Connecting the Norwegian Aquaculture Industry"
                    }
                    
                    div {
                        class: "aqio-footer-subtext",
                        "Built with Rust, Dioxus, and Axum"
                    }
                }
            }
            
            LoginModal {
                show: show_login,
                on_login: move |response| handle_login(response)
            }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        EventList {}
    }
}

#[component]
fn Calendar() -> Element {
    rsx! {
        EventCalendar {}
    }
}

#[component]
fn EventDetail(id: String) -> Element {
    rsx! {
        EventDetailPage { event_id: id }
    }
}

#[component]
fn CreateEvent() -> Element {
    rsx! {
        EventForm {}
    }
}

#[component]
pub fn Navigation(
    on_login: EventHandler<()>,
    on_logout: EventHandler<()>
) -> Element {
    // Access global auth state directly like in Dioxus examples
    rsx! {
        Navbar {
            // Left side - Brand
            Stack {
                direction: StackDirection::Horizontal,
                align: StackAlign::Center,
                gap: StackGap::Medium,
                
                NavbarBrand {
                    "🐟 AQIO"
                }
                
                NavbarSubtitle {
                    "Norwegian Aquaculture Events"
                }
            }
            
            // Right side - Navigation Links
            NavbarLinks {
                NavLink {
                    to: format!("{}", Route::Home {}),
                    "Events"
                }
                
                NavLink {
                    to: format!("{}", Route::Calendar {}),
                    "📅 Calendar"
                }
                
                if use_auth_state()().user.is_some() {
                    NavLink {
                        to: format!("{}", Route::CreateEvent {}),
                        "Create Event"
                    }
                }

                if let Some(user) = &use_auth_state()().user {
                    Stack {
                        direction: StackDirection::Horizontal,
                        align: StackAlign::Center,
                        gap: StackGap::Small,
                        
                        div {
                            class: "aqio-navbar-user-name",
                            "👋 {user.name}"
                        }
                        
                        NavButton {
                            variant: NavButtonVariant::Primary,
                            size: NavButtonSize::Medium,
                            onclick: move |_| on_logout.call(()),
                            "Logout"
                        }
                    }
                } else {
                    NavButton {
                        variant: NavButtonVariant::Primary,
                        size: NavButtonSize::Medium,
                        onclick: move |_| on_login.call(()),
                        "Login"
                    }
                }
            }
        }
    }
}