use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto">
            <h1 class="text-4xl md:text-5xl font-bold mb-8 text-center font-heading">
                "Proof of Work"
            </h1>
            
            <p class="text-xl text-gray-300 dark:text-gray-300 text-gray-700 dark:text-gray-300 text-center mb-12 font-sans">
                "Explore our portfolio of innovative technology solutions"
            </p>

            // Six symmetrical rows: Photo on left, Text box on right
            <div class="space-y-8">
                // Row 1: Fundraisely
                <div class="grid md:grid-cols-2 gap-8">
                    <ProjectImage
                        title="Fundraisely"
                        tags=vec!["Rust", "Web", "Full Stack"]
                        image="/xforcesolutions/public/images/fundraisely.png"
                    />
                    <ProjectTextBox
                        title="Fundraisely"
                        content="A comprehensive fundraising platform built with modern web technologies. Features include campaign management, donation processing, real-time analytics, and social media integration. Built end-to-end in Rust for maximum performance and security."
                    />
                </div>

                // Row 2: Yew Scaffold
                <div class="grid md:grid-cols-2 gap-8">
                    <ProjectImage
                        title="Yew Scaffold"
                        tags=vec!["Rust", "Yew", "Tooling"]
                        image="/xforcesolutions/public/images/yew-scaffold.png"
                    />
                    <ProjectTextBox
                        title="Yew Scaffold"
                        content="A scaffolding tool for Yew framework projects that generates boilerplate code and project structure. Streamlines the setup process for new Yew applications with best practices, routing, and component templates built-in."
                    />
                </div>

                // Row 3: XFTerminal
                <div class="grid md:grid-cols-2 gap-8">
                    <ProjectImage
                        title="XFTerminal"
                        tags=vec!["Rust", "Terminal", "CLI"]
                        image="/xforcesolutions/public/images/xfterminal.png"
                    />
                    <ProjectTextBox
                        title="XFTerminal"
                        content="A professional-grade DeFi trading terminal for Solana blockchain. Features Bloomberg-style native desktop interface, real-time price feeds, candlestick charts, and batch swap router contracts. Built entirely in Rust with non-custodial security."
                    />
                </div>

                // Row 4: XFChess
                <div class="grid md:grid-cols-2 gap-8">
                    <ProjectImage
                        title="XFChess"
                        tags=vec!["Rust", "Game", "Web"]
                        image="/xforcesolutions/public/images/xfchess.png"
                    />
                    <ProjectTextBox
                        title="XFChess"
                        content="A full-featured chess application built with Rust and web technologies. Includes multiplayer support, game analysis, move validation, and tournament modes. Demonstrates advanced state management and real-time communication patterns."
                    />
                </div>

                // Row 5: Solana Contracts
                <div class="grid md:grid-cols-2 gap-8">
                    <ProjectImage
                        title="Solana Contracts"
                        tags=vec!["Rust", "Solana", "Blockchain"]
                        image="/xforcesolutions/public/images/solana-contracts.png"
                    />
                    <ProjectTextBox
                        title="Solana Contracts"
                        content="Production-grade Solana smart contracts for batch token swaps. Features atomic execution, slippage protection, Jupiter integration, and comprehensive security validations. Deployed on Solana devnet with full open-source documentation."
                    />
                </div>

                // Row 6: Stellar Contracts
                <div class="grid md:grid-cols-2 gap-8">
                    <ProjectImage
                        title="Stellar Contracts"
                        tags=vec!["Rust", "Stellar", "Blockchain"]
                        image="/xforcesolutions/public/images/stellar-contracts.png"
                    />
                    <ProjectTextBox
                        title="Stellar Contracts"
                        content="Smart contracts and applications deployed on the Stellar network. Includes token issuance, multi-signature accounts, and payment processing. Demonstrates cross-chain capabilities and Stellar's efficient consensus mechanism integration."
                    />
                </div>
            </div>

            // Call to Action Section
            <div class="mt-16 text-center">
                <div class="bg-black dark:bg-black bg-white dark:bg-black p-8 rounded-lg">
                    <h2 class="text-3xl font-bold mb-4 font-heading">
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
fn ProjectImage(
    title: &'static str,
    tags: Vec<&'static str>,
    image: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg border border-gray-200 dark:border-gray-800 hover:shadow-xl transition-all duration-200 flex flex-col">
            <img
                src=image
                alt=title
                class="w-full object-contain rounded-lg mb-4 border border-gray-300 dark:border-gray-700 max-h-[500px]"
            />
            <div class="flex items-center justify-between">
                <h3 class="text-2xl font-bold font-heading text-gray-900 dark:text-white">{title}</h3>
                <div class="flex flex-wrap gap-2">
                    {tags.into_iter().map(|tag| view! {
                        <span class="px-3 py-1 bg-primary-100 dark:bg-primary-900/30 text-primary-700 dark:text-primary-400 rounded-full text-sm font-sans">
                            {tag}
                        </span>
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProjectTextBox(
    title: &'static str,
    content: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg border border-gray-200 dark:border-gray-800 hover:shadow-xl transition-all duration-200 flex flex-col h-full justify-center">
            <h3 class="text-2xl font-bold mb-4 font-heading text-gray-900 dark:text-white">{title}</h3>
            <p class="text-gray-700 dark:text-gray-300 font-sans leading-relaxed">
                {content}
            </p>
        </div>
    }
}

