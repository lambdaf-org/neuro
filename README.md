# Neuro

**Neuro** is a full-stack web application developed as a PROG3 course project. It features a high-performance Rust backend using Actix-Web for REST APIs and WebSocket connections, a modern Vue 3 frontend with TypeScript, and Supabase for database management and authentication. The application includes real-time game event handling and an anti-cheat system.

## Architecture

<img src="assets/systems-diagram.png" alt="System Architecture Diagram" width="700">

## Tech Stack

- **Backend:** Rust, Actix-Web, Tokio, tokio-tungtenite (WebSockets), Supabase (PostgreSQL + Auth)
- **Frontend:** Vue 3, TypeScript, Vite, Pinia
- **Tooling:** ESLint, OxLint, Prettier, Vitest

## Getting Started

### Prerequisites

- **Rust & Cargo** (latest stable version)
- **Node.js** (v18 or higher)

### Backend Setup

```bash
cd backend
cargo run
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
ADMIN_API_ENABLED=true
```

## Current Status

- ✅ Admin CRUD routes with env-based toggle (`ADMIN_API_ENABLED`)
- ✅ Backend API scaffolded with Actix-Web REST endpoints
- ✅ User authentication system implemented (register/login with JWT middleware)
- ✅ Frontend initialized with Vue 3, TypeScript, and Vite scaffolding
- ✅ WebSocket infrastructure for real-time communication
- ✅ Anti-cheat worker system architecture
