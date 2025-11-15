//! # Router Module
//!
//! This module handles route configuration and setup.

use axum::{middleware, routing::post, Router};
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos_meta::MetaTags;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use crate::app_middleware::rate_limit::{rate_limit_middleware, RateLimiter};
use crate::api::contact::submit_contact_form_internal;
use frontend::App;
use shared::ContactFormData;
use axum::{extract::State, Json};
use std::path::PathBuf;

/// API state for route handlers.
#[derive(Clone, Debug)]
pub struct ApiState {
    /// Leptos options
    pub leptos_options: LeptosOptions,
}

/// Contact form handler wrapper.
///
/// This function wraps the contact form handler to match Axum's expected signature.
async fn submit_contact_form_handler(
    State(_state): State<ApiState>,
    payload: Json<ContactFormData>,
) -> impl axum::response::IntoResponse {
    submit_contact_form_internal(payload.0).await
}

/// Create the application router.
///
/// This function sets up all routes including:
/// - Static file serving (CSS, JS, WASM, images)
/// - API routes (contact form)
/// - Leptos SSR routes
///
/// # Arguments
///
/// * `leptos_options` - Leptos configuration options
/// * `pkg_dir` - Directory for serving package files
/// * `images_path` - Directory for serving images
/// * `rate_limiter` - Rate limiter instance
///
/// # Returns
///
/// Returns the configured Axum router.
pub fn create_router(
    leptos_options: LeptosOptions,
    pkg_dir: PathBuf,
    images_path: PathBuf,
    rate_limiter: RateLimiter,
) -> Router {
    // Generate route list from Leptos router
    let routes = generate_route_list(App);

    // Create API state
    let api_state = ApiState {
        leptos_options: leptos_options.clone(),
    };

    // API routes with rate limiting and CORS
    let api_routes = Router::new()
        .route("/contact", post(submit_contact_form_handler))
        .layer(middleware::from_fn(move |req, next| {
            let limiter = rate_limiter.clone();
            rate_limit_middleware(limiter, req, next)
        }))
        .layer(
            CorsLayer::permissive()
                .allow_origin(tower_http::cors::Any),
        )
        .with_state(api_state);

    // CRITICAL: Static files MUST be served BEFORE leptos_routes
    // Axum matches routes in order, so static files get priority
    Router::new()
        // Serve static files FIRST - highest priority
        // This serves files from pkg_dir when URL is /pkg/*
        .nest_service("/pkg", ServeDir::new(&pkg_dir))
        .nest_service("/images", ServeDir::new(&images_path))
        // API routes
        .nest("/api", api_routes)
        // Leptos SSR routes - ONLY matches app routes, not /pkg or /images
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        // Fallback handler for unmatched routes
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        // Enable request logging to see what routes are being matched
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                    )
                })
        )
}

/// Shell function for Leptos SSR.
///
/// This function generates the HTML shell for the Leptos application.
fn shell(options: LeptosOptions) -> impl IntoView {
    // Build CSS path - should resolve to /pkg/xforcesolutions.css
    let css_path = format!("/{}/{}.css", options.site_pkg_dir, options.output_name);
    let css_path_debug = css_path.clone();
    
    // Log CSS path for debugging (only in debug builds)
    #[cfg(debug_assertions)]
    {
        eprintln!("Shell CSS path: {}", css_path);
    }
    
    view! {
        <!DOCTYPE html>
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" type="image/svg+xml" href="/images/favicon.svg"/>
                <link rel="icon" type="image/png" href="/images/favicon.png"/>
                <link rel="apple-touch-icon" href="/images/favicon.png"/>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
                <link href="https://fonts.googleapis.com/css2?family=Playfair+Display:wght@400;500;600;700;800;900&family=Inter:wght@300;400;500;600;700;800&family=Montserrat:wght@300;400;500;600;700;800;900&family=JetBrains+Mono:wght@400;500;600;700&display=swap" rel="stylesheet"/>
                <link rel="stylesheet" href={css_path}/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                <script>
                    {format!(r#"
                    (function() {{
                        console.log('[CSS DEBUG] Starting CSS debug checks...');
                        console.log('[CSS DEBUG] Expected CSS path: {}');
                        
                        // Check if CSS link exists
                        const cssLink = document.querySelector('link[href="{}"]');
                        if (cssLink) {{
                            console.log('[CSS DEBUG] CSS link found in DOM:', cssLink.href);
                            console.log('[CSS DEBUG] CSS link element:', cssLink);
                            
                            // Check if stylesheet is loaded
                            const checkStylesheet = () => {{
                                try {{
                                    const sheets = Array.from(document.styleSheets);
                                    const ourSheet = sheets.find(sheet => {{
                                        try {{
                                            return sheet.href && sheet.href.includes('xforcesolutions.css');
                                        }} catch(e) {{
                                            return false;
                                        }}
                                    }});
                                    
                                    if (ourSheet) {{
                                        console.log('[CSS DEBUG] Stylesheet found in document.styleSheets');
                                        console.log('[CSS DEBUG] Stylesheet href:', ourSheet.href);
                                        console.log('[CSS DEBUG] Stylesheet rules count:', ourSheet.cssRules ? ourSheet.cssRules.length : 'N/A');
                                        
                                        // Try to read a rule
                                        try {{
                                            if (ourSheet.cssRules && ourSheet.cssRules.length > 0) {{
                                                console.log('[CSS DEBUG] First rule:', ourSheet.cssRules[0].cssText.substring(0, 100));
                                            }}
                                        }} catch(e) {{
                                            console.log('[CSS DEBUG] WARNING: Cannot read CSS rules (CORS or cross-origin):', e.message);
                                        }}
                                    }} else {{
                                        console.log('[CSS DEBUG] ERROR: Stylesheet NOT found in document.styleSheets');
                                        console.log('[CSS DEBUG] Available stylesheets:', sheets.map(s => {{
                                            try {{
                                                return s.href || 'inline';
                                            }} catch(e) {{
                                                return 'unknown';
                                            }}
                                        }}));
                                    }}
                                }} catch(e) {{
                                    console.error('[CSS DEBUG] Error checking stylesheets:', e);
                                }}
                            }};
                            
                            // Check when CSS loads
                            cssLink.onload = () => {{
                                console.log('[CSS DEBUG] CSS file loaded successfully');
                                checkStylesheet();
                            }};
                            
                            cssLink.onerror = () => {{
                                console.error('[CSS DEBUG] ERROR: CSS file failed to load');
                                console.error('[CSS DEBUG] Failed href:', cssLink.href);
                            }};
                            
                            // Check immediately
                            if (cssLink.sheet) {{
                                console.log('[CSS DEBUG] CSS already loaded (sheet exists)');
                                checkStylesheet();
                            }} else {{
                                console.log('[CSS DEBUG] CSS not loaded yet, waiting...');
                                // Check again after a delay
                                setTimeout(checkStylesheet, 1000);
                            }}
                        }} else {{
                            console.error('[CSS DEBUG] ERROR: CSS link NOT found in DOM');
                            console.log('[CSS DEBUG] All link tags:', Array.from(document.querySelectorAll('link')).map(l => l.href));
                        }}
                        
                        // Check computed styles on body
                        setTimeout(() => {{
                            const body = document.body;
                            if (body) {{
                                const computedStyle = window.getComputedStyle(body);
                                console.log('[CSS DEBUG] Body computed styles:');
                                console.log('[CSS DEBUG]   background-color:', computedStyle.backgroundColor);
                                console.log('[CSS DEBUG]   color:', computedStyle.color);
                                console.log('[CSS DEBUG]   font-family:', computedStyle.fontFamily);
                                console.log('[CSS DEBUG]   Body classes:', body.className);
                                
                                // Check if content is rendering
                                console.log('[CSS DEBUG] Body innerHTML length:', body.innerHTML ? body.innerHTML.length : 0);
                                console.log('[CSS DEBUG] Body children count:', body.children ? body.children.length : 0);
                                
                                // Check for hydration errors
                                const errors = [];
                                const scripts = document.querySelectorAll('script');
                                scripts.forEach(script => {{
                                    if (script.textContent && script.textContent.includes('hydration')) {{
                                        console.log('[CSS DEBUG] Found hydration script');
                                    }}
                                }});
                                
                                // Check console for errors
                                const originalError = console.error;
                                console.error = function(...args) {{
                                    errors.push(args);
                                    originalError.apply(console, args);
                                }};
                                
                                setTimeout(() => {{
                                    if (errors.length > 0) {{
                                        console.log('[CSS DEBUG] WARNING: Console errors detected:', errors.length);
                                        errors.forEach(err => console.log('[CSS DEBUG] Error:', err));
                                    }} else {{
                                        console.log('[CSS DEBUG] No console errors detected');
                                    }}
                                }}, 2000);
                            }} else {{
                                console.error('[CSS DEBUG] ERROR: Body element not found!');
                            }}
                        }}, 500);
                        
                        // Check for hydration errors immediately
                        window.addEventListener('error', (e) => {{
                            console.error('[CSS DEBUG] ERROR: Global error:', e.message, e.filename, e.lineno);
                        }});
                        
                        // Check for unhandled promise rejections (common with hydration errors)
                        window.addEventListener('unhandledrejection', (e) => {{
                            console.error('[CSS DEBUG] ERROR: Unhandled promise rejection:', e.reason);
                        }});
                    }})();
                    "#, css_path_debug, css_path_debug)}
                </script>
            </head>
            <body class="font-sans">
                <App/>
            </body>
        </html>
    }
}

