
# Anomalous Inquiry

A neutral, documentary-style research platform for the systematic investigation of anomalous and exceptional experiences. Built with Rust + Axum. No frontend JavaScript.

## Coverage

### Parapsychology & ESP
- Extrasensory perception (ESP) overview
- Ganzfeld experiments
- Precognition and retrocausation research
- Psychokinesis (PK) overview
- PEAR lab micro-PK research
- Remote viewing — Stargate programme
- Mediumship research
- Stevenson reincarnation studies
- Terminal lucidity

### Anomalous Events & Cases
- Nimitz UAP incident (2004)
- Roswell (1947)
- Close encounter archive (CE1–CE5) & timeline

### Survival Research
- Near-death experiences (NDE)

### Altered States & Exceptional Experiences
- Out-of-body experiences (OBE)
- Dream telepathy
- Lucid dreaming (parapsychological context)
- Hypnagogic and hypnopompic states
- Trance states
- Non-pathological dissociative experiences
- Visionary experiences

## Features
- Static articles authored in Markdown with YAML front matter
- Reflections journal (admin-only, cookie-authenticated)
- Moderated anonymous comments per article
- Suggestions inbox
- CE1–CE5 archive and interactive timeline
- PDF export per article
- RSS feed
- Citation formatting (APA and Chicago)
- Tag browsing
- No frontend JavaScript

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (2021 edition) |
| HTTP framework | Axum 0.7 |
| Templating | Tera |
| Markdown rendering | pulldown-cmark |
| PDF generation | printpdf |
| Async runtime | Tokio |
| Date handling | Chrono |
| RSS | rss crate |

## Local Development

```bash
# Clone the repository
git clone https://github.com/Anaxagorius/Anomalous-inquiry.git
cd Anomalous-inquiry/anomalous_inquiry

# Run in development mode
cargo run

# The server starts at http://localhost:8080
```

The server reads articles from `content/articles/` at startup. Add or edit `.md` files there and restart the server to pick up changes.

### Admin Access

The reflections journal and comment moderation panel are available at `/admin`. Authentication uses a cookie set by a `POST /admin/login` form. The default password is defined at server startup via the `ADMIN_PASSWORD` environment variable (see `src/state.rs`).

### Adding Articles

Create a new `.md` file in `content/articles/` with the following front matter:

```markdown
---
title: Your Article Title
tags: tag-one, tag-two, tag-three
published: YYYY-MM-DD
---

Article body in standard Markdown...
```

The slug is derived automatically from the filename (without the `.md` extension).

## Deployment on Render.com

Anomalous Inquiry is designed for straightforward deployment on [Render](https://render.com) as a **Web Service**.

### Prerequisites

- A [Render account](https://dashboard.render.com)
- The repository pushed to GitHub or GitLab

### Steps

1. **Create a new Web Service** in the Render dashboard and connect your GitHub repository.

2. **Configure the build and start commands:**

   | Setting | Value |
   |---------|-------|
   | Runtime | `Docker` or `Rust` (native) |
   | Build Command | `cd anomalous_inquiry && cargo build --release` |
   | Start Command | `./anomalous_inquiry/target/release/anomalous_inquiry` |
   | Root Directory | *(leave blank — repository root)* |

3. **Set environment variables** in the Render dashboard under *Environment*:

   | Variable | Description |
   |----------|-------------|
   | `ADMIN_PASSWORD` | Password for the `/admin` panel |
   | `PORT` | Port Render assigns (use `$PORT` or set to `8080`) |

   Render injects `PORT` automatically. Ensure the application binds to `0.0.0.0:$PORT` (see `src/main.rs`).

4. **Persistent storage (optional):** Comments and suggestions are currently held in memory and lost on redeploy. To persist them, add a Render **Disk** mounted at a path of your choice and update `src/state.rs` to read/write JSON to that path.

5. **Deploy.** Render will install the Rust toolchain, build the release binary, and start the service. Cold-start build times are typically 3–6 minutes on the free tier.

### Render `render.yaml` (Infrastructure as Code)

You can commit a `render.yaml` to the repository root to configure the service declaratively:

```yaml
services:
  - type: web
    name: anomalous-inquiry
    runtime: rust
    buildCommand: cd anomalous_inquiry && cargo build --release
    startCommand: ./anomalous_inquiry/target/release/anomalous_inquiry
    envVars:
      - key: ADMIN_PASSWORD
        generateValue: true
      - key: RUST_LOG
        value: info
    disk:
      name: data
      mountPath: /data
      sizeGB: 1
```

### Notes

- The free tier on Render spins down after 15 minutes of inactivity; the first request after spin-down may take 30–60 seconds.
- Static files in `anomalous_inquiry/static/` are served via `tower-http`'s `ServeDir` and require no separate CDN or object storage configuration.
- Template files in `anomalous_inquiry/templates/` and content in `anomalous_inquiry/content/` are embedded at build time via the binary's working directory; ensure the start command runs from the correct working directory or adjust file paths in `src/state.rs`.
