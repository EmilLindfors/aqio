use dioxus::prelude::*;

// Import the CSS for our typography components
const AQIO_TYPOGRAPHY_CSS: Asset = asset!("/assets/aqio-typography.css");

/// Text size variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextSize {
    XSmall,   // 12px
    Small,    // 14px
    #[default]
    Medium,   // 16px
    Large,    // 18px
    XLarge,   // 20px
    XXLarge,  // 24px
}

impl TextSize {
    fn as_str(&self) -> &'static str {
        match self {
            Self::XSmall => "x-small",
            Self::Small => "small", 
            Self::Medium => "medium",
            Self::Large => "large",
            Self::XLarge => "x-large",
            Self::XXLarge => "xx-large",
        }
    }
}

/// Text weight variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextWeight {
    Light,
    #[default]
    Normal,
    Medium,
    SemiBold,
    Bold,
}

impl TextWeight {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Normal => "normal",
            Self::Medium => "medium",
            Self::SemiBold => "semi-bold",
            Self::Bold => "bold",
        }
    }
}

/// Text color variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextColor {
    #[default]
    Primary,
    Secondary,
    Disabled,
    Success,
    Warning,
    Error,
    Info,
    White,
}

impl TextColor {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Disabled => "disabled",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Info => "info",
            Self::White => "white",
        }
    }
}

/// Text alignment variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

impl TextAlign {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
            Self::Justify => "justify",
        }
    }
}

/// Props for the Text component
#[derive(Props, Clone, PartialEq)]
pub struct TextProps {
    /// Size of the text
    #[props(default)]
    pub size: TextSize,

    /// Weight of the text
    #[props(default)]
    pub weight: TextWeight,

    /// Color of the text
    #[props(default)]
    pub color: TextColor,

    /// Alignment of the text
    #[props(default)]
    pub align: TextAlign,

    /// Whether to truncate text with ellipsis
    #[props(default)]
    pub truncate: bool,

    /// Text transformation
    #[props(default)]
    pub uppercase: bool,

    /// Text transformation
    #[props(default)]
    pub lowercase: bool,

    /// Text transformation
    #[props(default)]
    pub capitalize: bool,

    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The text content or children
    pub children: Element,
}

/// # Text
/// 
/// A flexible text component that supports various sizes, weights, colors, and styling options.
/// Uses CSS classes and data attributes for styling following the preview pattern.
#[component]
pub fn Text(props: TextProps) -> Element {
    let class = format!("aqio-text {}", props.class.unwrap_or_default());
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: AQIO_TYPOGRAPHY_CSS,
        }
        
        span {
            class,
            "data-size": props.size.as_str(),
            "data-weight": props.weight.as_str(),
            "data-color": props.color.as_str(),
            "data-align": props.align.as_str(),
            "data-truncate": props.truncate,
            "data-uppercase": props.uppercase,
            "data-lowercase": props.lowercase,
            "data-capitalize": props.capitalize,
            ..props.attributes,
            {props.children}
        }
    }
}

/// Heading level variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum HeadingLevel {
    #[default]
    H1,
    H2, 
    H3,
    H4,
    H5,
    H6,
}

impl HeadingLevel {
    fn as_str(&self) -> &'static str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
        }
    }

    fn as_element(&self) -> &'static str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
        }
    }
}

/// Margin bottom variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum MarginBottom {
    #[default]
    Default,
    None,
    Small,
    Large,
}

impl MarginBottom {
    fn as_str(&self) -> Option<&'static str> {
        match self {
            Self::Default => None,
            Self::None => Some("none"),
            Self::Small => Some("small"),
            Self::Large => Some("large"),
        }
    }
}

/// Props for the Heading component
#[derive(Props, Clone, PartialEq)]
pub struct HeadingProps {
    /// Level of the heading (H1-H6)
    #[props(default)]
    pub level: HeadingLevel,

    /// Color of the heading
    #[props(default)]
    pub color: TextColor,

    /// Alignment of the heading
    #[props(default)]
    pub align: TextAlign,

    /// Bottom margin of the heading
    #[props(default)]
    pub margin_bottom: MarginBottom,

    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The heading content
    pub children: Element,
}

/// # Heading
/// 
/// A semantic heading component that supports different levels (H1-H6) with consistent styling.
#[component]
pub fn Heading(props: HeadingProps) -> Element {
    let class = format!("aqio-heading {}", props.class.unwrap_or_default());
    let element = props.level.as_element();
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: AQIO_TYPOGRAPHY_CSS,
        }
        
        {match element {
            "h1" => rsx! {
                h1 {
                    class,
                    "data-level": props.level.as_str(),
                    "data-color": props.color.as_str(),
                    "data-align": props.align.as_str(),
                    "data-margin-bottom": props.margin_bottom.as_str(),
                    ..props.attributes,
                    {props.children}
                }
            },
            "h2" => rsx! {
                h2 {
                    class,
                    "data-level": props.level.as_str(),
                    "data-color": props.color.as_str(),
                    "data-align": props.align.as_str(),
                    "data-margin-bottom": props.margin_bottom.as_str(),
                    ..props.attributes,
                    {props.children}
                }
            },
            "h3" => rsx! {
                h3 {
                    class,
                    "data-level": props.level.as_str(),
                    "data-color": props.color.as_str(),
                    "data-align": props.align.as_str(),
                    "data-margin-bottom": props.margin_bottom.as_str(),
                    ..props.attributes,
                    {props.children}
                }
            },
            "h4" => rsx! {
                h4 {
                    class,
                    "data-level": props.level.as_str(),
                    "data-color": props.color.as_str(),
                    "data-align": props.align.as_str(),
                    "data-margin-bottom": props.margin_bottom.as_str(),
                    ..props.attributes,
                    {props.children}
                }
            },
            "h5" => rsx! {
                h5 {
                    class,
                    "data-level": props.level.as_str(),
                    "data-color": props.color.as_str(),
                    "data-align": props.align.as_str(),
                    "data-margin-bottom": props.margin_bottom.as_str(),
                    ..props.attributes,
                    {props.children}
                }
            },
            "h6" => rsx! {
                h6 {
                    class,
                    "data-level": props.level.as_str(),
                    "data-color": props.color.as_str(),
                    "data-align": props.align.as_str(),
                    "data-margin-bottom": props.margin_bottom.as_str(),
                    ..props.attributes,
                    {props.children}
                }
            },
            _ => rsx! {
                h1 {
                    class,
                    "data-level": props.level.as_str(),
                    "data-color": props.color.as_str(),
                    "data-align": props.align.as_str(),
                    "data-margin-bottom": props.margin_bottom.as_str(),
                    ..props.attributes,
                    {props.children}
                }
            },
        }}
    }
}

/// Paragraph size variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ParagraphSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ParagraphSize {
    fn as_str(&self) -> Option<&'static str> {
        match self {
            Self::Small => Some("small"),
            Self::Medium => None,
            Self::Large => Some("large"),
        }
    }
}

/// Props for the Paragraph component
#[derive(Props, Clone, PartialEq)]
pub struct ParagraphProps {
    /// Size of the paragraph text
    #[props(default)]
    pub size: ParagraphSize,

    /// Color of the paragraph
    #[props(default)]
    pub color: TextColor,

    /// Alignment of the paragraph
    #[props(default)]
    pub align: TextAlign,

    /// Bottom margin of the paragraph
    #[props(default)]
    pub margin_bottom: MarginBottom,

    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The paragraph content
    pub children: Element,
}

/// # Paragraph
/// 
/// A semantic paragraph component with consistent typography and spacing.
#[component]
pub fn Paragraph(props: ParagraphProps) -> Element {
    let class = format!("aqio-paragraph {}", props.class.unwrap_or_default());
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: AQIO_TYPOGRAPHY_CSS,
        }
        
        p {
            class,
            "data-size": props.size.as_str(),
            "data-color": props.color.as_str(),
            "data-align": props.align.as_str(),
            "data-margin-bottom": props.margin_bottom.as_str(),
            ..props.attributes,
            {props.children}
        }
    }
}