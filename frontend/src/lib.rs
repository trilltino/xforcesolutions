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
    about::About,
    architecture::Architecture,
    contact::Contact,
    contracts::Contracts,
    documentation::Documentation,
    home::Home,
    projects::Projects,
    proposal::Proposal,
    roadmap::{Roadmap, Month1, Month2, Month3, Month4},
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
                        <Route path=StaticSegment("") view=About/>
                        <Route path=StaticSegment("home") view=Home/>
                        <Route path=StaticSegment("about") view=About/>
                        <Route path=StaticSegment("projects") view=Projects/>
                        <Route path=StaticSegment("contact") view=Contact/>
                        <Route path=StaticSegment("proposal") view=Proposal/>
                        <Route path=StaticSegment("architecture") view=Architecture/>
                        <Route path=StaticSegment("roadmap") view=Roadmap/>
                        <Route path=StaticSegment("roadmap/month1") view=Month1/>
                        <Route path=StaticSegment("roadmap/month2") view=Month2/>
                        <Route path=StaticSegment("roadmap/month3") view=Month3/>
                        <Route path=StaticSegment("roadmap/month4") view=Month4/>
                        <Route path=StaticSegment("contracts") view=Contracts/>
                        <Route path=StaticSegment("documentation") view=Documentation/>
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
