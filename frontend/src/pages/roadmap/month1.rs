use leptos::prelude::*;

#[component]
pub fn Month1() -> impl IntoView {
    view! {
        <div class="max-w-6xl mx-auto py-8 px-4">
            // Header
            <div class="text-center mb-12">
                <div class="inline-flex items-center justify-center w-16 h-16 bg-gradient-to-br from-red-500 to-red-700 rounded-full mb-4">
                    <span class="text-white font-bold text-2xl">"1"</span>
                </div>
                <h1 class="text-5xl font-bold mb-4 font-heading bg-gradient-to-r from-red-400 to-red-600 bg-clip-text text-transparent">
                    "Milestone 1 - Month 1"
                </h1>
                <p class="text-2xl text-gray-300 mb-2 font-sans">"$2,500"</p>
                <p class="text-lg text-gray-400 font-sans">
                    "UI/UX polish, minimal Beta release, community engagement, and documentation"
                </p>
            </div>

            // Main Content
            <div class="space-y-8">
                // UI/UX Enhancements Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"UI/UX Enhancements & Minimal Beta Release"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Polish core trading interface (terminal, wallet, transactions screens)"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Release minimal Beta v0.1.0 with core features: authentication, wallet connection, basic swaps, real-time price feeds, messaging and batch swapping (End of Month 1)"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Performance optimizations for Beta release"</span>
                        </li>
                    </ul>
                </div>

                // SuperteamUK Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"SuperteamUK Participation"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Attend SuperteamUK events in London"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Present XForce Terminal demos"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Network with Solana developers and ecosystem participants"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Collect feedback and potential partnerships"</span>
                        </li>
                    </ul>
                </div>

                // Documentation Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"Documentation & Content"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Complete usage manual covering installation, wallet setup, trading workflows, and troubleshooting"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Complete feature manual documenting all Beta features, keyboard shortcuts, and advanced functionality"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Publish 3 technical blog posts:"</span>
                        </li>
                        <li class="ml-8 space-y-2 text-gray-400">
                            <div>"• Building Native DeFi Apps with Rust"</div>
                            <div>"• Non-Custodial Architecture in XForce Terminal"</div>
                            <div>"• CRDT-Based Messaging for Decentralized Trading"</div>
                        </li>
                    </ul>
                </div>

                // Waitlist Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"Waitlist & Community Building"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Launch waitlist signup page"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Implement waitlist management system"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Create landing page with Beta signup CTA"</span>
                        </li>
                    </ul>
                </div>

                // Metrics & Success Criteria
                <div class="grid md:grid-cols-2 gap-8">
                    <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                        <h3 class="text-2xl font-bold text-white mb-6 font-heading">"Metrics"</h3>
                        <ul class="space-y-3 text-gray-300 font-sans">
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"Beta downloads: xx +users"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"Waitlist signups: xx signups"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"Blog posts: 3 published (Substack)"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"SuperteamUK: connections, connections, connections"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"Documentation: usage and install manuals completed"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"Beta stability: stable and observable beta"</span>
                            </li>
                        </ul>
                    </div>

                    <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                        <h3 class="text-2xl font-bold text-white mb-6 font-heading">"Success Criteria"</h3>
                        <ul class="space-y-3 text-gray-300 font-sans">
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Minimal Beta successfully released and accessible to early users (testnet)"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Active waitlist demonstrating community interest"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Documentation enables users to onboard independently"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"SuperteamUK participation establishes initial ecosystem presence"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Technical content establishes thought leadership in Rust-native DeFi development"</span>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}

