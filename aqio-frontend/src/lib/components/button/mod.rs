use dioxus::prelude::*;

// Import the CSS for our button components
const AQIO_BUTTON_CSS: Asset = asset!("/assets/aqio-buttons.css");

/// Button variant styles
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Ghost,
    Link,
}

impl ButtonVariant {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary", 
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Ghost => "ghost",
            Self::Link => "link",
        }
    }
}

/// Button size variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

/// Props for the Button component
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Visual style variant of the button
    #[props(default)]
    pub variant: ButtonVariant,
    
    /// Size of the button
    #[props(default)]
    pub size: ButtonSize,
    
    /// Whether the button is disabled
    #[props(default)]
    pub disabled: bool,
    
    /// Whether the button is in loading state
    #[props(default)]
    pub loading: bool,

    /// Whether the button should take full width
    #[props(default)]
    pub full_width: bool,

    /// Whether this is an icon-only button (square aspect ratio)
    #[props(default)]
    pub icon_only: bool,

    /// Whether the button should be full width on mobile
    #[props(default)]
    pub responsive: bool,
    
    /// Click event handler
    #[props(default)]
    pub onclick: EventHandler<MouseEvent>,

    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    
    /// Button content (text, icons, etc.)
    pub children: Element,
}

/// # Button
/// 
/// A flexible button component with multiple variants, sizes, and states.
/// Uses CSS classes and data attributes for styling following the preview pattern.
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let class = format!("aqio-button {}", props.class.unwrap_or_default());
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: AQIO_BUTTON_CSS,
        }
        
        button {
            r#type: "button",
            class,
            "data-variant": props.variant.as_str(),
            "data-size": props.size.as_str(),
            "data-disabled": props.disabled,
            "data-loading": props.loading,
            "data-full-width": props.full_width,
            "data-icon-only": props.icon_only,
            "data-responsive": props.responsive,
            disabled: props.disabled || props.loading,
            onclick: move |evt| {
                if !props.disabled && !props.loading {
                    props.onclick.call(evt);
                }
            },
            ..props.attributes,
            
            if props.loading {
                div { class: "aqio-button-spinner" }
            }
            
            div { class: "aqio-button-content", 
                {props.children}
            }
        }
    }
}

/// Props for a Button that acts as a link
#[derive(Props, Clone, PartialEq)]
pub struct ButtonLinkProps {
    /// URL to navigate to
    pub href: String,

    /// Visual style variant of the button
    #[props(default)]
    pub variant: ButtonVariant,
    
    /// Size of the button
    #[props(default)]
    pub size: ButtonSize,
    
    /// Whether the button is disabled
    #[props(default)]
    pub disabled: bool,

    /// Whether to open link in new tab
    #[props(default)]
    pub new_tab: bool,

    /// Whether the button should take full width
    #[props(default)]
    pub full_width: bool,

    /// Whether this is an icon-only button
    #[props(default)]
    pub icon_only: bool,

    /// Whether the button should be full width on mobile
    #[props(default)]
    pub responsive: bool,

    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    
    /// Button content
    pub children: Element,
}

/// # ButtonLink
/// 
/// A button component that acts as a link (using an anchor tag)
#[component]
pub fn ButtonLink(props: ButtonLinkProps) -> Element {
    let class = format!("aqio-button {}", props.class.unwrap_or_default());
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: AQIO_BUTTON_CSS,
        }
        
        a {
            href: props.href,
            target: if props.new_tab { "_blank" } else { "_self" },
            rel: if props.new_tab { "noopener noreferrer" } else { "" },
            class,
            "data-variant": props.variant.as_str(),
            "data-size": props.size.as_str(),
            "data-disabled": props.disabled,
            "data-full-width": props.full_width,
            "data-icon-only": props.icon_only,
            "data-responsive": props.responsive,
            "aria-disabled": props.disabled,
            ..props.attributes,
            
            div { class: "aqio-button-content", 
                {props.children}
            }
        }
    }
}

/// Props for ButtonGroup container
#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupProps {
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    
    /// Button components to group together
    pub children: Element,
}

/// # ButtonGroup
/// 
/// A container component that groups buttons together with connected borders
#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let class = format!("aqio-button-group {}", props.class.unwrap_or_default());
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: AQIO_BUTTON_CSS,
        }
        
        div {
            class,
            role: "group",
            ..props.attributes,
            {props.children}
        }
    }
}