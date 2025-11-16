use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Status() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto py-8 px-4">
            // Hero Section with XFTerminal Image
            <div class="relative mb-12">
                <div class="relative h-[400px] md:h-[500px] rounded-lg overflow-hidden border border-red-900/50">
                    <img 
                        src="/xforcesolutions/public/images/xfterminal-cover.png" 
                        alt="XFTerminal - Access SolanaDeFi Anywhere" 
                        class="w-full h-full object-cover"
                    />
                    <div class="absolute inset-0 bg-gradient-to-t from-black/95 via-black/70 to-black/40"></div>
                    <div class="absolute inset-0 flex items-end justify-start p-8">
                        <div class="text-white">
                            <h1 class="text-4xl md:text-6xl font-bold mb-4 font-heading">
                                <span class="text-red-500">"XF"</span>
                                <span class="text-white">"Terminal"</span>
                            </h1>
                            <p class="text-xl md:text-2xl text-gray-300 font-sans mb-2">
                                "Access SolanaDeFi Anywhere"
                            </p>
                            <p class="text-sm md:text-base text-gray-400 font-mono">
                                "Production-grade DeFi trading terminal built entirely in Rust"
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            // Status Overview
            <div class="bg-white dark:bg-gray-900 p-8 rounded-lg shadow-lg mb-8 border border-gray-200 dark:border-gray-800">
                <h2 class="text-3xl font-bold mb-6 font-heading text-gray-900 dark:text-white">
                    "System Status & Features"
                </h2>
                <p class="text-gray-700 dark:text-gray-300 mb-6 font-sans leading-relaxed">
                    "XFTerminal is the first end-to-end Rust DeFi trading terminal for Solana, featuring a unified stack from smart contracts to desktop GUI to web components. All components share type safety, ensuring consistency and reliability across the entire application."
                </p>
            </div>

            // Core Features Grid
            <div class="mb-12">
                <h2 class="text-3xl font-bold mb-8 text-center font-heading text-gray-900 dark:text-white">
                    "Core Features"
                </h2>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <FeatureCard
                        icon="🖥️"
                        title="Bloomberg-Style Desktop Interface"
                        description="Native desktop application with professional trading terminal UI built using egui/eframe for optimal performance and user experience."
                    />
                    <FeatureCard
                        icon="📊"
                        title="Real-Time Price Feeds"
                        description="Live market data and price updates directly from Solana blockchain, keeping you informed of market movements in real-time."
                    />
                    <FeatureCard
                        icon="📈"
                        title="Candlestick Charts"
                        description="Advanced charting capabilities with candlestick visualization for technical analysis and trading decisions."
                    />
                    <FeatureCard
                        icon="🔄"
                        title="Batch Swap Router Contracts"
                        description="Atomic batch token swaps on Solana, enabling multiple swaps in a single transaction with significant fee savings."
                    />
                    <FeatureCard
                        icon="🤖"
                        title="AI-Powered Trading Assistant"
                        description="Integrated AI assistance using rust-genai with multi-provider support (DeepSeek, OpenAI, Anthropic, Gemini) for market analysis and strategy guidance."
                    />
                    <FeatureCard
                        icon="💬"
                        title="Social Messaging System"
                        description="Decentralized messaging using Braid Protocol (CRDT-based) for copy-trading, signal sharing, and trader communication without centralized intermediaries."
                    />
                    <FeatureCard
                        icon="🔌"
                        title="REST API for Trading Bots"
                        description="Comprehensive REST API enabling developers to build automated trading strategies, copy-trading systems, and algorithmic trading bots in any language."
                    />
                    <FeatureCard
                        icon="🔐"
                        title="Non-Custodial Security"
                        description="Your keys, your funds. Full control over your assets with secure wallet integration and transaction signing."
                    />
                    <FeatureCard
                        icon="🌐"
                        title="Web Wallet Interface"
                        description="Browser-based wallet helper built with Leptos/WASM for seamless web integration and accessibility."
                    />
                </div>
            </div>

            // Technology Stack
            <div class="mb-12">
                <h2 class="text-3xl font-bold mb-8 text-center font-heading text-gray-900 dark:text-white">
                    "Technology Stack"
                </h2>
                <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <TechCard
                        category="Core Language"
                        technologies=vec!["Rust"]
                        color="bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400"
                    />
                    <TechCard
                        category="Desktop GUI"
                        technologies=vec!["egui", "eframe"]
                        color="bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400"
                    />
                    <TechCard
                        category="Web Framework"
                        technologies=vec!["Leptos", "WASM"]
                        color="bg-purple-100 dark:bg-purple-900/30 text-purple-700 dark:text-purple-400"
                    />
                    <TechCard
                        category="Backend Server"
                        technologies=vec!["Axum", "Tokio"]
                        color="bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400"
                    />
                    <TechCard
                        category="Blockchain"
                        technologies=vec!["Solana SDK", "Anchor"]
                        color="bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-400"
                    />
                    <TechCard
                        category="Database"
                        technologies=vec!["SQLite"]
                        color="bg-indigo-100 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-400"
                    />
                    <TechCard
                        category="Authentication"
                        technologies=vec!["JWT"]
                        color="bg-pink-100 dark:bg-pink-900/30 text-pink-700 dark:text-pink-400"
                    />
                    <TechCard
                        category="Messaging"
                        technologies=vec!["Braid Protocol", "CRDT"]
                        color="bg-cyan-100 dark:bg-cyan-900/30 text-cyan-700 dark:text-cyan-400"
                    />
                    <TechCard
                        category="AI Integration"
                        technologies=vec!["rust-genai"]
                        color="bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-400"
                    />
                    <TechCard
                        category="Styling"
                        technologies=vec!["Tailwind CSS"]
                        color="bg-teal-100 dark:bg-teal-900/30 text-teal-700 dark:text-teal-400"
                    />
                </div>
            </div>

            // Architecture Highlights
            <div class="mb-12">
                <h2 class="text-3xl font-bold mb-8 text-center font-heading text-gray-900 dark:text-white">
                    "Architecture Highlights"
                </h2>
                <div class="grid md:grid-cols-2 gap-6">
                    <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg border border-gray-200 dark:border-gray-800">
                        <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                            "🔧 Modular Library Architecture"
                        </h3>
                        <p class="text-gray-700 dark:text-gray-300 font-sans">
                            "Reusable libraries designed for crates.io publication: lib-solana (blockchain integration), lib-core (data models), lib-web (server code), lib-auth (authentication), and lib-utils (shared utilities)."
                        </p>
                    </div>
                    <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg border border-gray-200 dark:border-gray-800">
                        <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                            "🔄 Unified Type Safety"
                        </h3>
                        <p class="text-gray-700 dark:text-gray-300 font-sans">
                            "Shared types across Solana programs, desktop app, backend API, and web components ensure consistency and eliminate type-related bugs across the entire stack."
                        </p>
                    </div>
                    <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg border border-gray-200 dark:border-gray-800">
                        <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                            "🚀 Production-Ready Patterns"
                        </h3>
                        <p class="text-gray-700 dark:text-gray-300 font-sans">
                            "Idiomatic Rust code with comprehensive error handling, logging, documentation, and modular architecture suitable for production deployment and open-source contribution."
                        </p>
                    </div>
                    <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg border border-gray-200 dark:border-gray-800">
                        <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                            "🌍 Public Infrastructure"
                        </h3>
                        <p class="text-gray-700 dark:text-gray-300 font-sans">
                            "Fully open-source platform serving as public infrastructure for the Solana DeFi ecosystem, providing reusable patterns and best practices for the community."
                        </p>
                    </div>
                </div>
            </div>

            // Smart Contract Features
            <div class="mb-12">
                <h2 class="text-3xl font-bold mb-8 text-center font-heading text-gray-900 dark:text-white">
                    "Smart Contract Capabilities"
                </h2>
                <div class="bg-gradient-to-r from-purple-50 to-blue-50 dark:from-purple-900/20 dark:to-blue-900/20 p-8 rounded-lg shadow-lg">
                    <div class="grid md:grid-cols-2 gap-6">
                        <div>
                            <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                                "⚡ Atomic Execution"
                            </h3>
                            <p class="text-gray-700 dark:text-gray-300 font-sans">
                                "All swaps in a batch either succeed together or fail together. No partial executions - your funds are always safe."
                            </p>
                        </div>
                        <div>
                            <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                                "💰 Fee Reduction"
                            </h3>
                            <p class="text-gray-700 dark:text-gray-300 font-sans">
                                "Pay transaction fees once instead of multiple times. Execute up to 10 swaps in a single transaction, saving 60-90% on fees."
                            </p>
                        </div>
                        <div>
                            <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                                "🛡️ Slippage Protection"
                            </h3>
                            <p class="text-gray-700 dark:text-gray-300 font-sans">
                                "Built-in validation ensures you receive the minimum expected output. Maximum slippage tolerance: 5%."
                            </p>
                        </div>
                        <div>
                            <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                                "🔗 Jupiter Integration"
                            </h3>
                            <p class="text-gray-700 dark:text-gray-300 font-sans">
                                "Integrates with Jupiter aggregator for best-price routing across all DEXes on Solana."
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            // Developer Features
            <div class="mb-12">
                <h2 class="text-3xl font-bold mb-8 text-center font-heading text-gray-900 dark:text-white">
                    "Developer Infrastructure"
                </h2>
                <div class="grid md:grid-cols-3 gap-6">
                    <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg border border-gray-200 dark:border-gray-800">
                        <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                            "📚 Extensible Contract Plugin Architecture"
                        </h3>
                        <p class="text-gray-700 dark:text-gray-300 font-sans">
                            "Easy integration of new Solana programs and DeFi protocols without modifying core code. Plugin-based architecture for maximum flexibility."
                        </p>
                    </div>
                    <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg border border-gray-200 dark:border-gray-800">
                        <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                            "🔌 REST API"
                        </h3>
                        <p class="text-gray-700 dark:text-gray-300 font-sans">
                            "Comprehensive REST API enabling developers to build automated trading strategies, copy-trading systems, and algorithmic trading bots in any programming language."
                        </p>
                    </div>
                    <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg border border-gray-200 dark:border-gray-800">
                        <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                            "📖 Open Source & Documented"
                        </h3>
                        <p class="text-gray-700 dark:text-gray-300 font-sans">
                            "Fully open-source with comprehensive documentation, serving as reference implementation for Rust DeFi development patterns and best practices."
                        </p>
                    </div>
                </div>
            </div>

            // Call to Action
            <div class="bg-black dark:bg-black p-8 rounded-lg shadow-lg text-center border border-red-900/50">
                <h2 class="text-3xl font-bold mb-4 font-heading text-white">
                    "Ready to Explore?"
                </h2>
                <p class="text-lg text-gray-300 mb-6 font-sans">
                    "Discover more about XFTerminal's architecture, contracts, and documentation"
                </p>
                <div class="flex flex-wrap justify-center gap-4">
                    <A
                        href="/architecture"
                        attr:class="inline-block px-6 py-3 bg-red-600 hover:bg-red-700 text-white rounded-lg font-semibold transition-colors duration-200"
                    >
                        "View Architecture"
                    </A>
                    <A
                        href="/contracts"
                        attr:class="inline-block px-6 py-3 bg-red-600 hover:bg-red-700 text-white rounded-lg font-semibold transition-colors duration-200"
                    >
                        "Smart Contracts"
                    </A>
                    <A
                        href="/documentation"
                        attr:class="inline-block px-6 py-3 bg-red-600 hover:bg-red-700 text-white rounded-lg font-semibold transition-colors duration-200"
                    >
                        "Documentation"
                    </A>
                </div>
            </div>
        </div>
    }
}

#[component]
fn FeatureCard(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg border border-gray-200 dark:border-gray-800 hover:shadow-xl transition-all duration-200 hover:border-red-500/50">
            <div class="text-4xl mb-4">{icon}</div>
            <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                {title}
            </h3>
            <p class="text-gray-700 dark:text-gray-300 font-sans">
                {description}
            </p>
        </div>
    }
}

#[component]
fn TechCard(
    category: &'static str,
    technologies: Vec<&'static str>,
    color: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg border border-gray-200 dark:border-gray-800">
            <h3 class="text-lg font-bold mb-3 font-heading text-gray-900 dark:text-white">
                {category}
            </h3>
            <div class="flex flex-wrap gap-2">
                {technologies.into_iter().map(|tech| view! {
                    <span class=format!("px-3 py-1 rounded-full text-sm font-sans font-semibold {}", color)>
                        {tech}
                    </span>
                }).collect_view()}
            </div>
        </div>
    }
}

