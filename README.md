# Neuro

> Five short browser games that score five cognitive abilities, giving you a profile and not a single IQ number.

![Rust](https://img.shields.io/badge/backend-Rust-CE412B)
![Vue 3](https://img.shields.io/badge/frontend-Vue%203-42B883)

Neuro is a web-based cognitive benchmarking app. Each game targets one narrow ability from Cattell-Horn-Carroll (CHC) theory, so you get five orthogonal scores that sketch a cognitive fingerprint. A Rust/Actix-Web backend streams fine-grained per-round metrics over WebSockets into PostgreSQL, with per-game leaderboards and a live anti-cheat layer.

## The five games

| Code | Game | CHC factor | Measures | Primary metric | Better |
|------|------|------------|----------|----------------|--------|
| `gt` | Reaction Time | Gt | Psychomotor speed | Median RT (ms) | Lower |
| `gwm` | Sequence Memory | Gwm | Working memory span | Max sequence length | Higher |
| `gs` | Symbol Matching | Gs | Processing speed | Correct per time window | Higher |
| `gf` | Pattern Logic | Gf | Fluid reasoning | Accuracy (%) | Higher |
| `gv` | Mental Rotation | Gv | Spatial visualization | Accuracy (%) | Higher |

Each task is modeled on an established psychometric paradigm (simple reaction time, Corsi span, Digit Symbol, Raven matrices, Shepard-Metzler rotation). See the bottom of this file for citations.

## Features

- **Five orthogonal scores.** Every game isolates one narrow CHC ability, so the profile stays interpretable.
- **Real-time pipeline.** Each round streams to the backend over an `actix-ws` WebSocket and lands in PostgreSQL.
- **Per-game leaderboards.** Ranked by each game's primary metric, with direction handled automatically (lower RT wins, higher span wins).
- **Live anti-cheat.** The server scores every round as it arrives and can flag or ban mid-session.
- **Clean reaction-time data.** Scoring drops anticipatory responses below 150ms and lapses above 1500ms before computing median RT.
- **JWT auth.** Register and login issue JWTs that gate the API and the WebSocket handshake (via header or `token=` query param).
- **Admin asset CRUD.** Env-gated routes (`ADMIN_API_ENABLED`) manage the `gf` and `gv` image asset groups.

## How anti-cheat works

`backend/src/services/anticheat.rs` evaluates each round against rules grouped by timing, pattern, variance, rate, and temporal checks. The rules that are live today:

- **T-001 (timing).** Reactions below 80ms are an instant ban. Below 150ms flags once and bans on a second occurrence in the session.
- **V-001 (variance).** Reaction std-dev under 5ms across 10+ trials flags as too consistent to be human.
- **P-001 (pattern, gs only).** 5+ correct Symbol Matching trials under 300ms bans, catching paced scripts that know the answer.

Escalation: one flag logs and continues, two flags in a session ban, and hard rules ban immediately. See `ANTICHEAT_DEMOS.md` for console snippets that trigger each rule.

## Tech stack

- **Backend:** Rust, Actix-Web, Tokio, `actix-ws` (WebSockets), Supabase (PostgreSQL + Auth)
- **Frontend:** Vue 3, TypeScript, Vite, Pinia, Vue Router, Tailwind CSS v4, Nuxt UI
- **Tooling:** ESLint, OxLint, Prettier, Vitest

## Architecture

<img src="assets/systems-diagram.png" alt="System Architecture Diagram" width="700">

## Quickstart

Prerequisites: Rust + Cargo (stable), Node.js 20.19+ (or 22.12+), and a Supabase project (PostgreSQL + Auth).

```bash
git clone https://github.com/lambdaf-org/neuro.git
cd neuro
```

### 1. Database

In the Supabase SQL Editor (**SQL Editor > New Query**):

1. Run `backend/src/supabase/schema.sql` to create the schema (tables, `game_metadata`, leaderboard view).
2. Run `backend/src/supabase/seed_game_assets.sql` to populate `game_metadata`, `asset_groups`, and `game_assets`. Replace `<project>` with your Supabase project ID first. Safe to re-run (it clears existing data).

Game images live in a public Supabase Storage bucket named `game-assets`, with `gf/` and `gv/` each holding `group_00..09`. URL pattern:

```
https://<project>.supabase.co/storage/v1/object/public/game-assets/<game_code>/<group>/<file>
```

### 2. Backend

Create `backend/.env`:

```
SUPABASE_URL=your_supabase_url
SUPABASE_API_KEY=your_supabase_api_key
SUPABASE_ANON_KEY=your_supabase_anon_key
HOST=127.0.0.1
PORT=8080
ADMIN_API_ENABLED=true
```

```bash
cd backend
cargo run
```

### 3. Frontend

Create `frontend/.env.local`:

```
VITE_ADMIN_UI=true
```

```bash
cd frontend
npm install
npm run dev
```

The Vite dev proxy forwards `/backend` to `http://127.0.0.1:8080`.

## Testing

```bash
cd backend && cargo test     # Rust unit tests (anti-cheat, scoring, auth, sessions)
cd frontend && npm run test:unit   # Vitest
```

## Deploying

For a deployed frontend, set the public backend origin so browser calls reach the right service:

```
VITE_API_BASE_URL=https://your-backend.up.railway.app
```

Leaving `VITE_API_BASE_URL` unset on Railway can route `/backend/login` and `/backend/register` to the frontend service and return `405 Method Not Allowed`. For the backend Railway service, set `HOST=0.0.0.0` if you are not using the Dockerfile; Railway supplies `PORT` automatically.

## Contributing

Lambdaforge is open source and contributions are welcome. Start with the [contributor guide](https://github.com/lambdaf-org/contributing), and see the org-wide [CONTRIBUTING](https://github.com/lambdaf-org/.github/blob/main/CONTRIBUTING.md) and [Code of Conduct](https://github.com/lambdaf-org/.github/blob/main/CODE_OF_CONDUCT.md).

## References

- Baddeley, A. (2003). Working memory: Looking back and looking forward. *Nature Reviews Neuroscience, 4*(10), 829-839.
- Cattell, R. B. (1963). Theory of fluid and crystallized intelligence. *Journal of Educational Psychology, 54*(1), 1-22.
- Donders, F. C. (1869). On the speed of mental processes. *Acta Psychologica, 30*, 412-431.
- Engle, R. W., et al. (1999). Working memory, short-term memory, and general fluid intelligence. *Journal of Experimental Psychology: General, 128*(3), 309-331.
- Jensen, A. R. (2006). *Clocking the Mind*. Elsevier.
- McGrew, K. S. (2009). CHC theory and the human cognitive abilities project. *Intelligence, 37*(1), 1-10.
- Raven, J. C. (1938). *Progressive Matrices*. London: H. K. Lewis.
- Salthouse, T. A. (1996). The processing-speed theory of adult age differences. *Psychological Review, 103*(3), 403-428.
- Shepard, R. N., and Metzler, J. (1971). Mental rotation of three-dimensional objects. *Science, 171*(3972), 701-703.
- Wai, J., Lubinski, D., and Benbow, C. P. (2009). Spatial ability for STEM domains. *Journal of Educational Psychology, 101*(4), 817-835.
- Wechsler, D. (1997). *WAIS-III Administration and Scoring Manual*. The Psychological Corporation.

## License

This repository does not yet include a `LICENSE` file, so default copyright applies for now. A license is coming soon. If you want to use or build on this before then, please open an issue.
