use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use gloo_storage::{LocalStorage, Storage};
use crate::lib::components::button::{Button, ButtonVariant, ButtonSize};
use crate::lib::components::layout::{Stack, StackDirection, StackAlign, GapSize};
use crate::lib::components::typography::{Heading, HeadingLevel, Paragraph, TextColor, ParagraphSize};

const LOGIN_MODAL_CSS: Asset = asset!("/assets/login-modal.css");

// Storage key for localStorage
const AUTH_STORAGE_KEY: &str = "aqio_auth_state";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MockUser {
    pub id: String,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub user: MockUser,
}

#[derive(Clone)]
pub struct AuthService {
    api_base_url: String,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            api_base_url: "http://127.0.0.1:3000".to_string(),
        }
    }

    pub async fn login(&self, username: &str) -> Result<LoginResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let mut params = HashMap::new();
        params.insert("username", username);

        let response = client
            .get(&format!("{}/auth/login", self.api_base_url))
            .query(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let login_response: LoginResponse = response.json().await?;
            Ok(login_response)
        } else {
            Err(format!("Login failed: {}", response.status()).into())
        }
    }

    pub async fn logout(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/auth/logout", self.api_base_url))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Logout failed: {}", response.status()).into())
        }
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}

// Auth context for the application
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthState {
    pub user: Option<MockUser>,
    pub token: Option<String>,
    pub is_loading: bool,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            user: None,
            token: None,
            is_loading: false,
        }
    }
}

// localStorage helper functions using gloo-storage
fn save_auth_state(state: &AuthState) {
    let _ = LocalStorage::set(AUTH_STORAGE_KEY, state);
}

fn load_auth_state() -> AuthState {
    LocalStorage::get(AUTH_STORAGE_KEY).unwrap_or_default()
}

fn clear_auth_state() {
    LocalStorage::delete(AUTH_STORAGE_KEY);
}

// Global auth context - loads from localStorage on initialization
static AUTH_STATE: GlobalSignal<AuthState> = Signal::global(load_auth_state);

// Auth context hooks following Dioxus patterns
pub fn use_auth_state() -> &'static GlobalSignal<AuthState> {
    &AUTH_STATE
}

pub fn use_auth_login() -> impl Fn(LoginResponse) + Copy {
    move |login_response: LoginResponse| {
        let new_state = AuthState {
            user: Some(login_response.user),
            token: Some(login_response.access_token),
            is_loading: false,
        };
        
        // Save to localStorage first
        save_auth_state(&new_state);
        
        // Then update the global signal
        *AUTH_STATE.write() = new_state;
    }
}

pub fn use_auth_logout() -> impl Fn() + Copy {
    move || {
        // Clear localStorage first
        clear_auth_state();
        
        // Then reset the global signal
        *AUTH_STATE.write() = AuthState::default();
    }
}

pub fn use_auth_loading() -> impl Fn(bool) + Copy {
    move |loading: bool| {
        let mut state = AUTH_STATE.write();
        state.is_loading = loading;
        
        // Save the updated state to localStorage
        save_auth_state(&*state);
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LoginModalProps {
    pub show: Signal<bool>,
    pub on_login: EventHandler<LoginResponse>,
}

#[component]
pub fn LoginModal(mut props: LoginModalProps) -> Element {
    let mut username = use_signal(|| "dev-user".to_string());
    let mut is_loading = use_signal(|| false);
    let mut error_message = use_signal(|| None::<String>);

    let handle_login = move |_| {
        spawn(async move {
            is_loading.set(true);
            error_message.set(None);

            let auth_service = AuthService::new();
            match auth_service.login(&username()).await {
                Ok(login_response) => {
                    props.on_login.call(login_response);
                    props.show.set(false);
                }
                Err(e) => {
                    error_message.set(Some(format!("Login failed: {}", e)));
                }
            }
            is_loading.set(false);
        });
    };

    if !*props.show.read() {
        return rsx! { div { style: "display: none;" } };
    }

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: LOGIN_MODAL_CSS,
        }
        
        div { 
            class: "aqio-modal-overlay",
            onclick: move |_| props.show.set(false),
            
            div { 
                class: "aqio-modal-content",
                onclick: move |e| e.stop_propagation(),
                
                Stack {
                    direction: StackDirection::Horizontal,
                    align: StackAlign::Center,
                    gap: GapSize::Medium,
                    class: "aqio-modal-header",
                    
                    Heading {
                        level: HeadingLevel::H3,
                        "🐟 Login to AQIO"
                    }
                    
                    div { class: "aqio-modal-spacer" }
                    
                    button { 
                        class: "aqio-modal-close",
                        onclick: move |_| props.show.set(false),
                        "✕"
                    }
                }

                if let Some(error) = error_message() {
                    div { 
                        class: "aqio-error-message",
                        
                        Stack {
                            direction: StackDirection::Horizontal,
                            gap: GapSize::Small,
                            align: StackAlign::Start,
                            
                            div { 
                                class: "aqio-error-icon",
                                "⚠️" 
                            }
                            
                            Paragraph {
                                color: TextColor::Error,
                                size: ParagraphSize::Small,
                                "{error}"
                            }
                        }
                    }
                }

                div { 
                    class: "aqio-modal-form",
                    
                    Stack {
                        gap: GapSize::Medium,
                        
                        div {
                            label { 
                                class: "aqio-form-label",
                                r#for: "username",
                                "Select Mock User"
                            }
                            select {
                                id: "username",
                                class: "aqio-form-select",
                                value: "{username}",
                                onchange: move |e| username.set(e.value()),
                                option { value: "dev-user", "Development User (dev@aquanorway.no)" }
                                option { value: "admin-user", "Admin User (admin@aqio.no)" }
                                option { value: "john-doe", "John Doe (john.doe@salmonfarm.no)" }
                                option { value: "jane-smith", "Jane Smith (jane.smith@troutco.no)" }
                            }
                        }

                        div { 
                            class: "aqio-info-message",
                            
                            Stack {
                                direction: StackDirection::Horizontal,
                                gap: GapSize::Small,
                                align: StackAlign::Start,
                                
                                div { 
                                    class: "aqio-info-icon",
                                    "ℹ️" 
                                }
                                
                                Stack {
                                    gap: GapSize::Small,
                                    
                                    Paragraph {
                                        size: ParagraphSize::Small,
                                        "Mock Authentication"
                                    }
                                    Paragraph {
                                        size: ParagraphSize::Small,
                                        color: TextColor::Secondary,
                                        "This is a development-only authentication system. In production, this would integrate with Keycloak."
                                    }
                                }
                            }
                        }

                        Stack {
                            direction: StackDirection::Horizontal,
                            gap: GapSize::Medium,
                            
                            Button {
                                variant: ButtonVariant::Secondary,
                                size: ButtonSize::Medium,
                                class: "flex-1",
                                onclick: move |_| props.show.set(false),
                                "Cancel"
                            }
                            Button {
                                variant: ButtonVariant::Primary,
                                size: ButtonSize::Medium,
                                class: "flex-1",
                                disabled: is_loading(),
                                onclick: handle_login,
                                if is_loading() {
                                    "Logging in..."
                                } else {
                                    "Login"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}