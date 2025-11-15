use leptos::prelude::*;

#[component]
pub fn Architecture() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto py-8">
            <div class="text-center mb-8">
                <h1 class="text-4xl font-bold mb-4 font-heading reflective-header">"Architecture Documentation"</h1>
                <p class="text-xl text-gray-300 dark:text-gray-300 text-gray-700 dark:text-gray-300 mb-6 font-sans">
                    "Technical architecture and system design for XFTerminal."
                </p>
                <a
                    href="/xforcesolutions/docs/index.html"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-block px-8 py-4 bg-primary-600 hover:bg-primary-700 text-white rounded-lg font-semibold transition-colors duration-200 text-lg"
                >
                    "Open Architecture Documentation →"
                </a>
            </div>

            <div class="bg-black dark:bg-black bg-white dark:bg-black p-8 rounded-lg shadow-lg mb-6">
                <h2 class="text-2xl font-bold mb-4 font-heading text-gray-900 dark:text-white">"Documentation Sections"</h2>
                <div class="grid md:grid-cols-2 gap-4">
                    <a
                        href="/xforcesolutions/docs/index.html"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="block p-4 bg-gray-800 dark:bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors"
                    >
                        <h3 class="text-lg font-bold mb-2 text-white">"Overview"</h3>
                        <p class="text-gray-400 text-sm">"Main documentation index"</p>
                    </a>
                    <a
                        href="/xforcesolutions/docs/tech-stack.html"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="block p-4 bg-gray-800 dark:bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors"
                    >
                        <h3 class="text-lg font-bold mb-2 text-white">"Tech Stack"</h3>
                        <p class="text-gray-400 text-sm">"Technology stack details"</p>
                    </a>
                    <a
                        href="/xforcesolutions/docs/terminal.html"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="block p-4 bg-gray-800 dark:bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors"
                    >
                        <h3 class="text-lg font-bold mb-2 text-white">"Terminal"</h3>
                        <p class="text-gray-400 text-sm">"Terminal architecture"</p>
                    </a>
                    <a
                        href="/xforcesolutions/docs/data-flows.html"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="block p-4 bg-gray-800 dark:bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors"
                    >
                        <h3 class="text-lg font-bold mb-2 text-white">"Data Flows"</h3>
                        <p class="text-gray-400 text-sm">"Data flow diagrams"</p>
                    </a>
                    <a
                        href="/xforcesolutions/docs/integrations.html"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="block p-4 bg-gray-800 dark:bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors"
                    >
                        <h3 class="text-lg font-bold mb-2 text-white">"Integrations"</h3>
                        <p class="text-gray-400 text-sm">"Integration architecture"</p>
                    </a>
                    <a
                        href="/xforcesolutions/docs/news-service.html"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="block p-4 bg-gray-800 dark:bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors"
                    >
                        <h3 class="text-lg font-bold mb-2 text-white">"News Service"</h3>
                        <p class="text-gray-400 text-sm">"News service architecture"</p>
                    </a>
                    <a
                        href="/xforcesolutions/docs/contracts.html"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="block p-4 bg-gray-800 dark:bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors"
                    >
                        <h3 class="text-lg font-bold mb-2 text-white">"Contracts"</h3>
                        <p class="text-gray-400 text-sm">"Contract specifications"</p>
                    </a>
                </div>
            </div>
        </div>
    }
}

