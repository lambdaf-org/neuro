# Neuro

**Neuro** is a web-based cognitive benchmarking platform that measures individual performance across five core domains of human cognition. Each test is modeled after established paradigms from cognitive psychology and psychometrics, and maps to a broad ability factor from the Cattell-Horn-Carroll (CHC) theory of intelligence, the most widely accepted hierarchical framework in differential psychology (McGrew, 2009).

The platform administers short, gamified tasks via a real-time WebSocket pipeline (Rust/Actix-Web backend, Vue 3 frontend) and records fine-grained performance metrics to a PostgreSQL database. Results can be compared across users via per-game leaderboards, enabling both individual self-assessment and normative comparison.

Each game targets a single narrow ability so that the resulting score profile is interpretable. A user does not receive one IQ-like number but five orthogonal data points that together sketch a cognitive fingerprint.

## Cognitive Tests

### Gt - Reaction Time

| | |
|---|---|
| **CHC Factor** | Gt, Processing and Decision Speed |
| **Domain** | Simple reaction time, psychomotor speed |
| **Task** | A visual stimulus appears after a randomized foreperiod. The user responds as quickly as possible by clicking or tapping. Multiple trials are administered to reduce noise. |
| **Metric** | Median reaction time in milliseconds (lower is better). Anticipatory responses below 150 ms and lapses above 1500 ms are excluded. |
| **Basis** | Simple and choice reaction time tasks have been used since Galton (1883) and Donders (1869) to quantify neural signal transduction and motor-response latency. Reaction time correlates moderately with general intelligence (Jensen, 2006). |

### Gwm - Sequence Memory

| | |
|---|---|
| **CHC Factor** | Gwm, Short-Term and Working Memory |
| **Domain** | Serial-order short-term memory span |
| **Task** | The system highlights a sequence of positions in order. Sequence length starts short and grows by one after each correct recall. The user reproduces the sequence in exact order. The game ends on first failure. |
| **Metric** | Maximum sequence length correctly recalled (higher is better). |
| **Basis** | Digit-span and Corsi-block paradigms are gold-standard Gwm measures (Baddeley, 2003). Span at failure predicts fluid reasoning (Engle et al., 1999). |

### Gs - Symbol Matching

| | |
|---|---|
| **CHC Factor** | Gs, Processing Speed |
| **Domain** | Perceptual speed, clerical speed and accuracy |
| **Task** | A target symbol and a set of candidates are shown. The user identifies whether the target is present as quickly and accurately as possible within a fixed time window. |
| **Metric** | Correct responses within the time limit (higher is better). |
| **Basis** | Derived from the Digit Symbol and Symbol Search paradigms (Wechsler, 1997; Salthouse, 1996), loading on CHC Gs. These are among the most sensitive measures of cognitive aging. |

### Gf - Pattern Logic

| | |
|---|---|
| **CHC Factor** | Gf, Fluid Reasoning |
| **Domain** | Inductive and figural reasoning |
| **Task** | A visual pattern with a missing element is presented. The user selects the correct completion from four options. Each asset group provides one matrix image and four option images, one of which is correct. |
| **Metric** | Accuracy as percent correct (higher is better). |
| **Basis** | Matrix-reasoning tasks (Raven, 1938; Cattell, 1963) are the prototypical markers of fluid intelligence. Gf is the single best predictor of learning and novel problem-solving. |

### Gv - Mental Rotation

| | |
|---|---|
| **CHC Factor** | Gv, Visual-Spatial Processing |
| **Domain** | Mental rotation, spatial visualization |
| **Task** | A reference figure and four candidates are shown. Exactly one candidate is a rotated version of the reference. The others are mirror-images or distractors. The user selects the correct match. |
| **Metric** | Accuracy as percent correct (higher is better), with optional per-item response time as a secondary metric. |
| **Basis** | Mental rotation tasks (Shepard and Metzler, 1971) reveal an analog spatial transformation process. Spatial visualization loads on CHC Gv and predicts STEM aptitude (Wai et al., 2009). |

### Summary

| Code | Name | CHC | Domain | Primary Metric | Direction |
|------|------|-----|--------|----------------|-----------|
| `gt` | Reaction Time | Gt | Psychomotor speed | Median RT (ms) | Lower |
| `gwm` | Sequence Memory | Gwm | Working memory span | Max sequence length | Higher |
| `gs` | Symbol Matching | Gs | Processing speed | Correct per time window | Higher |
| `gf` | Pattern Logic | Gf | Fluid reasoning | Accuracy (%) | Higher |
| `gv` | Mental Rotation | Gv | Spatial visualization | Accuracy (%) | Higher |

## Architecture

<img src="assets/systems-diagram.png" alt="System Architecture Diagram" width="700">

## Tech Stack

- **Backend:** Rust, Actix-Web, Tokio, tokio-tungtenite (WebSockets), Supabase (PostgreSQL + Auth)
- **Frontend:** Vue 3, TypeScript, Vite, Pinia
- **Tooling:** ESLint, OxLint, Prettier, Vitest

## Getting Started

### Prerequisites

- **Rust and Cargo** (latest stable version)
- **Node.js** (v18 or higher)

### Backend Setup

```bash
cd backend
cargo run
```

### Database Reset
To reset all tables, run the contents of `backend/src/supabase/schema.sql` in your Supabase dashboard under **SQL Editor > New Query**.

### Seed Data
1. Run `backend/src/supabase/schema.sql` in the Supabase SQL Editor to create the required database schema (including `game_metadata`).
2. Run `backend/src/supabase/seed_game_assets.sql` in the Supabase SQL Editor. Replace `<project>` in the seed script with your Supabase project ID before running. This populates `game_metadata`, `asset_groups`, and `game_assets`. Safe to re-run as it clears existing data first.

### Game Assets
Assets are stored in a public Supabase Storage bucket called `game-assets`.
```
game-assets/
├── gf/
│   └── group_00..09/
└── gv/
    └── group_00..09/
        ├── option_0.svg
        ├── option_1.svg
        ├── option_2.svg
        ├── option_3.svg
        └── reference.svg
```

Each group contains one prompt image and four options. For `gf` the prompt image is `matrix.svg`; for `gv` it is `reference.svg`. The public URL pattern is:
```
https://<project>.supabase.co/storage/v1/object/public/game-assets/<game_code>/<group>/<file>
```

### Frontend Setup

```bash
cd frontend
npm install
npm run dev
```

### Environment Variables

Create a `.env` file in the `backend` directory with the following variables:

```
SUPABASE_URL=your_supabase_url
SUPABASE_API_KEY=your_supabase_api_key
SUPABASE_ANON_KEY=your_supabase_anon_key
HOST=127.0.0.1
PORT=8080
CORS_ALLOWED_ORIGINS=http://localhost:5173
ADMIN_API_ENABLED=true
```

Create a `.env.local` file in the `frontend` directory with the following variable:

```
VITE_ADMIN_UI=true
```

## Testing
```bash
cd backend
cargo test
```

## Current Status

- Backend API scaffolded with Actix-Web REST endpoints
- Admin CRUD routes with env-based toggle (`ADMIN_API_ENABLED`)
- User authentication system implemented (register and login with JWT middleware)
- Frontend initialized with Vue 3, TypeScript, and Vite scaffolding
- WebSocket infrastructure for real-time communication
- Anti-cheat worker system architecture

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
