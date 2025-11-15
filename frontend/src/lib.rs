use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

mod components;
mod pages;

use components::navbar::Navbar;
use pages::{
    architecture::Architecture,
    contact::Contact,
    projects::Projects,
    proposal::Proposal,
    roadmap::Roadmap,
};

#[component]
pub fn App() -> impl IntoView {
    // Provide meta context - safe to call multiple times (idempotent)
    // On server: AppShell also calls this, but that's okay
    // On client: This is the only place it's called
    provide_meta_context();
    
    // Initialize theme - default to dark (HTML has class="dark" by default)
    // Theme toggle component will sync with localStorage on mount
    let (theme_read, theme_write) = signal("dark".to_string());
    provide_context((theme_read, theme_write));

    view! {
        <Title text="XFSolutions - Professional Solutions"/>
        <Meta name="description" content="XFSolutions - Delivering excellence in technology solutions"/>

        <Router>
            <div class="min-h-screen bg-gray-50 dark:bg-black text-gray-900 dark:text-white transition-colors duration-300">
                <Navbar/>
                <main class="container mx-auto px-4 py-8">
                    <Routes fallback=|| "Page not found.">
                        <Route path=StaticSegment("") view=Projects/>
                        <Route path=StaticSegment("projects") view=Projects/>
                        <Route path=StaticSegment("contact") view=Contact/>
                        <Route path=StaticSegment("proposal") view=Proposal/>
                        <Route path=StaticSegment("architecture") view=Architecture/>
                        <Route path=StaticSegment("roadmap") view=Roadmap/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
