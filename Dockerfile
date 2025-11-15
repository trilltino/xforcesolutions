# Multi-stage Dockerfile for XFSolutions
# Builds Rust WASM frontend, compiles Tailwind CSS, and creates production backend

# Stage 1: Build Frontend WASM
FROM rust:1.83-slim as wasm-builder

# Install wasm32 target and wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown && \
    cargo install wasm-bindgen-cli --version 0.2.87

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY frontend/Cargo.toml ./frontend/
COPY backend/Cargo.toml ./backend/
COPY shared/Cargo.toml ./shared/

# Copy source code
COPY frontend ./frontend
COPY shared ./shared
COPY backend ./backend

# Build frontend WASM
RUN cd frontend && \
    cargo build --target wasm32-unknown-unknown --release --lib --features hydrate

# Run wasm-bindgen
RUN mkdir -p target/site/pkg && \
    wasm-bindgen target/wasm32-unknown-unknown/release/frontend.wasm \
        --out-dir target/site/pkg \
        --target web \
        --no-typescript && \
    cp target/site/pkg/frontend.js target/site/pkg/xforcesolutions.js && \
    cp target/site/pkg/frontend_bg.wasm target/site/pkg/xforcesolutions_bg.wasm && \
    sed -i.bak 's/frontend_bg.wasm/xforcesolutions_bg.wasm/g' target/site/pkg/xforcesolutions.js && \
    rm target/site/pkg/xforcesolutions.js.bak

# Stage 2: Build Tailwind CSS
FROM node:20-slim as tailwind-builder

WORKDIR /app

# Copy package files
COPY package.json package-lock.json* ./
COPY frontend/tailwind.config.js ./frontend/

# Install dependencies
RUN npm ci

# Copy Tailwind input file and source files (needed for class scanning)
COPY frontend/style/tailwind.css ./frontend/style/
COPY frontend/src ./frontend/src

# Create target directory and build Tailwind CSS
RUN mkdir -p target/site/pkg && \
    cd frontend && \
    npx tailwindcss -i ./style/tailwind.css -o ../target/site/pkg/xforcesolutions.css --minify

# Stage 3: Build Backend
FROM rust:1.83-slim as backend-builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY frontend/Cargo.toml ./frontend/
COPY backend/Cargo.toml ./backend/
COPY shared/Cargo.toml ./shared/

# Copy source code
COPY frontend ./frontend
COPY backend ./backend
COPY shared ./shared

# Copy WASM artifacts from stage 1
COPY --from=wasm-builder /app/target/site/pkg/*.js /app/target/site/pkg/
COPY --from=wasm-builder /app/target/site/pkg/*.wasm /app/target/site/pkg/

# Copy Tailwind CSS from stage 2
COPY --from=tailwind-builder /app/target/site/pkg/xforcesolutions.css /app/target/site/pkg/

# Build backend
RUN cargo build --release --bin backend

# Stage 4: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
        && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/target/release/backend /app/backend

# Copy static assets
COPY --from=backend-builder /app/target/site /app/target/site
COPY frontend/public /app/frontend/public

# Create data directory for logs
RUN mkdir -p /app/data

# Set environment variables
# Fly.io sets PORT automatically, backend will use it if available
# SERVER_ADDR is used as fallback if PORT is not set
ENV SERVER_ADDR=0.0.0.0:3000
ENV RATE_LIMIT_REQUESTS=60
ENV RATE_LIMIT_WINDOW=60
ENV CONTACT_LOG_FILE=/app/data/contact_submissions.log

# Expose port (Fly.io will map this automatically)
# The backend reads PORT env var and binds to 0.0.0.0:PORT
EXPOSE 3000

# Run the backend
# The backend automatically uses PORT env var if set (Fly.io provides this)
CMD ["./backend"]

