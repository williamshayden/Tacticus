# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Tacticus is an AI chess training desktop app featuring "Gurgeh" - an LLM coach (named after the protagonist of Iain M. Banks' "The Player of Games"). Built with Tauri 2.0 (Rust backend) + React 19 (TypeScript frontend), featuring a Windows XP-inspired UI aesthetic.

**Key Innovation**: Uses LLM tool-calling instead of RAG - the coach queries structured chess data via SQL tools rather than semantic search. See `ARCHITECTURE.md` for details.

## Build Commands

```bash
# All commands run from tacticus-ui/
cd tacticus-ui

# Development (hot reload)
npm run tauri dev

# Production build
npm run tauri build
# Output: target/release/bundle/

# Frontend only
npm run build

# Rust tests (workspace)
cargo test --workspace

# Single crate test
cargo test -p chess-engine

# E2E tests (Playwright)
npx playwright test

# Run single E2E test
npx playwright test tests/e2e/onboarding.spec.ts
```

## Architecture

```
Tacticus/
├── crates/                      # Rust workspace
│   ├── chess-core/              # Board, moves, game state
│   ├── chess-engine/            # Position evaluation, analysis
│   ├── chess-trainer/           # Exercise generation, sessions
│   ├── chess-ai/                # Playstyle classification (ML)
│   └── chess-llm-agent/         # Gurgeh coach + OpenRouter integration
│       ├── chess_coach.rs       # ChessCoach, CoachingSession
│       ├── openrouter.rs        # API client
│       ├── tools.rs             # Tool definitions (get_recent_games, etc.)
│       └── prompts.rs           # System prompts
│
├── tacticus-ui/                 # Tauri + React app
│   ├── src/                     # React frontend
│   │   ├── components/          # UI components
│   │   │   ├── board/           # Chess board
│   │   │   ├── gurgeh/          # Chat interface
│   │   │   └── xp/              # Windows XP themed components
│   │   └── stores/              # Zustand state (userStore, gameStore, trainingStore)
│   └── src-tauri/               # Tauri Rust backend
│       └── src/commands/        # IPC command handlers (game, training, coach, user)
```

### Data Flow

React UI → Tauri Commands (IPC) → Rust crates → SQLite (local) / OpenRouter API

### Tool-Calling Pattern

The LLM coach decides what data it needs, then calls structured tools:
- `get_recent_games(n)` - Last N games with analysis
- `get_player_stats()` - Rating, win rate, playstyle
- `get_weakness_history()` - Tracked weaknesses
- `search_games_by_opening(name)` - Filter by opening
- `get_games_with_mistakes(quality)` - Games with errors
- `get_training_progress()` - Exercise completion stats

## Code Conventions

- **TypeScript**: Functional components with hooks, Zustand for state
- **Rust**: rustfmt + clippy, doc comments on public APIs
- **UI**: No emojis - use ASCII indicators like `[G]` (Gurgeh), `[!]` (Alert), `[P]` (Play)
- **CSS**: Use variables from `xp-theme.css` for XP aesthetic

## Git Workflow

- **Branch protection on main** - all changes via PR, requires code owner review (@williamshayden)
- **Conventional commits**: `feat(scope):`, `fix(scope):`, `docs:`, `refactor:`, `test:`, `chore:`
- **Branch naming**: `feature/`, `fix/`, `docs/`, `refactor/`

## Environment

OpenRouter API key required for LLM features:
- Set via `.env` file (gitignored): `OPENROUTER_API_KEY=sk-or-v1-...`
- Or configure in app Settings
- Get key at https://openrouter.ai
