use leptos::prelude::*;

#[component]
pub fn Contracts() -> impl IntoView {
    view! {
        <div class="max-w-4xl mx-auto py-8">
            <h1 class="text-4xl font-bold text-center mb-8 font-heading">"Contracts"</h1>
            <p class="text-xl text-gray-300 dark:text-gray-300 text-gray-700 dark:text-gray-300 text-center mb-12 font-sans">
                "Contract information and agreements for XFTerminal."
            </p>

            <div class="bg-black dark:bg-black bg-white dark:bg-black p-8 rounded-lg shadow-lg">
                <h2 class="text-2xl font-bold mb-4 font-heading text-gray-900 dark:text-white">"Contract Overview"</h2>
                <p class="text-gray-600 dark:text-gray-300 text-gray-700 dark:text-gray-300 mb-6 font-sans">
                    "This section will contain contract information, terms, and agreements for the XFTerminal project."
                </p>
            </div>
        </div>
    }
}

