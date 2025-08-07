use dioxus::prelude::*;

// pub mod tokens;

#[derive(Clone, Debug, PartialEq)]
pub enum AqioTheme {
    Light,
    Dark,
    Auto,
}

#[component]
pub fn ThemeProvider(theme: AqioTheme, children: Element) -> Element {
    use_context_provider(|| theme.clone());
    
    let theme_class = match theme {
        AqioTheme::Light => "aqio-theme-light",
        AqioTheme::Dark => "aqio-theme-dark", 
        AqioTheme::Auto => "aqio-theme-auto",
    };

    rsx! {
        div {
            style: "
                /* AQIO Design System Variables */
                --aqio-blue-primary: #1B4D8C;
                --aqio-blue-secondary: #4A90E2;
                --aqio-green-primary: #2D5A3D;
                --aqio-green-secondary: #52C41A;
                --aqio-orange-primary: #FF6B35;
                --aqio-text: #1E293B;
                --aqio-text-secondary: #64748B;
                --aqio-text-disabled: #94A3B8;
                --aqio-background: #FFFFFF;
                --aqio-surface: #F8FAFC;
                --aqio-border: #E2E8F0;
                --aqio-success: #52C41A;
                --aqio-warning: #F59E0B;
                --aqio-error: #EF4444;
                --aqio-error-light: #FEE2E2;
                --aqio-info: #4A90E2;
                
                /* Spacing */
                --aqio-space-0: 0;
                --aqio-space-1: 0.25rem;
                --aqio-space-2: 0.5rem;
                --aqio-space-3: 0.75rem;
                --aqio-space-4: 1rem;
                --aqio-space-6: 1.5rem;
                --aqio-space-8: 2rem;
                --aqio-space-12: 3rem;
                
                /* Typography */
                --aqio-text-xs: 0.75rem;
                --aqio-text-sm: 0.875rem;
                --aqio-text-base: 1rem;
                --aqio-text-lg: 1.125rem;
                --aqio-text-xl: 1.25rem;
                --aqio-text-2xl: 1.5rem;
                --aqio-text-3xl: 1.875rem;
                --aqio-text-4xl: 2.25rem;
                
                --aqio-font-light: 300;
                --aqio-font-normal: 400;
                --aqio-font-medium: 500;
                --aqio-font-semibold: 600;
                --aqio-font-bold: 700;
                
                --aqio-leading-normal: 1.5;
                --aqio-leading-tight: 1.25;
                
                /* Border Radius */
                --aqio-radius-md: 0.375rem;
                --aqio-radius-lg: 0.5rem;
                
                /* Transitions */
                --aqio-transition-fast: 150ms ease-in-out;
                
                /* Shadows */
                --aqio-shadow-lg: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
                
                /* Font */
                --aqio-font-family: Inter, -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
                
                font-family: var(--aqio-font-family);
                color: var(--aqio-text);
                background-color: var(--aqio-background);
            ",
            class: "{theme_class}",
            "data-theme": match theme {
                AqioTheme::Light => "light",
                AqioTheme::Dark => "dark",
                AqioTheme::Auto => "auto",
            },
            {children}
        }
    }
}

pub fn use_theme() -> AqioTheme {
    use_context::<AqioTheme>()
}

// Alias for backward compatibility
pub use AqioTheme as Theme;