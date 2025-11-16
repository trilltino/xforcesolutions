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
                        tags=vec!["Rust", "Leptos", "Axum", "Full Stack"]
                        image="/xforcesolutions/public/images/fundraisely.png"
                    />
                    <ProjectTextBox
                        title="Fundraisely"
                        bullets=vec![
                            "Comprehensive fundraising platform built end-to-end in Rust for maximum performance and security",
                            "Full-stack web application with Leptos for reactive UI and Axum for high-performance backend API",
                            "Campaign management system with real-time analytics and donation processing capabilities",
                            "Social media integration for campaign promotion and donor engagement",
                            "Production-ready Rust web development patterns with idiomatic code architecture",
                            "Type-safe frontend and backend with shared types across the entire application stack"
                        ]
                    />
                </div>

                // Row 2: Yew Scaffold
                <div class="grid md:grid-cols-2 gap-8">
                    <ProjectImage
                        title="Yew Scaffold"
                        tags=vec!["Rust", "Yew", "Tooling", "CLI"]
                        image="/xforcesolutions/public/images/yew-scaffold.png"
                    />
                    <ProjectTextBox
                        title="Yew Scaffold"
                        bullets=vec![
                            "Production-ready scaffolding tool for Yew framework projects that generates boilerplate code",
                            "Streamlines the setup process for new Rust web applications with best practices built-in",
                            "Includes routing templates and component structure for rapid development",
                            "wasm-bindgen integration for transaction signing with multi-wallet support (Phantom, Solflare, Backpack, Sollet, Ledger)",
                            "JavaScript interop via wasm-bindgen inline functions enabling secure wallet adapter bindings for signing Solana transactions",
                            "Transaction serialization/deserialization with base64 encoding for seamless wallet communication",
                            "Type-safe Rust bindings to wallet provider APIs with error handling and async transaction signing workflows"
                        ]
                    />
                </div>

                // Row 3: XFTerminal
                <div class="grid md:grid-cols-2 gap-8">
                    <ProjectImage
                        title="XFTerminal"
                        tags=vec!["Rust", "DeFi", "Solana", "Systems"]
                        image="/xforcesolutions/public/images/xfterminal.png"
                    />
                    <ProjectTextBox
                        title="XFTerminal"
                        bullets=vec![
                            "First end-to-end Rust DeFi trading terminal for Solana—built entirely in Rust from smart contracts to desktop GUI to web components",
                            "Bloomberg-style native desktop interface with egui/eframe featuring multiple screens: terminal, wallet, live charts, AI chat, messaging, swap history",
                            "Jupiter aggregator integration for optimal swap routing with quote, price, and swap APIs",
                            "Pyth oracle integration for real-time price feeds with candle aggregation for charting",
                            "Braid Protocol (CRDT-based) messaging system for decentralized copy-trading with friend management and AI bot assistance",
                            "Modular architecture with reusable libraries: lib-solana (blockchain), lib-core (data models), lib-web (API server), lib-auth (security)",
                            "Comprehensive REST API with endpoints for market data, swaps, staking, transactions, and contract plugins"
                        ]
                    />
                </div>

                // Row 4: XFChess
                <div class="grid md:grid-cols-2 gap-8">
                    <ProjectImage
                        title="XFChess"
                        tags=vec!["Rust", "Bevy", "Game Engine", "Chess AI"]
                        image="/xforcesolutions/public/images/xfchess.png"
                    />
                    <ProjectTextBox
                        title="XFChess"
                        bullets=vec![
                            "Bevy-powered chess game engine with 3D graphics rendering using GLB models and sound effects",
                            "Complete minimax-based chess AI with alpha-beta pruning, transposition tables (80MB hash table), and iterative deepening search",
                            "Bevy 0.17 ECS architecture with egui inspector for real-time component debugging and state visualization",
                            "Chess engine library with bitboard representation, precalculated move tables, and move ordering optimizations",
                            "Web support with WASM compilation enabling browser-based gameplay with Bevy's web backend",
                            "Visual debugging tools with bevy-inspector-egui for inspecting game state, components, and resources in real-time",
                            "Performance optimizations: ~500K-1M nodes per second, 6-10 ply search depth, estimated 1800-2000 ELO strength"
                        ]
                    />
                </div>

                // Row 5: Solana Contracts
                <div class="grid md:grid-cols-2 gap-8">
                    <ProjectImage
                        title="Solana Contracts"
                        tags=vec!["Rust", "Anchor", "Solana", "Smart Contracts"]
                        image="/xforcesolutions/public/images/solana-contracts.png"
                    />
                    <ProjectTextBox
                        title="Solana Contracts"
                        bullets=vec![
                            "Production-grade Solana smart contracts built with Anchor framework for batch token swaps (up to 10 swaps per transaction)",
                            "Atomic execution ensures all swaps in a batch succeed or all fail together—eliminates partial execution risk",
                            "Jupiter integration for best-price routing across all Solana DEXs with automatic route optimization",
                            "Slippage protection and validation with configurable tolerance levels to prevent bad trades from volatility",
                            "Comprehensive security validations with event emission for transaction tracking and audit trails",
                            "Type-safe Rust client library with error handling for seamless integration into applications",
                            "Deployed on Solana devnet (Program ID: HS63bw1V1qTM5uWf92q3uaFdqogrc4SN9qUJSR8aqBMx) with full documentation"
                        ]
                    />
                </div>

                // Row 6: Stellar Contracts
                <div class="grid md:grid-cols-2 gap-8">
                    <ProjectImage
                        title="Stellar Contracts"
                        tags=vec!["Rust", "Stellar", "Blockchain", "DeFi"]
                        image="/xforcesolutions/public/images/stellar-contracts.png"
                    />
                    <ProjectTextBox
                        title="Stellar Contracts"
                        bullets=vec![
                            "Smart contracts and applications deployed on the Stellar blockchain network",
                            "Token issuance capabilities with custom asset configuration and distribution mechanisms",
                            "Multi-signature account management for enhanced security and governance",
                            "Payment processing with efficient transaction handling and low-cost operations",
                            "Cross-chain capabilities demonstrating Stellar's interoperability features",
                            "Integration with Stellar's efficient consensus mechanism (Federated Byzantine Agreement)",
                            "Production-ready DeFi applications leveraging Stellar's fast settlement times"
                        ]
                    />
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
    bullets: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg border border-gray-200 dark:border-gray-800 hover:shadow-xl transition-all duration-200 flex flex-col h-full justify-center">
            <h3 class="text-2xl font-bold mb-4 font-heading text-gray-900 dark:text-white">{title}</h3>
            <ul class="space-y-2 text-gray-700 dark:text-gray-300 font-sans leading-relaxed list-disc list-inside">
                {bullets.into_iter().map(|bullet| view! {
                    <li>{bullet}</li>
                }).collect_view()}
            </ul>
        </div>
    }
}

