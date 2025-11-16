use leptos::prelude::*;

#[component]
pub fn Month3() -> impl IntoView {
    view! {
        <div class="max-w-6xl mx-auto py-8 px-4">
            // Header
            <div class="text-center mb-12">
                <div class="inline-flex items-center justify-center w-16 h-16 bg-gradient-to-br from-red-500 to-red-700 rounded-full mb-4">
                    <span class="text-white font-bold text-2xl">"3"</span>
                </div>
                <h1 class="text-5xl font-bold mb-4 font-heading bg-gradient-to-r from-red-400 to-red-600 bg-clip-text text-transparent">
                    "Milestone 3 - Month 3"
                </h1>
                <p class="text-2xl text-gray-300 mb-2 font-sans">"$2,500"</p>
                <p class="text-lg text-gray-400 font-sans">
                    "Devnet deployment, contract testing, security hardening, and production readiness"
                </p>
            </div>

            // Main Content
            <div class="space-y-8">
                // Devnet Deployment Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"Devnet Deployment & Contract Integration (End of Month 3)"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Deploy batch swap router contract to Solana devnet"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Integrate devnet contract addresses into XForce Terminal"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"End-to-end testing: wallet connection, swaps, batch swaps, messaging on devnet"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Devnet faucet integration for test SOL distribution"</span>
                        </li>
                    </ul>
                </div>

                // Beta v0.3.0 Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"Beta v0.3.0 Release with Devnet Support"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Release with full devnet functionality"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Network selector (devnet/mainnet toggle)"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Devnet transaction explorer integration"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Enhanced error handling for devnet-specific scenarios"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Performance optimizations based on Month 2 feedback"</span>
                        </li>
                    </ul>
                </div>

                // Security Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"Security & Audit Preparation"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Security audit of batch swap router contract Code review"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Dependency vulnerability scanning and updates"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Penetration testing of API endpoints"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Security documentation and best practices guide"</span>
                        </li>
                    </ul>
                </div>

                // Community Card
                <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl">
                    <div class="flex items-center mb-6">
                        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center mr-4">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                            </svg>
                        </div>
                        <h2 class="text-3xl font-bold text-white font-heading">"Community & Ecosystem"</h2>
                    </div>
                    <ul class="space-y-4 text-gray-300 font-sans">
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Launch devnet beta program with xx+ active testers"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Developer workshop: Building on XForce Terminal Devnet"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Begin Community governance model: feedback collection and feature prioritization"</span>
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
                            <span>"Devnet deployment guide: contract addresses, RPC endpoints, testing workflows"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Security best practices documentation"</span>
                        </li>
                        <li class="flex items-start">
                            <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Publish 3 technical blog posts:"</span>
                        </li>
                        <li class="ml-8 space-y-2 text-gray-400">
                            <div>"• Deploying Solana Programs: Lessons from XForce Terminal"</div>
                            <div>"• Security Architecture of Non-Custodial Trading Applications"</div>
                            <div>"• Testing DeFi Applications on Solana Devnet"</div>
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
                                <span>"Beta v0.3.0 downloads: xx+ users"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"Waitlist signups: xxx+ total signups"</span>
                            </li>
                            <li class="flex items-center">
                                <span class="text-red-400 mr-2">"•"</span>
                                <span>"Devnet beta testers: 20+ active testers providing feedback"</span>
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
                                <span>"Batch swap router contract successfully deployed and tested on devnet"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Beta v0.3.0 released with full devnet support and network switching"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Basic Security audit completed"</span>
                            </li>
                            <li class="flex items-start">
                                <svg class="w-5 h-5 text-red-400 mr-3 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                </svg>
                                <span>"Comprehensive test coverage"</span>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}

