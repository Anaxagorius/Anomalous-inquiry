
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
- Phoenix Lights (1997)
- Rendlesham Forest incident (1980)
- Belgian Wave (1989–91)
- Close encounter archive (CE1–CE5) & timeline

### Survival Research
- Near-death experiences (NDE)
- Shared death experiences (SDE) — deathbed visions, after-death communications, peak-in-Darien cases
- Mind-brain independence

### Altered States & Exceptional Experiences
- Out-of-body experiences (OBE)
- Dream telepathy
- Lucid dreaming (parapsychological context)
- Hypnagogic and hypnopompic states
- Trance states
- Non-pathological dissociative experiences
- Visionary experiences

### Non-Human Intelligences (NHI)
- Hub overview and typology framework
- Extraterrestrial hypothesis (Greys, Nordics, Reptilians, Mantids, Avian)
- Ultra-terrestrial hypothesis (Tonnies, Agartha, Dulce)
- Interdimensional hypothesis (Vallée, Keel, Shadow Beings, Skinwalker Ranch)
- Plasma entities (Hessdalen Project, Marfa Lights, ball lightning)
- Orbs (foo fighters, crop circle orbs, Congressional testimony)
- Artificial intelligences / probes (Bracewell/Von Neumann, biomechanical entities)
- Hybrid beings (Hopkins, Jacobs, Mack, CERO research)
- Ancient intelligences (Anunnaki, Watchers/Nephilim, Vimanas, Djinn)
- Consciousness-based entities (Monroe Institute, DMT entities, Philip Experiment)
- Trickster hypothesis (Vallée Control System, Keel/Mothman, MIBs)
- Aquatic/USO phenomena (Soviet Navy files, Lake Baikal, USS Trepang)
- Other/unclassified encounters
- Documentation archive (AARO, AATIP, congressional hearings, FOIA releases)

## Features
- Comprehensive static topic pages covering all subjects with sourced, multi-section reference content
- Reflections journal (admin-only, cookie-authenticated) with guest-visible reflections
- CE1–CE5 archive and interactive timeline (108+ chronological entries from antiquity to 2023)
- Organizations directory (36+ research institutions, government programmes, and journals)
- No frontend JavaScript (Tailwind CDN via `<script>` tag for styling only)

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (2021 edition) |
| HTTP framework | Axum 0.7 |
| Templating | Askama (compile-time) |
| Authentication | Argon2 + JWT (`jsonwebtoken`) |
| Async runtime | Tokio |
| Date handling | Chrono |
| Cookies | tower-cookies |

## Local Development

```bash
# Clone the repository
git clone https://github.com/Anaxagorius/Anomalous-inquiry.git
cd Anomalous-inquiry/anomalous_inquiry

# Run in development mode
cargo run

# The server starts at http://localhost:8080
```

All content (organizations, timeline) is loaded from JSON files in `content/` at startup. Topic pages are compiled into the binary via Askama templates.

### Admin Access

The reflections journal is available at `/admin`. Authentication uses a JWT cookie set by a `POST /admin/login` form. The default password is `anomalous2024!` and the server prompts a forced password change on first login. Set a custom password via the `ADMIN_PASSWORD` environment variable or through the change-password UI.

### Content Data

- **Timeline:** Edit `content/timeline.json` — array of `{ date, title, description, category, sources }` objects sorted chronologically.
- **Organizations:** Edit `content/organizations.json` — array of `{ name, category, description, website?, founded?, notes? }` objects.
- **Journal entries:** Persisted to `data/journal_entries.json` (path configurable via `JOURNAL_DATA_FILE` env var).

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
