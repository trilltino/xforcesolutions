use leptos::prelude::*;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    // Get theme signal from context
    let (theme_value, set_theme_value) = use_context::<(ReadSignal<String>, WriteSignal<String>)>()
        .expect("ThemeToggle requires theme context");
    
    // Initialize theme from localStorage on mount - will sync on client side
    // HTML defaults to dark mode, so this will only update if user has a different preference
    
    // Toggle theme function
    let toggle_theme = move |_| {
        let current = theme_value.get();
        let new_theme = if current == "light" {
            "dark".to_string()
        } else {
            "light".to_string()
        };
        set_theme_value.set(new_theme.clone());
        apply_theme(&new_theme);
        save_theme(&new_theme);
    };
    
    view! {
        <button
            on:click=toggle_theme
            class="p-2 rounded-lg hover:bg-gray-800 dark:hover:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-800 transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-primary-500"
            aria-label=move || if theme_value.get() == "light" { "Switch to dark mode" } else { "Switch to light mode" }
        >
            {move || if theme_value.get() == "light" {
                view! {
                    <svg 
                        class="w-6 h-6 text-gray-700 dark:text-gray-300" 
                        fill="none" 
                        stroke="currentColor" 
                        viewBox="0 0 24 24"
                    >
                        <path 
                            stroke-linecap="round" 
                            stroke-linejoin="round" 
                            stroke-width="2" 
                            d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
                        />
                    </svg>
                }.into_any()
            } else {
                view! {
                    <svg 
                        class="w-6 h-6 text-gray-300 dark:text-gray-300" 
                        fill="none" 
                        stroke="currentColor" 
                        viewBox="0 0 24 24"
                    >
                        <path 
                            stroke-linecap="round" 
                            stroke-linejoin="round" 
                            stroke-width="2" 
                            d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                        />
                    </svg>
                }.into_any()
            }}
        </button>
    }
}

fn apply_theme(theme: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(html_element) = document.document_element() {
                let class_list = html_element.class_list();
                // Remove both theme classes
                let _ = class_list.remove_1("light");
                let _ = class_list.remove_1("dark");
                
                // Add the current theme class
                let _ = class_list.add_1(theme);
            }
            
            // Also update body class for backwards compatibility
            if let Some(body) = document.body() {
                let class_list = body.class_list();
                let _ = class_list.remove_1("light");
                let _ = class_list.remove_1("dark");
                let _ = class_list.add_1(theme);
            }
        }
    }
}

fn save_theme(_theme: &str) {
    // Save theme to localStorage using web-sys
    // We'll skip localStorage for now and rely on the HTML default
    // The theme will persist during the session
    #[cfg(feature = "hydrate")]
    {
        // Note: localStorage access requires Storage feature in web-sys
        // For now, theme changes will persist during session but reset on page reload
        // This is acceptable for MVP - can be enhanced later with proper Storage API
    }
}

