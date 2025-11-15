use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto">
            // Hero Section with Image
            <div class="relative mb-16">
                <div class="relative h-[500px] md:h-[600px] rounded-lg overflow-hidden">
                    <img 
                        src="./images/hero.webp" 
                        alt="Technology Solutions" 
                        class="w-full h-full object-cover"
                    />
                    <div class="absolute inset-0 bg-gradient-to-t from-black/90 via-black/50 to-transparent"></div>
                    <div class="absolute inset-0 flex items-center justify-center">
                        <div class="text-center px-4">
                            <h1 class="text-5xl md:text-7xl font-bold mb-6 font-heading reflective-header">
                                "Welcome to XFSolutions"
                            </h1>
                            <p class="text-xl md:text-2xl text-gray-300 dark:text-gray-300 text-gray-700 dark:text-gray-300 max-w-2xl mx-auto font-sans">
                                "Delivering innovative technology solutions for your business"
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            // Call to Action Section
            <div class="grid md:grid-cols-2 gap-6 mb-16">
                <div class="bg-black dark:bg-black bg-white dark:bg-black p-8 rounded-lg">
                    <h2 class="text-3xl font-bold mb-4 font-heading reflective-header">
                        "Explore Our Projects"
                    </h2>
                    <p class="text-lg text-gray-300 dark:text-gray-300 text-gray-700 dark:text-gray-300 mb-6 font-sans">
                        "Discover our portfolio of innovative technology solutions"
                    </p>
                    <a
                        href="/projects"
                        class="inline-block px-6 py-3 bg-primary-600 hover:bg-primary-700 text-white rounded-lg font-semibold transition-colors duration-200"
                    >
                        "View Projects"
                    </a>
                </div>
                <div class="bg-black dark:bg-black bg-white dark:bg-black p-8 rounded-lg">
                    <h2 class="text-3xl font-bold mb-4 font-heading reflective-header">
                        "Get in Touch"
                    </h2>
                    <p class="text-lg text-gray-300 dark:text-gray-300 text-gray-700 dark:text-gray-300 mb-6 font-sans">
                        "Have a project in mind? Let's discuss how we can help"
                    </p>
                    <a
                        href="/contact"
                        class="inline-block px-6 py-3 bg-primary-600 hover:bg-primary-700 text-white rounded-lg font-semibold transition-colors duration-200"
                    >
                        "Contact Us"
                    </a>
                </div>
            </div>

            // Technology Section
            <div class="relative rounded-lg overflow-hidden mb-16">
                <img 
                    src="./images/technology.webp" 
                    alt="Technology Innovation" 
                    class="w-full h-[400px] object-cover"
                />
                <div class="absolute inset-0 bg-gradient-to-r from-black/90 to-transparent"></div>
                <div class="absolute inset-0 flex items-center">
                    <div class="max-w-2xl px-8">
                        <h2 class="text-4xl md:text-5xl font-bold mb-4 text-white dark:text-white text-gray-900 dark:text-white font-heading reflective-header">
                            "Innovation at the Core"
                        </h2>
                        <p class="text-lg text-gray-300 dark:text-gray-300 text-gray-700 dark:text-gray-300 font-sans">
                            "We combine cutting-edge technology with real-world expertise to deliver solutions that drive your business forward."
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

