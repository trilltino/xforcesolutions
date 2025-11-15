use leptos::prelude::*;
use leptos_router::components::A;

use super::theme_toggle::ThemeToggle;

#[component]
pub fn Navbar() -> impl IntoView {
    let (mobile_menu_open, set_mobile_menu_open) = signal(false);

    let toggle_mobile_menu = move |_| {
        set_mobile_menu_open.update(|open| *open = !*open);
    };

    view! {
        <nav class="fixed top-0 left-0 right-0 z-50 bg-black/95 backdrop-blur-sm border-b border-red-900 shadow-lg">
            <div class="container mx-auto px-4">
                <div class="flex items-center justify-between h-16">
                    // Logo/Brand with icon
                    <div class="flex-shrink-0">
                        <A href="/" attr:class="flex items-center space-x-2 text-2xl font-bold text-white hover:text-primary-400 transition-all duration-200 group">
                            <div class="w-10 h-10 bg-gradient-to-br from-primary-500 to-primary-700 rounded-lg flex items-center justify-center group-hover:scale-110 transition-transform duration-200">
                                <span class="text-white font-black text-lg font-nav">"XF"</span>
                            </div>
                            <span class="bg-gradient-to-r from-white to-gray-300 bg-clip-text text-transparent font-nav">
                                "XFSolutions"
                            </span>
                        </A>
                    </div>

                    // Desktop Navigation Links
                    <div class="hidden md:flex items-center space-x-1">
                        <NavLink href="/about" label="About"/>
                        <NavLink href="/projects" label="Projects"/>
                        <NavLink href="/contact" label="Contact"/>
                        
                        // Theme Toggle
                        <div class="ml-2">
                            <ThemeToggle/>
                        </div>
                    </div>

                    // Mobile menu button and theme toggle
                    <div class="md:hidden flex items-center space-x-2">
                        <ThemeToggle/>
                        <button
                            on:click=toggle_mobile_menu
                            class="text-gray-400 hover:text-white focus:outline-none p-2 rounded-lg hover:bg-gray-800 transition-colors"
                            aria-label="Toggle menu"
                        >
                            {move || if mobile_menu_open.get() {
                                view! {
                                    <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                                    </svg>
                                }.into_any()
                            } else {
                                view! {
                                    <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                                    </svg>
                                }.into_any()
                            }}
                        </button>
                    </div>
                </div>

                // Mobile menu
                {move || if mobile_menu_open.get() {
                    view! {
                        <div class="md:hidden pb-4 border-t border-red-900 mt-2 animate-fade-in">
                            <div class="flex flex-col space-y-2 pt-4">
                                <MobileNavLink href="/about" label="About" on_click=move |_| set_mobile_menu_open.set(false)/>
                                <MobileNavLink href="/projects" label="Projects" on_click=move |_| set_mobile_menu_open.set(false)/>
                                <MobileNavLink href="/contact" label="Contact" on_click=move |_| set_mobile_menu_open.set(false)/>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>
        </nav>

        // Spacer to prevent content from going under fixed navbar
        <div class="h-16"></div>
    }
}

#[component]
fn NavLink(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class="relative px-4 py-2 text-gray-300 hover:text-red-400 text-sm font-medium font-nav transition-all duration-200 rounded-lg hover:bg-red-900/30 aria-[current]:text-red-400 aria-[current]:bg-red-900/20 group"
        >
            {label}
            <span class="absolute bottom-0 left-0 w-0 h-0.5 bg-gradient-to-r from-primary-500 to-primary-700 group-hover:w-full transition-all duration-300"></span>
        </A>
    }
}

#[component]
fn MobileNavLink(
    href: &'static str,
    label: &'static str,
    on_click: impl Fn(leptos::ev::MouseEvent) + 'static,
) -> impl IntoView {
    view! {
        <A
            href=href
            on:click=on_click
            attr:class="px-4 py-3 text-gray-300 hover:text-red-400 hover:bg-red-900/30 rounded-lg transition-all duration-200 aria-[current]:text-red-400 aria-[current]:bg-red-900/20 font-medium font-nav"
        >
            {label}
        </A>
    }
}
