use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto">
            <h1 class="text-4xl md:text-5xl font-bold mb-8 text-center font-heading reflective-header">
                "Proof of Work"
            </h1>
            
            <p class="text-xl text-gray-300 dark:text-gray-300 text-gray-700 dark:text-gray-300 text-center mb-12 font-sans">
                "Explore our portfolio of innovative technology solutions"
            </p>

            // Projects Grid
            <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
                <ProjectCard
                    title="Fundraisely"
                    description="A fundraising platform built with modern web technologies"
                    tags=vec!["Rust", "Web", "Full Stack"]
                    image="/xforcesolutions/public/images/fundraisely.png"
                />
                <ProjectCard
                    title="XFChess"
                    description="A chess application built with Rust and web technologies"
                    tags=vec!["Rust", "Game", "Web"]
                    image="/xforcesolutions/public/images/xfchess.png"
                />
                <ProjectCard
                    title="Yew Scaffold"
                    description="A scaffolding tool for Yew framework projects"
                    tags=vec!["Rust", "Yew", "Tooling"]
                    image="/xforcesolutions/public/images/yew-scaffold.png"
                />
                <ProjectCard
                    title="XFTerminal"
                    description="A terminal application built with Rust"
                    tags=vec!["Rust", "Terminal", "CLI"]
                    image="/xforcesolutions/public/images/xfterminal.png"
                />
                <ProjectCard
                    title="Solana Contracts"
                    description="Smart contracts deployed on the Solana blockchain"
                    tags=vec!["Rust", "Solana", "Blockchain"]
                    image="/xforcesolutions/public/images/solana-contracts.png"
                />
                <ProjectCard
                    title="Stellar Contracts"
                    description="Smart contracts and applications on the Stellar network"
                    tags=vec!["Rust", "Stellar", "Blockchain"]
                    image="/xforcesolutions/public/images/stellar-contracts.png"
                />
            </div>

            // Call to Action Section
            <div class="mt-16 text-center">
                <div class="bg-black dark:bg-black bg-white dark:bg-black p-8 rounded-lg">
                    <h2 class="text-3xl font-bold mb-4 font-heading reflective-header">
                        "Have a Project in Mind?"
                    </h2>
                    <p class="text-lg text-gray-300 dark:text-gray-300 text-gray-700 dark:text-gray-300 mb-6 font-sans">
                        "Let's work together to bring your vision to life"
                    </p>
                    <a
                        href="/contact"
                        class="inline-block px-8 py-3 bg-primary-600 hover:bg-primary-700 text-white rounded-lg font-semibold transition-colors duration-200"
                    >
                        "Get in Touch"
                    </a>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProjectCard(
    title: &'static str,
    description: &'static str,
    tags: Vec<&'static str>,
    image: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-black dark:bg-black bg-white dark:bg-black p-6 rounded-lg border border-gray-800 dark:border-gray-800 border-gray-200 dark:border-gray-800 hover:border-primary-500 transition-all duration-200 flex flex-col">
            <img
                src=image
                alt=title
                class="w-full h-48 object-cover rounded-lg mb-4 border border-gray-700 dark:border-gray-700 border-gray-300 dark:border-gray-700"
            />
            <h3 class="text-2xl font-bold mb-3 font-heading">{title}</h3>
            <p class="text-gray-300 dark:text-gray-300 text-gray-700 dark:text-gray-300 mb-4 font-sans flex-grow">
                {description}
            </p>
            <div class="flex flex-wrap gap-2">
                {tags.into_iter().map(|tag| view! {
                    <span class="px-3 py-1 bg-primary-900/30 text-primary-400 rounded-full text-sm font-sans">
                        {tag}
                    </span>
                }).collect_view()}
            </div>
        </div>
    }
}

