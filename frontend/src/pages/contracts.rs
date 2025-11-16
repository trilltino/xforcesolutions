use leptos::prelude::*;

#[component]
pub fn Contracts() -> impl IntoView {
    view! {
        <div class="max-w-6xl mx-auto py-8 px-4">
            // Hero Section
            <div class="text-center mb-12">
                <h1 class="text-5xl font-bold mb-4 font-heading text-gray-900 dark:text-white">
                    "Batch Swap Router Contracts"
                </h1>
                <p class="text-xl text-gray-700 dark:text-gray-300 mb-6 font-sans">
                    "Production-grade Solana smart contracts for atomic batch token swaps"
                </p>
                <div class="flex items-center justify-center gap-4 text-sm text-gray-600 dark:text-gray-400">
                    <span class="px-3 py-1 bg-purple-100 dark:bg-purple-900/30 rounded-full">
                        "Solana Devnet"
                    </span>
                    <span class="px-3 py-1 bg-blue-100 dark:bg-blue-900/30 rounded-full">
                        "MIT License"
                    </span>
                    <span class="px-3 py-1 bg-green-100 dark:bg-green-900/30 rounded-full">
                        "100% Open Source"
                    </span>
                </div>
            </div>

            // Overview Section
            <div class="bg-white dark:bg-gray-900 p-8 rounded-lg shadow-lg mb-8">
                <h2 class="text-3xl font-bold mb-6 font-heading text-gray-900 dark:text-white">
                    "Overview"
                </h2>
                <p class="text-gray-700 dark:text-gray-300 mb-4 font-sans leading-relaxed">
                    "The Batch Swap Router is a Solana smart contract that enables users to execute multiple token swaps atomically in a single transaction. Built with the Anchor framework, it leverages Solana's high throughput and low transaction fees to provide cost-effective batch swapping capabilities."
                </p>
                <p class="text-gray-700 dark:text-gray-300 mb-4 font-sans leading-relaxed">
                    "Instead of executing 3 separate swaps (SOL → USDC → BONK → RAY) that would cost 3 transaction fees, you can execute all swaps in ONE transaction, saving 60-90% on fees while ensuring atomic execution - either all swaps succeed or all fail together."
                </p>
                <div class="bg-gray-50 dark:bg-gray-800 p-4 rounded-lg mt-4">
                    <p class="text-sm font-semibold text-gray-900 dark:text-white mb-2">"Program ID (Devnet):"</p>
                    <code class="text-sm text-purple-600 dark:text-purple-400 break-all">
                        "HS63bw1V1qTM5uWf92q3uaFdqogrc4SN9qUJSR8aqBMx"
                    </code>
                    <a
                        href="https://explorer.solana.com/address/HS63bw1V1qTM5uWf92q3uaFdqogrc4SN9qUJSR8aqBMx?cluster=devnet"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="ml-4 text-blue-600 dark:text-blue-400 hover:underline text-sm"
                    >
                        "View on Explorer →"
                    </a>
                </div>
            </div>

            // Key Features
            <div class="grid md:grid-cols-2 gap-6 mb-8">
                <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg">
                    <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                        "⚡ Atomic Execution"
                    </h3>
                    <p class="text-gray-700 dark:text-gray-300 font-sans">
                        "All swaps in a batch either succeed together or fail together. No partial executions - your funds are always safe."
                    </p>
                </div>
                <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg">
                    <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                        "💰 Fee Reduction"
                    </h3>
                    <p class="text-gray-700 dark:text-gray-300 font-sans">
                        "Pay transaction fees once instead of multiple times. Execute up to 10 swaps in a single transaction."
                    </p>
                </div>
                <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg">
                    <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                        "🛡️ Slippage Protection"
                    </h3>
                    <p class="text-gray-700 dark:text-gray-300 font-sans">
                        "Built-in validation ensures you receive the minimum expected output. Maximum slippage tolerance: 5%."
                    </p>
                </div>
                <div class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-lg">
                    <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                        "🔗 Jupiter Integration"
                    </h3>
                    <p class="text-gray-700 dark:text-gray-300 font-sans">
                        "Integrates with Jupiter aggregator for best-price routing across all DEXes on Solana."
                    </p>
                </div>
            </div>

            // Code Snippets Section
            <div class="bg-white dark:bg-gray-900 p-8 rounded-lg shadow-lg mb-8">
                <h2 class="text-3xl font-bold mb-6 font-heading text-gray-900 dark:text-white">
                    "How It Works on Solana"
                </h2>
                <p class="text-gray-700 dark:text-gray-300 mb-6 font-sans leading-relaxed">
                    "The contract leverages Solana's unique architecture to provide efficient batch swapping. Here's how it uses Solana's blockchain to achieve this:"
                </p>

                // Batch Swap Instruction
                <div class="mb-8">
                    <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                        "Batch Swap Instruction"
                    </h3>
                    <p class="text-gray-700 dark:text-gray-300 mb-3 font-sans text-sm">
                        "The core instruction that executes multiple swaps atomically in a single Solana transaction:"
                    </p>
                    <pre class="bg-gray-900 dark:bg-black p-6 rounded-lg overflow-x-auto text-sm text-gray-100">
<code>//! Execute multiple token swaps in a single transaction
//! This leverages Solana's transaction model where multiple
//! instructions execute atomically

pub fn batch_swap(
    ctx: Context&lt;BatchSwap&gt;,
    swaps: Vec&lt;SwapParams&gt;
) -&gt; Result&lt;()&gt; {{
    // Validate batch size (max 10 swaps per transaction)
    require!(!swaps.is_empty(), ErrorCode::EmptySwaps);
    require!(
        swaps.len() &lt;= MAX_BATCH_SIZE,
        ErrorCode::TooManySwaps
    );

    // Validate each swap parameter
    for swap in &amp;swaps {{
        require!(
            swap.amount &gt;= MIN_SWAP_AMOUNT,
            ErrorCode::InvalidAmount
        );
        // Input and output mints must differ
        assert_different_mints(
            &amp;swap.input_mint,
            &amp;swap.output_mint
        )?;
    }}

    // Calculate total fees with safe math
    let mut total_input_amount: u64 = 0;
    let mut total_protocol_fees: u64 = 0;
    
    for swap in &amp;swaps {{
        let fee = calculate_protocol_fee(swap.amount)?;
        total_input_amount = total_input_amount.safe_add(swap.amount)?;
        total_protocol_fees = total_protocol_fees.safe_add(fee)?;
    }}

    // Emit event for tracking (stored on-chain)
    emit!(BatchSwapEvent {{
        authority: ctx.accounts.authority.key(),
        swap_count: swaps.len() as u8,
        total_input_amount,
        total_protocol_fees,
        timestamp: Clock::get()?.unix_timestamp,
    }});

    Ok(())
}}</code></pre>
                </div>

                // Swap Parameters Structure
                <div class="mb-8">
                    <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                        "Swap Parameters Structure"
                    </h3>
                    <p class="text-gray-700 dark:text-gray-300 mb-3 font-sans text-sm">
                        "Each swap in the batch is defined by this structure, using Solana's native Pubkey type for addresses:"
                    </p>
                    <pre class="bg-gray-900 dark:bg-black p-6 rounded-lg overflow-x-auto text-sm text-gray-100">
<code>// Swap parameters for a single swap operation
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SwapParams {{
    /// Input token mint (Solana address)
    pub input_mint: Pubkey,
    
    /// Output token mint (Solana address)
    pub output_mint: Pubkey,
    
    /// Amount in token's smallest unit
    /// (e.g., lamports for SOL)
    pub amount: u64,
    
    /// Minimum output for slippage protection
    pub min_output_amount: u64,
}}

// Example usage:
// SwapParams {{
//     input_mint: sol_mint,           // SOL address
//     output_mint: usdc_mint,         // USDC address
//     amount: 1_000_000_000,          // 1 SOL (in lamports)
//     min_output_amount: 90_000_000,  // 90 USDC (10% slippage)
// }}</code></pre>
                </div>

                // Account Structure
                <div class="mb-8">
                    <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                        "Solana Account Structure"
                    </h3>
                    <p class="text-gray-700 dark:text-gray-300 mb-3 font-sans text-sm">
                        "The program uses Anchor's account validation to ensure security on Solana:"
                    </p>
                    <pre class="bg-gray-900 dark:bg-black p-6 rounded-lg overflow-x-auto text-sm text-gray-100">
<code>// Account structure for batch swap instruction
// Anchor automatically validates these accounts
#[derive(Accounts)]
pub struct BatchSwap&lt;'info&gt; {{
    /// Authority must sign the transaction
    /// Solana ensures only the signer can execute
    #[account(mut)]
    pub authority: Signer&lt;'info&gt;,
    
    /// Fee recipient (optional)
    #[account(mut)]
    pub fee_recipient: UncheckedAccount&lt;'info&gt;,
    
    /// SPL Token program (required for token ops)
    pub token_program: Program&lt;'info, Token&gt;,
    
    /// System program (Solana's core program)
    pub system_program: Program&lt;'info, System&gt;,
}}

// Anchor's #[derive(Accounts)] automatically:
// - Validates account ownership
// - Checks account types
// - Verifies signers
// - Ensures account mutability</code></pre>
                </div>

                // Security Validations
                <div class="mb-8">
                    <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                        "Security &amp; Validation"
                    </h3>
                    <p class="text-gray-700 dark:text-gray-300 mb-3 font-sans text-sm">
                        "Comprehensive security checks protect users on Solana:"
                    </p>
                    <pre class="bg-gray-900 dark:bg-black p-6 rounded-lg overflow-x-auto text-sm text-gray-100">
<code>// Program constants that define limits
pub const MAX_BATCH_SIZE: usize = 10;      // Max swaps per batch
pub const MIN_SWAP_AMOUNT: u64 = 1;        // Minimum swap amount
pub const PROTOCOL_FEE_BPS: u64 = 30;      // 0.3% protocol fee
pub const MAX_SLIPPAGE_BPS: u64 = 500;     // 5% max slippage

// Security validations
fn assert_different_mints(
    input: &amp;Pubkey,
    output: &amp;Pubkey
) -&gt; Result&lt;()&gt; {{
    require!(
        input != output,
        ErrorCode::InvalidSwapPair
    );
    Ok(())
}

// Safe math to prevent overflow/underflow
impl SafeMath for u64 {{
    fn safe_add(self, other: u64) -&gt; Result&lt;u64&gt; {{
        self.checked_add(other)
            .ok_or(ErrorCode::MathOverflow)
    }}
}}</code></pre>
                </div>

                // Event Emission
                <div class="mb-8">
                    <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                        "On-Chain Event Tracking"
                    </h3>
                    <p class="text-gray-700 dark:text-gray-300 mb-3 font-sans text-sm">
                        "Events are stored on Solana's blockchain and can be indexed for analytics:"
                    </p>
                    <pre class="bg-gray-900 dark:bg-black p-6 rounded-lg overflow-x-auto text-sm text-gray-100">
<code>// Event emitted on-chain for tracking
#[event]
pub struct BatchSwapEvent {{
    pub authority: Pubkey,           // Who executed
    pub swap_count: u8,              // Number of swaps
    pub total_input_amount: u64,     // Total input
    pub total_protocol_fees: u64,    // Fees collected
    pub timestamp: i64,              // Solana clock time
}}

// Events are:
// - Stored in transaction logs
// - Indexable by block explorers
// - Queryable via RPC
// - Used for analytics and monitoring</code></pre>
                </div>
            </div>

            // Solana Benefits Section
            <div class="bg-gradient-to-r from-purple-50 to-blue-50 dark:from-purple-900/20 dark:to-blue-900/20 p-8 rounded-lg shadow-lg mb-8">
                <h2 class="text-3xl font-bold mb-6 font-heading text-gray-900 dark:text-white">
                    "Why Solana?"
                </h2>
                <div class="grid md:grid-cols-2 gap-6">
                    <div>
                        <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                            "⚡ High Throughput"
                        </h3>
                        <p class="text-gray-700 dark:text-gray-300 font-sans">
                            "Solana processes thousands of transactions per second, enabling batch swaps to execute quickly and efficiently."
                        </p>
                    </div>
                    <div>
                        <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                            "💸 Low Fees"
                        </h3>
                        <p class="text-gray-700 dark:text-gray-300 font-sans">
                            "Transaction fees are typically $0.00025 or less, making batch swaps economically viable even for small amounts."
                        </p>
                    </div>
                    <div>
                        <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                            "🔒 Atomic Transactions"
                        </h3>
                        <p class="text-gray-700 dark:text-gray-300 font-sans">
                            "Solana's transaction model ensures all instructions in a transaction execute atomically - perfect for batch swaps."
                        </p>
                    </div>
                    <div>
                        <h3 class="text-xl font-bold mb-3 font-heading text-gray-900 dark:text-white">
                            "🌐 Program Composability"
                        </h3>
                        <p class="text-gray-700 dark:text-gray-300 font-sans">
                            "Cross-program invocations (CPI) allow seamless integration with Jupiter and other DeFi protocols."
                        </p>
                    </div>
                </div>
            </div>

            // Repository Link
            <div class="bg-white dark:bg-gray-900 p-8 rounded-lg shadow-lg text-center">
                <h2 class="text-2xl font-bold mb-4 font-heading text-gray-900 dark:text-white">
                    "Open Source &amp; Auditable"
                </h2>
                <p class="text-gray-700 dark:text-gray-300 mb-6 font-sans">
                    "View the full source code, contribute, or deploy your own instance."
                </p>
                <a
                    href="https://github.com/trilltino/xforce-terminal-contracts"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex items-center gap-2 px-6 py-3 bg-gray-900 dark:bg-white text-white dark:text-gray-900 rounded-lg hover:bg-gray-800 dark:hover:bg-gray-100 transition-colors duration-200 font-semibold"
                >
                    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                    </svg>
                    "View on GitHub"
                </a>
            </div>
        </div>
    }
}

