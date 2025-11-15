# Fly.io Setup Instructions

## 1. Install flyctl CLI

Run this command in PowerShell (as Administrator if needed):

```powershell
powershell -Command "iwr https://fly.io/install.ps1 -useb | iex"
```

This will install `flyctl` to your system. After installation, you may need to:
- Close and reopen your terminal/PowerShell
- Or add flyctl to your PATH manually

## 2. Verify Installation

After installation, verify it works:

```powershell
fly version
```

## 3. Login to Fly.io

```powershell
fly auth login
```

This will open a browser for authentication.

## 4. Launch Your App

Since the app was destroyed, you need to create it fresh:

```powershell
fly launch
```

When prompted:
- **App name:** `xforcesolutions` (or choose a new one)
- **Region:** Choose `iad` (Washington, D.C.) or your preferred region
- **PostgreSQL:** Say "No" (you're using Railway for database)
- **Redis:** Say "No"
- **Deploy now:** Say "Yes" to deploy immediately

The `fly launch` command will:
- Detect your Dockerfile automatically
- Generate/update `fly.toml` with proper configuration
- Deploy your app

## 5. Set Environment Variables

After deployment, set your Railway database URL:

```powershell
fly secrets set DATABASE_URL="your-railway-postgres-url"
```

You can also set other environment variables:
```powershell
fly secrets set SMTP_HOST="smtp.gmail.com"
fly secrets set SMTP_PORT="587"
fly secrets set CONTACT_EMAIL="your-email@example.com"
# ... etc
```

## 6. View Your App

```powershell
fly open
```

This will open your deployed app in a browser.

## Troubleshooting

If `fly launch` still gives the manifest error:
1. Make sure `fly.toml` is minimal (it should be after the fix)
2. Try deleting `fly.toml` temporarily and let `fly launch` generate it fresh
3. Check that your Dockerfile is in the root directory

## Notes

- The simplified `fly.toml` now only contains app name, region, and Dockerfile reference
- `fly launch` will add the proper service configuration automatically
- Your backend code already handles the `PORT` environment variable that Fly.io provides

