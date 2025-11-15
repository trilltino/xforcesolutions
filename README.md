# XForce Solutions

**Static website built with Leptos and deployed to GitHub Pages**

A modern, client-side only web application showcasing XForce Solutions' services and portfolio.

## Features

- **Client-side only**: Pure WASM frontend with no backend required
- **Leptos framework**: Fast, modern Rust web framework
- **Tailwind CSS**: Utility-first CSS framework for styling
- **EmailJS integration**: Contact form powered by EmailJS
- **GitHub Pages**: Automatic deployment via GitHub Actions

## Project Structure

```
xforcesolutions/
├── frontend/          # Leptos frontend application
│   ├── src/           # Source code
│   ├── style/         # Tailwind CSS styles
│   └── public/         # Static assets (images, etc.)
├── .github/
│   └── workflows/     # GitHub Actions deployment workflow
├── Cargo.toml         # Rust workspace configuration
├── Leptos.toml        # Leptos build configuration
└── index.html         # HTML entry point
```

## Development

### Prerequisites

- Rust (latest stable)
- Node.js 20+ and npm
- `cargo-leptos` (install with `cargo install cargo-leptos --locked`)
- `wasm-bindgen-cli` (install with `cargo install wasm-bindgen-cli --version 0.2.87`)

### Local Development

1. Install dependencies:
   ```bash
   npm install
   ```

2. Build Tailwind CSS:
   ```bash
   npm run tailwind:build
   ```

3. Build the Leptos application:
   ```bash
   cargo leptos build --release
   ```

4. Serve the static site:
   ```bash
   # Using a simple HTTP server (Python example)
   cd target/site
   python -m http.server 8000
   ```

   Or use any static file server to serve the `target/site` directory.

## EmailJS Configuration

The contact form uses EmailJS to send emails. To configure:

1. Create a free account at [EmailJS](https://www.emailjs.com/)
2. Set up an email service (Gmail, Outlook, etc.)
3. Create an email template with the following variables:
   - `from_name`: Sender's name
   - `from_email`: Sender's email
   - `message`: Message content
   - `to_email`: Recipient email (set to `isicheivalentine@gmail.com`)

4. Update the EmailJS credentials in `frontend/src/pages/contact.rs`:
   - Replace `YOUR_SERVICE_ID` with your EmailJS service ID
   - Replace `YOUR_TEMPLATE_ID` with your EmailJS template ID
   - Replace `YOUR_PUBLIC_KEY` with your EmailJS public key

## Deployment

### GitHub Pages

The site is automatically deployed to GitHub Pages via GitHub Actions when you push to the `main` branch.

1. Enable GitHub Pages in your repository settings:
   - Go to Settings → Pages
   - Source: GitHub Actions

2. Push to `main` branch - the workflow will automatically:
   - Build the Leptos static site
   - Build Tailwind CSS
   - Deploy to GitHub Pages

The site will be available at `https://trilltino.github.io/xforcesolutions/`

## Pages

- **Home**: Landing page with overview
- **About**: Information about XForce Solutions
- **Projects**: Portfolio showcase
- **Contact**: Contact form powered by EmailJS

## Technologies

- **Leptos**: Rust web framework for building fast, reactive UIs
- **Tailwind CSS**: Utility-first CSS framework
- **EmailJS**: Email service for contact forms
- **WebAssembly**: Compiled Rust code running in the browser
- **GitHub Actions**: CI/CD for automated deployment

## License

[Your License Here]
