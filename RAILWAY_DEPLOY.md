# Railway Deployment Guide

This guide explains how to deploy XFSolutions to Railway using Docker.

## Prerequisites

1. A Railway account (sign up at [railway.app](https://railway.app))
2. Your project pushed to a Git repository (GitHub, GitLab, or Bitbucket)

## Deployment Steps

### Option 1: Deploy via Railway Dashboard

1. **Create a New Project**
   - Log in to Railway
   - Click "New Project"
   - Select "Deploy from GitHub repo" (or your Git provider)
   - Choose your repository

2. **Configure the Service**
   - Railway will automatically detect the `Dockerfile`
   - The `railway.json` file provides additional configuration
   - Railway will automatically set the `PORT` environment variable

3. **Set Environment Variables** (Optional)
   - Go to your service settings
   - Add any required environment variables:
     - `SMTP_HOST` - SMTP server hostname (default: smtp.gmail.com)
     - `SMTP_PORT` - SMTP server port (default: 587)
     - `SMTP_USER` - SMTP username (if using authenticated SMTP)
     - `SMTP_PASS` - SMTP password (if using authenticated SMTP)
     - `SMTP_FROM` - Email sender address (default: noreply@xforcesolutions.com)
     - `CONTACT_EMAIL` - Contact form recipient email
     - `RATE_LIMIT_REQUESTS` - Rate limit requests per window (default: 60)
     - `RATE_LIMIT_WINDOW` - Rate limit window in seconds (default: 60)
     - `CONTACT_LOG_FILE` - Contact log file path (default: /app/data/contact_submissions.log)

4. **Deploy**
   - Railway will automatically build and deploy your application
   - The build process includes:
     - Building Rust WASM frontend
     - Compiling Tailwind CSS
     - Building Rust backend
   - Once deployed, Railway will provide a public URL

### Option 2: Deploy via Railway CLI

1. **Install Railway CLI**
   ```bash
   npm i -g @railway/cli
   ```

2. **Login to Railway**
   ```bash
   railway login
   ```

3. **Initialize Railway in your project**
   ```bash
   railway init
   ```

4. **Deploy**
   ```bash
   railway up
   ```

## How It Works

- **Port Configuration**: Railway automatically sets the `PORT` environment variable. The backend code checks for `PORT` first, then falls back to `SERVER_ADDR` (default: `0.0.0.0:3000`).

- **Docker Build**: The multi-stage Dockerfile:
  1. Builds the Rust WASM frontend
  2. Compiles Tailwind CSS
  3. Builds the Rust backend
  4. Creates a minimal runtime image

- **Static Assets**: All static assets (WASM, CSS, images) are included in the Docker image.

## Environment Variables

The application supports the following environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `PORT` | Server port (set automatically by Railway) | `3000` |
| `SERVER_ADDR` | Server bind address | `0.0.0.0:3000` |
| `SMTP_HOST` | SMTP server hostname | `smtp.gmail.com` |
| `SMTP_PORT` | SMTP server port | `587` |
| `SMTP_USER` | SMTP username (optional) | - |
| `SMTP_PASS` | SMTP password (optional) | - |
| `SMTP_FROM` | Email sender address | `noreply@xforcesolutions.com` |
| `CONTACT_EMAIL` | Contact form recipient | `isicheivalentine@gmail.com` |
| `RATE_LIMIT_REQUESTS` | Rate limit requests per window | `60` |
| `RATE_LIMIT_WINDOW` | Rate limit window (seconds) | `60` |
| `CONTACT_LOG_FILE` | Contact log file path | `/app/data/contact_submissions.log` |

## Troubleshooting

### Build Fails
- Check Railway build logs for specific errors
- Ensure all dependencies are properly specified in `Cargo.toml` and `package.json`
- Verify the Dockerfile syntax is correct

### Application Won't Start
- Check that the `PORT` environment variable is being set by Railway
- Review application logs in Railway dashboard
- Verify all required environment variables are set

### Static Assets Not Loading
- Ensure the build process completed successfully (WASM and CSS generation)
- Check that `frontend/public` directory is copied correctly in Dockerfile

## Notes

- Railway provides persistent storage for the `/app/data` directory
- The application logs to stdout/stderr, which Railway captures automatically
- Railway automatically handles HTTPS/SSL certificates
- The application is stateless and can scale horizontally if needed

