use leptos::prelude::*;

#[component]
pub fn Month4() -> impl IntoView {
    view! {
        <div class="max-w-6xl mx-auto py-8 px-4">
            // Header
            <div class="text-center mb-12">
                <div class="inline-flex items-center justify-center w-16 h-16 bg-gradient-to-br from-red-500 to-red-700 rounded-full mb-4">
                    <span class="text-white font-bold text-2xl">"4"</span>
                </div>
                <h1 class="text-5xl font-bold mb-4 font-heading bg-gradient-to-r from-red-400 to-red-600 bg-clip-text text-transparent">
                    "Milestone 4 - Month 4"
                </h1>
                <p class="text-2xl text-gray-300 mb-2 font-sans">"$2,500"</p>
                <p class="text-lg text-gray-400 font-sans">
                    "Mainnet Beta Deployment & Production Launch"
                </p>
            </div>

            // Main Content
            <div class="space-y-8">
                // Mainnet Deployment Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"Mainnet Beta Deployment & Production Launch (End of Month)"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Deploy batch swap router contract to Solana mainnet"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Mainnet integration: contract addresses, RPC endpoints, transaction monitoring"</span>
                        </li>
                    </ul>
                </div>

                // Production Release Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"Production Release v1.0.0"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Mainnet Beta Launch production-ready version with all core features"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Performance optimizations for mainnet workloads"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Enhanced security measures for production environment"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Mainnet-specific error handling and recovery mechanisms"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Production monitoring and alerting systems"</span>
                        </li>
                    </ul>
                </div>

                // Infrastructure Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"Infrastructure & Scaling"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Backend infrastructure scaling for increased load"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Database optimization for transaction history and user data"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"WebSocket connection pooling and load balancing"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Backup and disaster recovery procedures"</span>
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
                        <h2 class="text-3xl font-bold text-white font-heading">"Software Development & Production Hardening"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Address all bugs from devnet testing"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Production logging and monitoring integration"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"User analytics and usage tracking (privacy-preserving)"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Performance benchmarking and optimization"</span>
                        </li>
                    </ul>
                </div>

                // Launch & Marketing Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5.882V19.24a1.76 1.76 0 01-3.417.592l-2.147-6.15M18 13a3 3 0 100-6M5.436 13.683A4.001 4.001 0 017 6h1.832c4.1 0 7.625-1.234 9.168-3v14c-1.543-1.766-5.067-3-9.168-3H7a3.988 3.988 0 01-1.564-.317z"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"Launch & Marketing"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Official launch announcement and press release"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Launch event or virtual demo for community"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Launch campaign: social media, crypto communities, newsletters"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Early adopter program with incentives"</span>
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
                            <span>"Publish 3 technical blog posts:"</span>
                        </li>
                        <li class="ml-8 space-y-2 text-gray-400">
                            <div>"• From Devnet to Mainnet: XForce Terminal Production Launch"</div>
                            <div>"• Scaling Rust Applications for DeFi: Lessons Learned"</div>
                            <div>"• Building Production-Grade Solana Applications"</div>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Launch video: product demo and feature walkthrough"</span>
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
                                <span>"Production v1.0.0 downloads: XXX+ users"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"Waitlist signups: xxx+ total signups"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"Active users: 200+ monthly active users"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"API developers: developers building with API"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"Blog posts: 3 published on Substack"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"Production stability: <1% crash rate, 99%+ uptime"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"Production Observability"</span>
                            </li>
                        </ul>
                    </div>

                    <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                        <h3 class="text-2xl font-bold text-white mb-6 font-heading">"Success Criteria (End of Month)"</h3>
                        <ul class="space-y-3 text-gray-300 font-sans">
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Ready for Mainnet"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Batch swap router contract successfully deployed and operational on mainnet"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Production v1.0.0 successfully launched with stable performance"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Infrastructure scales to support initial user base without degradation"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Active mainnet usage with real transactions and trading activity"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Strong launch momentum with community engagement and media coverage"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Developer ecosystem wanting to learn how to create modular components"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Technical content establishes production deployment expertise"</span>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}

