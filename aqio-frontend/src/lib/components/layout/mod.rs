use dioxus::prelude::*;

// Import the CSS for our layout components
const AQIO_LAYOUT_CSS: Asset = asset!("/assets/aqio-layout.css");

/// Container size variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ContainerSize {
    Small,    // max-width: 640px
    Medium,   // max-width: 768px 
    Large,    // max-width: 1024px
    #[default]
    XLarge,   // max-width: 1280px
    Full,     // max-width: 100%
}

impl ContainerSize {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::XLarge => "x-large",
            Self::Full => "full",
        }
    }
}

/// Props for the Container component
#[derive(Props, Clone, PartialEq)]
pub struct ContainerProps {
    /// Maximum width of the container
    #[props(default)]
    pub size: ContainerSize,

    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The container content
    pub children: Element,
}

/// # Container
/// 
/// A responsive container component with configurable maximum widths
#[component]
pub fn Container(props: ContainerProps) -> Element {
    let class = format!("aqio-container {}", props.class.unwrap_or_default());
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: AQIO_LAYOUT_CSS,
        }
        
        div {
            class,
            "data-size": props.size.as_str(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// Grid column count variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum GridColumns {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Twelve,
}

impl GridColumns {
    fn as_str(&self) -> &'static str {
        match self {
            Self::One => "one",
            Self::Two => "two",
            Self::Three => "three",
            Self::Four => "four",
            Self::Five => "five",
            Self::Six => "six",
            Self::Twelve => "twelve",
        }
    }
}

/// Gap size variants for layout components
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum GapSize {
    None,
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
}

impl GapSize {
    fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::XLarge => "x-large",
        }
    }
}

/// Props for the Grid component
#[derive(Props, Clone, PartialEq)]
pub struct GridProps {
    /// Number of columns in the grid
    #[props(default)]
    pub columns: GridColumns,

    /// Gap size between grid items
    #[props(default)]
    pub gap: GapSize,

    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The grid items
    pub children: Element,
}

/// # Grid
/// 
/// A CSS grid layout component with responsive behavior
#[component]
pub fn Grid(props: GridProps) -> Element {
    let class = format!("aqio-grid {}", props.class.unwrap_or_default());
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: AQIO_LAYOUT_CSS,
        }
        
        div {
            class,
            "data-columns": props.columns.as_str(),
            "data-gap": props.gap.as_str(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// Stack direction variants
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

/// Stack alignment variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum StackAlign {
    #[default]
    Start,
    Center,
    End,
    Stretch,
}

impl StackAlign {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center", 
            Self::End => "end",
            Self::Stretch => "stretch",
        }
    }
}

/// Stack justification variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum StackJustify {
    #[default]
    Start,
    Center,
    End,
    Between,
    Around,
    Evenly,
}

impl StackJustify {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::Between => "between",
            Self::Around => "around",
            Self::Evenly => "evenly",
        }
    }
}

/// Props for the Stack component
#[derive(Props, Clone, PartialEq)]
pub struct StackProps {
    /// Direction of the stack layout
    #[props(default)]
    pub direction: StackDirection,

    /// Alignment of items in the cross axis
    #[props(default)]
    pub align: StackAlign,

    /// Justification of items in the main axis  
    #[props(default)]
    pub justify: StackJustify,

    /// Gap size between items
    #[props(default)]
    pub gap: GapSize,

    /// Whether items should wrap
    #[props(default)]
    pub wrap: bool,

    /// Whether to be responsive (vertical on mobile, respect direction on desktop)
    #[props(default)]
    pub responsive: bool,

    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The items to stack
    pub children: Element,
}

/// # Stack
/// 
/// A flexible layout component for arranging items in a vertical or horizontal stack
#[component]  
pub fn Stack(props: StackProps) -> Element {
    let class = format!("aqio-stack {}", props.class.unwrap_or_default());
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: AQIO_LAYOUT_CSS,
        }
        
        div {
            class,
            "data-direction": props.direction.as_str(),
            "data-align": props.align.as_str(),
            "data-justify": props.justify.as_str(), 
            "data-gap": props.gap.as_str(),
            "data-wrap": props.wrap,
            "data-responsive": props.responsive,
            ..props.attributes,
            {props.children}
        }
    }
}

/// Spacer size variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SpacerSize {
    #[default]
    Auto,  // flex: 1 (takes available space)
    Small,
    Medium,
    Large,
    XLarge,
}

impl SpacerSize {
    fn as_str(&self) -> Option<&'static str> {
        match self {
            Self::Auto => None,
            Self::Small => Some("small"),
            Self::Medium => Some("medium"),
            Self::Large => Some("large"),
            Self::XLarge => Some("x-large"),
        }
    }
}

/// Props for the Spacer component
#[derive(Props, Clone, PartialEq)]
pub struct SpacerProps {
    /// Size of the spacer
    #[props(default)]
    pub size: SpacerSize,

    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # Spacer
/// 
/// A flexible spacer component that takes up available space or provides fixed spacing
#[component]
pub fn Spacer(props: SpacerProps) -> Element {
    let class = format!("aqio-spacer {}", props.class.unwrap_or_default());
    
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: AQIO_LAYOUT_CSS,
        }
        
        div {
            class,
            "data-size": props.size.as_str(),
            ..props.attributes,
        }
    }
}