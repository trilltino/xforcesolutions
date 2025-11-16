pub mod month1;
pub mod month2;
pub mod month3;
pub mod month4;

pub use month1::Month1;
pub use month2::Month2;
pub use month3::Month3;
pub use month4::Month4;

use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Roadmap() -> impl IntoView {
    view! {
        <div class="max-w-6xl mx-auto py-8 px-4">
            // Header
            <div class="text-center mb-12">
                <h1 class="text-5xl font-bold mb-4 font-heading bg-gradient-to-r from-red-400 to-red-600 bg-clip-text text-transparent">
                    "Roadmap"
                </h1>
                <p class="text-2xl text-gray-300 mb-2 font-sans">"10,000 USDC"</p>
                <p class="text-lg text-gray-400 font-sans">
                    "Development roadmap and milestones for XForce Terminal"
                </p>
            </div>

            // Roadmap Cards Grid
            <div class="grid md:grid-cols-2 gap-8 mb-12">
                // Month 1 Card
                <A href="/roadmap/month1" attr:class="group">
                    <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl hover:border-red-500 transition-all duration-300 hover:scale-105">
                        <div class="flex items-center mb-6">
                            <div class="w-16 h-16 bg-gradient-to-br from-red-500 to-red-700 rounded-full flex items-center justify-center mr-4">
                                <span class="text-white font-bold text-2xl">"1"</span>
                            </div>
                            <div>
                                <h2 class="text-2xl font-bold text-white font-heading group-hover:text-red-400 transition-colors">"Month 1"</h2>
                                <p class="text-red-400 font-sans">"$2,500"</p>
                            </div>
                        </div>
                        <p class="text-gray-300 font-sans mb-4">
                            "UI/UX polish, minimal Beta release, community engagement, and documentation"
                        </p>
                        <div class="flex items-center text-red-400 group-hover:text-red-300 transition-colors">
                            <span class="mr-2">"View Details"</span>
                            <svg class="w-5 h-5 transform group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                            </svg>
                        </div>
                    </div>
                </A>

                // Month 2 Card
                <A href="/roadmap/month2" attr:class="group">
                    <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl hover:border-red-500 transition-all duration-300 hover:scale-105">
                        <div class="flex items-center mb-6">
                            <div class="w-16 h-16 bg-gradient-to-br from-red-500 to-red-700 rounded-full flex items-center justify-center mr-4">
                                <span class="text-white font-bold text-2xl">"2"</span>
                            </div>
                            <div>
                                <h2 class="text-2xl font-bold text-white font-heading group-hover:text-red-400 transition-colors">"Month 2"</h2>
                                <p class="text-red-400 font-sans">"$2,500"</p>
                            </div>
                        </div>
                        <p class="text-gray-300 font-sans mb-4">
                            "Advanced features, API infrastructure, community growth, and ecosystem integration"
                        </p>
                        <div class="flex items-center text-red-400 group-hover:text-red-300 transition-colors">
                            <span class="mr-2">"View Details"</span>
                            <svg class="w-5 h-5 transform group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                            </svg>
                        </div>
                    </div>
                </A>

                // Month 3 Card
                <A href="/roadmap/month3" attr:class="group">
                    <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl hover:border-red-500 transition-all duration-300 hover:scale-105">
                        <div class="flex items-center mb-6">
                            <div class="w-16 h-16 bg-gradient-to-br from-red-500 to-red-700 rounded-full flex items-center justify-center mr-4">
                                <span class="text-white font-bold text-2xl">"3"</span>
                            </div>
                            <div>
                                <h2 class="text-2xl font-bold text-white font-heading group-hover:text-red-400 transition-colors">"Month 3"</h2>
                                <p class="text-red-400 font-sans">"$2,500"</p>
                            </div>
                        </div>
                        <p class="text-gray-300 font-sans mb-4">
                            "Devnet deployment, contract testing, security hardening, and production readiness"
                        </p>
                        <div class="flex items-center text-red-400 group-hover:text-red-300 transition-colors">
                            <span class="mr-2">"View Details"</span>
                            <svg class="w-5 h-5 transform group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                            </svg>
                        </div>
                    </div>
                </A>

                // Month 4 Card
                <A href="/roadmap/month4" attr:class="group">
                    <div class="bg-gradient-to-br from-gray-900 to-black border border-red-900/50 rounded-xl p-8 shadow-2xl hover:border-red-500 transition-all duration-300 hover:scale-105">
                        <div class="flex items-center mb-6">
                            <div class="w-16 h-16 bg-gradient-to-br from-red-500 to-red-700 rounded-full flex items-center justify-center mr-4">
                                <span class="text-white font-bold text-2xl">"4"</span>
                            </div>
                            <div>
                                <h2 class="text-2xl font-bold text-white font-heading group-hover:text-red-400 transition-colors">"Month 4"</h2>
                                <p class="text-red-400 font-sans">"$2,500"</p>
                            </div>
                        </div>
                        <p class="text-gray-300 font-sans mb-4">
                            "Mainnet Beta Deployment & Production Launch"
                        </p>
                        <div class="flex items-center text-red-400 group-hover:text-red-300 transition-colors">
                            <span class="mr-2">"View Details"</span>
                            <svg class="w-5 h-5 transform group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                            </svg>
                        </div>
                    </div>
                </A>
            </div>
        </div>
    }
}

