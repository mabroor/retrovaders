# CLAUDE.md - RetroVaders Project Context

## Project Overview

RetroVaders is a Rust-based desktop Space Invaders game built with Macroquad. The goal is authentic recreation of the 1978 arcade experience with CRT effects, period-accurate color zones, and the iconic accelerating march soundtrack.

## Quick Commands

```bash
# Build and run
cargo run

# Run tests
cargo test

# Check for issues
cargo clippy

# Release build
cargo build --release

# Check feature progress
cat features.json | jq '.summary'
```

## Key Files

| File | Purpose |
|------|---------|
| `RALPH_PROMPT.md` | Main development prompt with subagent patterns |
| `PRD.json` | Product requirements document |
| `features.json` | Feature tracking (32 features, 4 phases) |
| `claude-progress.txt` | Session-by-session progress log |

## Architecture

```
src/
├── main.rs          # Entry point
├── game/            # Game state, world, config
├── entities/        # Player, invaders, bullets, shields, UFO
├── systems/         # Movement, collision, shooting, scoring
├── rendering/       # Sprites, animation, CRT effects
├── audio/           # Sound playback
└── input/           # Keyboard handling
```

## Tech Stack

- **Language**: Rust
- **Engine**: Macroquad (chosen for simplicity, fast compile, cross-platform)
- **Audio**: Macroquad audio module
- **Persistence**: serde_json for high scores

## Key Constants

```rust
GAME_WIDTH: 224
GAME_HEIGHT: 256
SCALE: 3
TARGET_FPS: 60
INVADER_TOTAL: 55 (5 rows × 11 columns)
PLAYER_LIVES: 3
```

## Scoring

| Target | Points |
|--------|--------|
| Squid (top row) | 30 |
| Crab (middle rows) | 20 |
| Octopus (bottom rows) | 10 |
| UFO | 50-300 |

## Session Protocol

1. **Start**: `pwd && ls -la && cat claude-progress.txt | tail -50`
2. **Work**: Focus on 1-3 features max
3. **End**: Commit, update progress file, note blockers

## Subagent Usage

Always run Haiku discovery before implementing:
- Check existing patterns
- Verify dependencies
- Identify edge cases

## Current Phase

Phase 1: Foundation (Features F001-F006)
- Project setup
- Window config
- Game loop
- Asset loading
- Input handling
- Entity system
