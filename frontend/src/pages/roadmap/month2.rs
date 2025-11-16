use leptos::prelude::*;

#[component]
pub fn Month2() -> impl IntoView {
    view! {
        <div class="max-w-6xl mx-auto py-8 px-4">
            // Header
            <div class="text-center mb-12">
                <div class="inline-flex items-center justify-center w-16 h-16 bg-gradient-to-br from-red-500 to-red-700 rounded-full mb-4">
                    <span class="text-white font-bold text-2xl">"2"</span>
                </div>
                <h1 class="text-5xl font-bold mb-4 font-heading bg-gradient-to-r from-red-400 to-red-600 bg-clip-text text-transparent">
                    "Milestone 2 - Month 2"
                </h1>
                <p class="text-2xl text-gray-300 mb-2 font-sans">"$2,500"</p>
                <p class="text-lg text-gray-400 font-sans">
                    "Advanced features, API infrastructure, community growth, and ecosystem integration"
                </p>
            </div>

            // Main Content
            <div class="space-y-8">
                // Advanced Features Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"Advanced Features & Beta v0.2.0 Release"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Integrate AI assistant using rust-genai (DeepSeek, OpenAI, Anthropic, Gemini support)"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Enhance SociaFI messaging with full CRDT implementation and conflict resolution"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Advanced charting: candlestick charts, technical indicators, multiple timeframes"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Portfolio analytics: P&L tracking, performance metrics, trade history analysis"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Batch swap router contract deployment and integration"</span>
                        </li>
                    </ul>
                </div>

                // REST API Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"REST API Development"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Design and implement REST API endpoints for trading operations"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Example trading bot integrations (Python, JavaScript, Rust)"</span>
                        </li>
                    </ul>
                </div>

                // Software Development Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"Software Development & Performance"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Address Beta v0.1.0 user feedback and bug fixes"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Performance optimizations: reduce memory footprint, improve WebSocket latency"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Code review, dependency scanning"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Enhanced error handling and user feedback systems"</span>
                        </li>
                    </ul>
                </div>

                // Community Growth Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"Community Growth & Partnerships"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Launch developer program with API access for early adopters"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Engage with Solana developer communities (SuperteamUK)"</span>
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
                            <span>"API documentation: endpoints, authentication, rate limits, examples"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Developer guide: building trading bots, integrating with XForce Terminal"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Publish 3 technical blog posts:"</span>
                        </li>
                        <li class="ml-8 space-y-2 text-gray-400">
                            <div>"• Building Trading Bots with XForce Terminal API"</div>
                            <div>"• CRDT Conflict Resolution in Decentralized Messaging"</div>
                            <div>"• Rust-GenAI Integration for DeFi Trading Assistants"</div>
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
                                <span>"Beta stability, Beta v0.2.0 downloads end of month with more features: xx+ users"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"Waitlist signups: xxx+ total signups (2.5x growth)"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"Blog posts: 3 published on Substack"</span>
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
                                <span>"Beta v0.2.0 released with AI assistant and advanced features (testnet)"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"REST API functional and documented for developer use"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Technical content demonstrates advanced capabilities and attracts developers"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Community growth shows sustainable engagement and interest"</span>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}

