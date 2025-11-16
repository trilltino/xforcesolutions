use leptos::prelude::*;

#[component]
pub fn Proposal() -> impl IntoView {
    let (current_card, set_current_card) = signal(0);
    
    let cards = vec![
        ProposalCard {
            title: "XForce Terminal: Professional-Grade DeFi Trading Terminal",
            subtitle: "Grant Request: 10,000 USDC",
            content: r#"
                XForce Terminal is a DeFi trading terminal for Solana, built as a solo open-source project. 
                It provides a Bloomberg-style native desktop interface for Solana DeFi, built end-to-end in Rust.
                
                Security is non-custodial: private keys never leave the device, with all transaction signing handled locally. 
                The extensible contract plugin architecture allows easy integration of new Solana programs and DeFi protocols 
                without modifying core code.
                
                The terminal will deliver a professional trading interface with real-time price feeds, candlestick charts, 
                and portfolio tracking integrating with Jupiter aggregator. Furthermore, the custom batch swap router contract 
                executes multiple swaps atomically in a single transaction, reducing fees by up to 90% for complex trades. 
                Beyond trading, XForce Terminal includes SociaFI features through a built-in messaging system using the Braid 
                protocol (CRDT-based) for decentralized, conflict-free messaging between traders.
                
                REST APIs will enable developers to build automated trading strategies, copy-trading systems, and algorithmic 
                trading bots in any programming language.
            "#,
            icon: "🚀",
        },
        ProposalCard {
            title: "Public Goods Contribution & DeFi Innovation",
            subtitle: "Open-Source Infrastructure for Solana",
            content: r#"
                XForce Terminal is the first end-to-end Rust DeFi trading terminal for Solana, built entirely in Rust from 
                Solana programs (Anchor) to the native desktop GUI (egui/eframe) to the backend API (Axum) and web components 
                (Leptos/WASM). This unified stack enables type safety, performance, and code reuse across the entire application. 
                The codebase uses idiomatic Rust patterns, production-level documentation, and a modular architecture with 
                reusable libraries (`lib-solana`, `lib-core`, `lib-web`) that can be published to crates.io and shared across 
                the ecosystem.
                
                This is a novel use of Rust in a language that exponentially is increasing in its use, demand and popularity - 
                by showing the rust ecosystem what is truly possible we can continuously inspire more dev simply by upholding 
                the "Art of the Possible".
                
                XForce Terminal includes SociaFI features through a built-in messaging system using the Braid protocol 
                (CRDT-based) for decentralized, conflict-free messaging between traders, enabling copy-trading and signal 
                sharing without centralized intermediaries. Using braid.org HTTP protocol any prospective or current app that 
                wants to build a social layer can now do so by looking at this code as a reference - Braid makes chat apps, 
                simple, cheap and scalable all while taking advantage and teaching the forefront of computer software programming CRDTS.
                
                The platform serves as developer infrastructure through a comprehensive REST API that enables developers to build 
                automated trading strategies, copy-trading systems, and algorithmic trading bots in any programming language. 
                The extensible contract plugin architecture allows easy integration of new Solana programs and DeFi protocols 
                without modifying core code. AI assistance is integrated using rust-genai, supporting multiple providers 
                (DeepSeek, OpenAI, Anthropic, Gemini) for market analysis, trading strategy guidance, and real-time token 
                information—bringing AI-powered trading support directly into the terminal interface.
                
                As fully open-source XForce Terminal serves as public infrastructure for the Solana DeFi ecosystem. The reusable 
                Rust libraries provide production-ready Solana integration patterns that other developers can adopt, reducing 
                duplication and improving ecosystem code quality. The CRDT messaging implementation offers a reference for 
                decentralized communication in DeFi applications, while the comprehensive REST API democratizes access to 
                sophisticated DeFi tools for trading bot development and algorithmic strategies.
                
                By providing both end-user tools and developer infrastructure, XForce Terminal bridges the gap between professional 
                trading needs and open-source accessibility, ensuring that advanced DeFi capabilities are available to all rather 
                than locked behind proprietary platforms. Every component is designed to be reusable, extensible, and educational, 
                establishing patterns and best practices that benefit the entire Solana developer community.
            "#,
            icon: "🌐",
        },
        ProposalCard {
            title: "Competitive Analysis",
            subtitle: "Unique Position in the Solana Ecosystem",
            content: r#"
                XForce Terminal is the only native desktop application in the Solana DeFi trading space, the only Crypto specific 
                trading terminal, the only one in Rust - but that's not because it can be done but because it's time for the 
                ecosystem to apply the mature frameworks in the to solve production as proof.
                
                The Terminal combines a Bloomberg-style terminal interface with non-custodial security, AI assistance, and social 
                features delivering sub-50ms latency and <10% CPU usage through its Rust-based native architecture. The platform 
                is fully non-custodial—private keys never leave the user's device—and integrates Jupiter aggregator and Pyth 
                Network oracles for optimal pricing.
                
                As an Apache 2.0 licensed open-source project, XForce Terminal embodies FOSS DeFi principles of transparency and 
                verifiability, serving as a public good that enables developers to build upon the codebase, researchers to audit 
                security, and traders to customize without vendor lock-in.
                
                By providing professional-grade, non-custodial trading tools as free and open-source software, XForce Terminal 
                democratizes access to institutional-quality infrastructure while strengthening the decentralized finance ecosystem 
                through community collaboration and transparency.
            "#,
            icon: "⚡",
        },
    ];
    
    let total_cards = cards.len();
    
    let go_to_next = move |_| {
        set_current_card.update(|idx| {
            *idx = (*idx + 1) % total_cards;
        });
    };
    
    let go_to_prev = move |_| {
        set_current_card.update(|idx| {
            *idx = if *idx == 0 { total_cards - 1 } else { *idx - 1 };
        });
    };
    
    let go_to_card = move |idx: usize| {
        set_current_card.set(idx);
    };
    
    view! {
        <div class="max-w-6xl mx-auto py-8 px-4">
            <div class="text-center mb-12">
                <h1 class="text-5xl font-bold mb-4 font-heading bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent">
                    "Proposal"
                </h1>
                <p class="text-xl text-gray-600 dark:text-gray-400 font-sans">
                    "XForce Terminal: Professional-Grade DeFi Trading Terminal for Solana"
                </p>
            </div>

            // Card Container
            <div class="relative">
                // Navigation Buttons
                <button
                    on:click=go_to_prev
                    class="absolute left-0 top-1/2 -translate-y-1/2 -translate-x-4 md:-translate-x-12 z-10 w-12 h-12 md:w-14 md:h-14 rounded-full bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white shadow-lg hover:shadow-xl transition-all duration-300 flex items-center justify-center hover:scale-110"
                    aria-label="Previous card"
                >
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                    </svg>
                </button>
                
                <button
                    on:click=go_to_next
                    class="absolute right-0 top-1/2 -translate-y-1/2 translate-x-4 md:translate-x-12 z-10 w-12 h-12 md:w-14 md:h-14 rounded-full bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white shadow-lg hover:shadow-xl transition-all duration-300 flex items-center justify-center hover:scale-110"
                    aria-label="Next card"
                >
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                    </svg>
                </button>

                // Card Display
                <div class="relative overflow-hidden rounded-2xl">
                    <div 
                        class="flex transition-transform duration-500 ease-in-out"
                        style=move || format!("transform: translateX(-{}%)", current_card.get() * 100)
                    >
                        {cards.into_iter().enumerate().map(|(_idx, card)| {
                            let card_clone = card.clone();
                            view! {
                                <div class="w-full flex-shrink-0 px-2">
                                    <div class="bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 rounded-xl shadow-2xl p-8 md:p-12 border border-gray-200 dark:border-gray-700 min-h-[600px] flex flex-col">
                                        // Card Header
                                        <div class="mb-6">
                                            <div class="text-6xl mb-4">{card_clone.icon}</div>
                                            <h2 class="text-3xl md:text-4xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                                                {card_clone.title}
                                            </h2>
                                            <div class="inline-block px-4 py-2 bg-gradient-to-r from-blue-500 to-purple-500 text-white rounded-full text-sm font-semibold shadow-lg">
                                                {card_clone.subtitle}
                                            </div>
                                        </div>
                                        
                                        // Card Content
                                        <div class="flex-grow overflow-y-auto">
                                            <div class="prose prose-lg dark:prose-invert max-w-none">
                                                <p class="text-gray-700 dark:text-gray-300 leading-relaxed whitespace-pre-line font-sans">
                                                    {card_clone.content}
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                // Indicator Dots
                <div class="flex justify-center mt-8 space-x-3">
                    {(0..total_cards).map(|idx| {
                        let is_active = move || current_card.get() == idx;
                        let card_idx = idx;
                        view! {
                            <button
                                on:click=move |_| go_to_card(card_idx)
                                class=move || {
                                    if is_active() {
                                        "w-3 h-3 rounded-full bg-gradient-to-r from-blue-500 to-purple-500 shadow-lg transition-all duration-300"
                                    } else {
                                        "w-3 h-3 rounded-full bg-gray-400 dark:bg-gray-600 hover:bg-gray-500 dark:hover:bg-gray-500 transition-all duration-300"
                                    }
                                }
                                aria-label=format!("Go to card {}", idx + 1)
                            />
                        }
                    }).collect::<Vec<_>>()}
                </div>

                // Card Counter
                <div class="text-center mt-6">
                    <span class="text-sm text-gray-500 dark:text-gray-400 font-sans">
                        {move || format!("{} / {}", current_card.get() + 1, total_cards)}
                    </span>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone)]
struct ProposalCard {
    title: &'static str,
    subtitle: &'static str,
    content: &'static str,
    icon: &'static str,
}

