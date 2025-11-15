use leptos::prelude::*;
use leptos::ev;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// Contact form data structure
#[derive(Clone)]
struct ContactFormData {
    name: String,
    email: String,
    message: String,
}

#[component]
pub fn Contact() -> impl IntoView {
    // Form state signals
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());

    // Validation error signals
    let (name_error, set_name_error) = signal(Option::<String>::None);
    let (email_error, set_email_error) = signal(Option::<String>::None);
    let (message_error, set_message_error) = signal(Option::<String>::None);

    // UI state signals
    let (is_submitting, set_is_submitting) = signal(false);
    let (submit_status, set_submit_status) = signal(Option::<(bool, String)>::None);

    // Validate name field
    let validate_name = move |value: &str| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            set_name_error.set(Some("Name is required".to_string()));
            false
        } else if trimmed.len() < 2 {
            set_name_error.set(Some("Name must be at least 2 characters".to_string()));
            false
        } else if trimmed.len() > 100 {
            set_name_error.set(Some("Name must be less than 100 characters".to_string()));
            false
        } else {
            set_name_error.set(None);
            true
        }
    };

    // Validate email field
    let validate_email = move |value: &str| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            set_email_error.set(Some("Email is required".to_string()));
            false
        } else if !is_valid_email(trimmed) {
            set_email_error.set(Some("Please enter a valid email address".to_string()));
            false
        } else {
            set_email_error.set(None);
            true
        }
    };

    // Validate message field
    let validate_message = move |value: &str| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            set_message_error.set(Some("Message is required".to_string()));
            false
        } else if trimmed.len() < 10 {
            set_message_error.set(Some("Message must be at least 10 characters".to_string()));
            false
        } else if trimmed.len() > 1000 {
            set_message_error.set(Some("Message must be less than 1000 characters".to_string()));
            false
        } else {
            set_message_error.set(None);
            true
        }
    };

    // Send email via EmailJS
    // Note: User needs to configure EmailJS credentials
    // Get these from https://www.emailjs.com/ after creating an account
    let send_email_via_emailjs = move |form_data: ContactFormData| {
        let window = web_sys::window().expect("no global `window` exists");
        
        // EmailJS configuration - user needs to set these
        // TODO: Make these configurable via environment or config file
        let service_id = "YOUR_SERVICE_ID"; // Replace with your EmailJS service ID
        let template_id = "YOUR_TEMPLATE_ID"; // Replace with your EmailJS template ID
        let public_key = "YOUR_PUBLIC_KEY"; // Replace with your EmailJS public key

        // EmailJS REST API endpoint
        let url = "https://api.emailjs.com/api/v1.0/email/send";

        let mut opts = web_sys::RequestInit::new();
        opts.set_method("POST");
        opts.set_mode(web_sys::RequestMode::Cors);
        
        let headers = web_sys::Headers::new().unwrap();
        headers.set("Content-Type", "application/json").unwrap();
        opts.set_headers(&JsValue::from(headers));

        // EmailJS expects form data, but we'll send as JSON
        let body = serde_json::json!({
            "service_id": service_id,
            "template_id": template_id,
            "user_id": public_key,
            "template_params": {
                "from_name": form_data.name,
                "from_email": form_data.email,
                "message": form_data.message,
                "to_email": "isicheivalentine@gmail.com"
            }
        });

        let body_js = JsValue::from_str(&serde_json::to_string(&body).unwrap());
        opts.set_body(&body_js);

        let request = web_sys::Request::new_with_str_and_init(&url, &opts).unwrap();
        
        let future = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request));
        
        leptos::task::spawn_local(async move {
            match future.await {
                Ok(response) => {
                    let resp: web_sys::Response = response.dyn_into().unwrap();
                    let status = resp.status();
                    if (200..300).contains(&status) {
                        set_is_submitting.set(false);
                        set_submit_status.set(Some((true, "Thank you for contacting us! We'll get back to you soon.".to_string())));
                        // Reset form after success
                        set_timeout(
                            move || {
                                set_name.set(String::new());
                                set_email.set(String::new());
                                set_message.set(String::new());
                                set_submit_status.set(None);
                            },
                            std::time::Duration::from_secs(3),
                        );
                    } else {
                        set_is_submitting.set(false);
                        set_submit_status.set(Some((false, format!("Failed to send message (status: {}). Please check your EmailJS configuration.", status))));
                    }
                }
                Err(_e) => {
                    set_is_submitting.set(false);
                    let error_msg = "Failed to send message. Please check your EmailJS configuration and try again.".to_string();
                    set_submit_status.set(Some((false, error_msg)));
                }
            }
        });
    };

    // Form submission handler
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        // Validate all fields
        let name_valid = validate_name(&name.get());
        let email_valid = validate_email(&email.get());
        let message_valid = validate_message(&message.get());

        if !name_valid || !email_valid || !message_valid {
            return;
        }

        // Prepare form data
        let form_data = ContactFormData {
            name: name.get().trim().to_string(),
            email: email.get().trim().to_lowercase(),
            message: message.get().trim().to_string(),
        };

        // Set submitting state
        set_is_submitting.set(true);
        set_submit_status.set(None);

        // Send email via EmailJS
        send_email_via_emailjs(form_data);
    };

    view! {
        <div class="max-w-4xl mx-auto">
            <h1 class="text-4xl font-bold mb-8 text-center font-heading reflective-header">"Contact Us"</h1>

            <p class="text-xl text-gray-300 dark:text-gray-300 text-gray-700 dark:text-gray-300 text-center mb-12 font-sans">
                "Get in touch with us to discuss your project"
            </p>

            <div class="grid md:grid-cols-2 gap-8">
                <div class="bg-black dark:bg-black bg-white dark:bg-black p-8 rounded-lg">
                    <h2 class="text-2xl font-bold mb-6 font-heading reflective-header">"Send us a message"</h2>

                    // Success/Error Message
                    {move || submit_status.get().map(|(success, msg)| {
                        let bg_color = if success { "bg-green-900/50" } else { "bg-red-900/50" };
                        let border_color = if success { "border-green-600" } else { "border-red-600" };
                        let text_color = if success { "text-green-400" } else { "text-red-400" };
                        view! {
                            <div class={format!("mb-4 p-4 rounded-lg border {} {}", bg_color, border_color)}>
                                <p class={text_color}>{msg}</p>
                            </div>
                        }
                    })}

                    <form on:submit=on_submit class="space-y-4">
                        <div>
                            <label class="block text-sm font-medium mb-2" for="name">
                                "Name"
                            </label>
                            <input
                                type="text"
                                id="name"
                                class="w-full px-4 py-2 bg-gray-800 dark:bg-gray-800 bg-gray-100 dark:bg-gray-800 border border-gray-700 dark:border-gray-700 border-gray-300 dark:border-gray-700 rounded-lg focus:outline-none focus:border-primary-500 dark:focus:border-primary-500 text-white dark:text-white text-gray-900 dark:text-white disabled:opacity-50 font-sans"
                                placeholder="Your name"
                                prop:value=move || name.get()
                                on:input=move |ev| set_name.set(event_target_value(&ev))
                                on:blur=move |_| { validate_name(&name.get()); }
                                disabled=move || is_submitting.get()
                            />
                            {move || name_error.get().map(|err| view! {
                                <p class="text-red-400 text-sm mt-1">{err}</p>
                            })}
                        </div>
                        <div>
                            <label class="block text-sm font-medium mb-2" for="email">
                                "Email"
                            </label>
                            <input
                                type="email"
                                id="email"
                                class="w-full px-4 py-2 bg-gray-800 dark:bg-gray-800 bg-gray-100 dark:bg-gray-800 border border-gray-700 dark:border-gray-700 border-gray-300 dark:border-gray-700 rounded-lg focus:outline-none focus:border-primary-500 dark:focus:border-primary-500 text-white dark:text-white text-gray-900 dark:text-white disabled:opacity-50 font-sans"
                                placeholder="your@email.com"
                                prop:value=move || email.get()
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                                on:blur=move |_| { validate_email(&email.get()); }
                                disabled=move || is_submitting.get()
                            />
                            {move || email_error.get().map(|err| view! {
                                <p class="text-red-400 text-sm mt-1">{err}</p>
                            })}
                        </div>
                        <div>
                            <label class="block text-sm font-medium mb-2" for="message">
                                "Message"
                            </label>
                            <textarea
                                id="message"
                                rows="5"
                                class="w-full px-4 py-2 bg-gray-800 dark:bg-gray-800 bg-gray-100 dark:bg-gray-800 border border-gray-700 dark:border-gray-700 border-gray-300 dark:border-gray-700 rounded-lg focus:outline-none focus:border-primary-500 dark:focus:border-primary-500 text-white dark:text-white text-gray-900 dark:text-white disabled:opacity-50 font-sans"
                                placeholder="Tell us about your project..."
                                prop:value=move || message.get()
                                on:input=move |ev| set_message.set(event_target_value(&ev))
                                on:blur=move |_| { validate_message(&message.get()); }
                                disabled=move || is_submitting.get()
                            />
                            {move || message_error.get().map(|err| view! {
                                <p class="text-red-400 text-sm mt-1">{err}</p>
                            })}
                        </div>
                        <button
                            type="submit"
                            class="w-full px-6 py-3 bg-primary-600 hover:bg-primary-700 text-white rounded-lg font-semibold transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center"
                            disabled=move || is_submitting.get()
                        >
                            {move || if is_submitting.get() {
                                view! {
                                    <>
                                        <span class="inline-block animate-spin mr-2">"..."</span>
                                        "Sending..."
                                    </>
                                }.into_any()
                            } else {
                                view! { "Send Message" }.into_any()
                            }}
                        </button>
                    </form>
                </div>

                <div class="space-y-6">
                    <ContactInfo
                        title="Email"
                        content="isicheivalentine@gmail.com"
                        icon="[EMAIL]"
                    />
                    <ContactInfo
                        title="Phone"
                        content="+1 (555) 123-4567"
                        icon="[PHONE]"
                    />
                    <ContactInfo
                        title="Location"
                        content="123 Tech Street, Innovation City, IC 12345"
                        icon="[LOCATION]"
                    />
                    <ContactInfo
                        title="Business Hours"
                        content="Monday - Friday: 9:00 AM - 6:00 PM"
                        icon="[HOURS]"
                    />

                    <div class="bg-black dark:bg-black bg-white dark:bg-black p-6 rounded-lg">
                        <h3 class="text-lg font-bold mb-4 font-heading">"Connect With Us"</h3>
                        <div class="flex gap-4">
                            <SocialLink
                                href="https://calendly.com/isicheivalentine/30min"
                                alt="Schedule a meeting"
                                image_url="/images/calendly.svg"
                            />
                            <SocialLink
                                href="https://www.linkedin.com/in/valentine-i-b0619b2b6/"
                                alt="LinkedIn"
                                image_url="/images/linkedin.svg"
                            />
                            <SocialLink
                                href="https://x.com/StellarEuropa"
                                alt="X (Twitter)"
                                image_url="/images/x.svg"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ContactInfo(title: &'static str, content: &'static str, icon: &'static str) -> impl IntoView {
    view! {
        <div class="bg-black dark:bg-black bg-white dark:bg-black p-6 rounded-lg">
            <div class="flex items-start">
                <div class="text-3xl mr-4">{icon}</div>
                <div>
                    <h3 class="text-lg font-bold mb-1 font-heading">{title}</h3>
                    <p class="text-gray-400 dark:text-gray-400 text-gray-600 dark:text-gray-400 font-sans">{content}</p>
                </div>
            </div>
        </div>
    }
}

/// Email validation helper
fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 3 && email.len() <= 254
}

#[component]
fn SocialLink(href: &'static str, alt: &'static str, image_url: &'static str) -> impl IntoView {
    view! {
        <a
            href=href
            target="_blank"
            rel="noopener noreferrer"
            class="w-12 h-12 flex items-center justify-center bg-gray-800 rounded-lg border border-gray-700 hover:border-primary-500 transition-all duration-200 hover:scale-110"
        >
            <img
                src=image_url
                alt=alt
                class="w-8 h-8 object-contain"
            />
        </a>
    }
}
