use dioxus::prelude::*;

// Import the CSS for our navigation components
const AQIO_NAVIGATION_CSS: Asset = asset!("/assets/aqio-navigation.css");

/// Props for the main Navbar component
#[derive(Props, Clone, PartialEq)]
pub struct NavbarProps {
    /// Whether the navbar is disabled
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply to the navbar element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the navbar component
    pub children: Element,
}

/// # Navbar
/// 
/// The main navigation bar component that provides a consistent header for the application.
/// Uses CSS classes and data attributes for styling following the preview pattern.
#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    let class = format!("aqio-navbar {}", props.class.unwrap_or_default());
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: AQIO_NAVIGATION_CSS,
        }
        
        nav {
            class,
            "data-disabled": props.disabled,
            ..props.attributes,
            
            div { class: "aqio-container",
                {props.children}
            }
        }
    }
}

/// Props for the NavbarBrand component
#[derive(Props, Clone, PartialEq)]
pub struct NavbarBrandProps {
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the brand component
    pub children: Element,
}

/// # NavbarBrand
/// 
/// Component for displaying the brand/logo section of the navbar
#[component]
pub fn NavbarBrand(props: NavbarBrandProps) -> Element {
    let class = format!("aqio-navbar-brand {}", props.class.unwrap_or_default());
    
    rsx! {
        div {
            class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for the NavbarSubtitle component
#[derive(Props, Clone, PartialEq)]
pub struct NavbarSubtitleProps {
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the subtitle component
    pub children: Element,
}

/// # NavbarSubtitle
/// 
/// Component for displaying a subtitle below the brand (hidden on mobile)
#[component]
pub fn NavbarSubtitle(props: NavbarSubtitleProps) -> Element {
    let class = format!("aqio-navbar-subtitle {}", props.class.unwrap_or_default());
    
    rsx! {
        div {
            class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for the NavbarLinks container
#[derive(Props, Clone, PartialEq)]
pub struct NavbarLinksProps {
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children navigation links
    pub children: Element,
}

/// # NavbarLinks
/// 
/// Container component for navigation links
#[component]
pub fn NavbarLinks(props: NavbarLinksProps) -> Element {
    let class = format!("aqio-navbar-links {}", props.class.unwrap_or_default());
    
    rsx! {
        div {
            class,
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for individual navigation links
#[derive(Props, Clone, PartialEq)]
pub struct NavLinkProps {
    /// The URL to navigate to
    pub to: String,

    /// Whether this link is currently active
    #[props(default)]
    pub active: bool,

    /// Whether this link is disabled
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the link
    pub children: Element,
}

/// # NavLink
/// 
/// Individual navigation link component
#[component]
pub fn NavLink(props: NavLinkProps) -> Element {
    let class = format!("aqio-nav-link {}", props.class.unwrap_or_default());
    
    rsx! {
        a {
            href: props.to,
            class,
            "data-active": props.active,
            "data-disabled": props.disabled,
            ..props.attributes,
            {props.children}
        }
    }
}

/// Button variants for navigation buttons
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum NavButtonVariant {
    #[default]
    Primary,
    Secondary,
}

impl NavButtonVariant {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
        }
    }
}

/// Button sizes for navigation buttons
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum NavButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl NavButtonSize {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

/// Props for navigation buttons
#[derive(Props, Clone, PartialEq)]
pub struct NavButtonProps {
    /// The button variant
    #[props(default)]
    pub variant: NavButtonVariant,

    /// The button size
    #[props(default)]
    pub size: NavButtonSize,

    /// Whether the button is disabled
    #[props(default)]
    pub disabled: bool,

    /// Click handler for the button
    #[props(default)]
    pub onclick: EventHandler<MouseEvent>,

    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the button
    pub children: Element,
}

/// # NavButton
/// 
/// Button component specifically styled for navigation bars
#[component]
pub fn NavButton(props: NavButtonProps) -> Element {
    let class = format!("aqio-nav-button {}", props.class.unwrap_or_default());
    
    rsx! {
        button {
            r#type: "button",
            class,
            "data-variant": props.variant.as_str(),
            "data-size": props.size.as_str(),
            "data-disabled": props.disabled,
            disabled: props.disabled,
            onclick: move |evt| {
                if !props.disabled {
                    props.onclick.call(evt);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for the Footer component
#[derive(Props, Clone, PartialEq)]
pub struct FooterProps {
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the footer
    pub children: Element,
}

/// # Footer
/// 
/// Footer component for the application
#[component]
pub fn Footer(props: FooterProps) -> Element {
    let class = format!("aqio-footer {}", props.class.unwrap_or_default());
    
    rsx! {
        footer {
            class,
            ..props.attributes,
            
            div { class: "aqio-footer-content",
                {props.children}
            }
        }
    }
}

/// Stack direction enum for layout components
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum StackDirection {
    #[default]
    Vertical,
    Horizontal,
}

impl StackDirection {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

/// Stack alignment enum
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum StackAlign {
    #[default]
    Start,
    Center,
    End,
}

impl StackAlign {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center", 
            Self::End => "end",
        }
    }
}

/// Stack justify enum
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum StackJustify {
    #[default]
    Start,
    Center,
    End,
    Between,
}

impl StackJustify {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::Between => "between",
        }
    }
}

/// Gap size for stacks
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum StackGap {
    Small,
    #[default]
    Medium,
    Large,
}

impl StackGap {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

/// Props for Stack layout component
#[derive(Props, Clone, PartialEq)]
pub struct StackProps {
    /// Direction of the stack
    #[props(default)]
    pub direction: StackDirection,

    /// Alignment of items
    #[props(default)]
    pub align: StackAlign,

    /// Justification of items  
    #[props(default)]
    pub justify: StackJustify,

    /// Gap size between items
    #[props(default)]
    pub gap: StackGap,

    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to stack
    pub children: Element,
}

/// # Stack
/// 
/// Layout component for arranging items in a vertical or horizontal stack
#[component]  
pub fn Stack(props: StackProps) -> Element {
    let class = format!("aqio-stack {}", props.class.unwrap_or_default());
    
    rsx! {
        div {
            class,
            "data-direction": props.direction.as_str(),
            "data-align": props.align.as_str(),
            "data-justify": props.justify.as_str(), 
            "data-gap": props.gap.as_str(),
            ..props.attributes,
            {props.children}
        }
    }
}

// Legacy alias for migration (this was causing the compilation issues)
pub struct Breadcrumb;