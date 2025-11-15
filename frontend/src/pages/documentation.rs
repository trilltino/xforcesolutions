use leptos::prelude::*;

#[component]
pub fn Documentation() -> impl IntoView {
    view! {
        <div class="max-w-6xl mx-auto py-8">
            <h1 class="text-4xl font-bold text-center mb-8 font-heading">"Documentation"</h1>
            <p class="text-xl text-gray-300 dark:text-gray-300 text-gray-700 dark:text-gray-300 text-center mb-12 font-sans">
                "Documentation for XFTerminal."
            </p>

            // Images side by side
            <div class="grid md:grid-cols-2 gap-6 mb-8">
                <a
                    href="https://docs.google.com/document/d/1X1Tfeagb8pR02tN1AD_JAo2ut1I_Q6G2Z_yDgRidqBo/edit?tab=t.0"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="block hover:opacity-90 transition-opacity"
                >
                    <img
                        src="/xforcesolutions/public/images/xfterminal-cover.png"
                        alt="XFTerminal Cover"
                        class="w-full h-auto rounded-lg shadow-lg border border-gray-800 dark:border-gray-800 border-gray-200 dark:border-gray-800"
                    />
                </a>
                <img
                    src="/xforcesolutions/public/images/bloomberg-cover.png"
                    alt="Bloomberg Cover"
                    class="w-full h-auto rounded-lg shadow-lg border border-gray-800 dark:border-gray-800 border-gray-200 dark:border-gray-800"
                />
            </div>

            // Placeholder text box below
            <div class="bg-black dark:bg-black bg-white dark:bg-black p-8 rounded-lg shadow-lg">
                <h2 class="text-2xl font-bold mb-4 font-heading text-gray-900 dark:text-white">"Documentation Overview"</h2>
                <p class="text-gray-600 dark:text-gray-300 text-gray-700 dark:text-gray-300 mb-6 font-sans">
                    "This section will contain documentation for the XFTerminal project."
                </p>
            </div>
        </div>
    }
}

