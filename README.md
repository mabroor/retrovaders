# RetroVaders

An authentic recreation of the 1978 Space Invaders arcade game, built in Rust using the Macroquad game engine.

## Features

- **Authentic Gameplay**: 55 invaders (5 rows × 11 columns) with the classic march pattern
- **Original Scoring**: Squid (30pts), Crab (20pts), Octopus (10pts), Mystery UFO (50-300pts)
- **Speed Acceleration**: Invaders move faster as they're eliminated (original arcade behavior)
- **CRT Effects**: Scanlines, color zones, and vignette effects (toggleable with F1)
- **Destructible Shields**: 4 bunkers with pixel-based erosion
- **Lives System**: 3 lives, extra life at 1500 points
- **High Score Persistence**: Local save file for high scores
- **Cross-Platform**: Runs on Windows, macOS, and Linux

## Controls

| Action | Keys |
|--------|------|
| Move Left | ← / A |
| Move Right | → / D |
| Fire | Space / W / ↑ |
| Pause | Escape / P |
| Toggle CRT | F1 |
| Quit | Q |

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) (1.70 or later)

### Build & Run

```bash
# Clone the repository
git clone https://github.com/yourusername/retrovaders.git
cd retrovaders

# Debug build and run
cargo run

# Release build (optimized)
cargo build --release
./target/release/retrovaders
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

## Technical Specifications

### Display

- Native resolution: 224×256 (portrait mode)
- Window size: 672×768 (3× scaling)
- Target framerate: 60 FPS

### Original Arcade Facts

| Aspect | Original 1978 Arcade |
|--------|---------------------|
| Resolution | 224 × 256 pixels (portrait) |
| Colors | Monochrome with colored gel overlays |
| Aliens | 55 (5 rows × 11 columns) |
| Alien Types | Squid (30pts), Crab (20pts), Octopus (10pts) |
| UFO | Random spawn, 50-300 points |
| Shields | 4 destructible bunkers |
| Lives | 3 (configurable) |
| Speed Quirk | Faster movement with fewer aliens |

## Project Structure

```
retrovaders/
├── Cargo.toml              # Dependencies and build config
├── src/
│   ├── main.rs             # Entry point, game loop
│   ├── lib.rs              # Library exports
│   ├── game/
│   │   ├── config.rs       # Game constants
│   │   ├── state.rs        # Game state machine
│   │   ├── world.rs        # World containing entities
│   │   └── settings.rs     # Configurable settings
│   ├── entities/
│   │   ├── player.rs       # Player cannon
│   │   ├── invader.rs      # Invaders and grid
│   │   ├── bullet.rs       # Projectiles
│   │   ├── shield.rs       # Destructible bunkers
│   │   └── ufo.rs          # Mystery ship
│   ├── systems/
│   │   ├── movement.rs     # Entity movement
│   │   ├── collision.rs    # Collision detection
│   │   ├── shooting.rs     # Fire mechanics
│   │   └── scoring.rs      # Score and high scores
│   ├── rendering/
│   │   ├── mod.rs          # Rendering functions
│   │   ├── crt.rs          # CRT shader effects
│   │   ├── sprites.rs      # Sprite management
│   │   └── animation.rs    # Animation system
│   ├── audio/
│   │   └── sounds.rs       # Audio playback
│   └── input/
│       └── keyboard.rs     # Input handling
├── assets/
│   ├── shaders/            # GLSL shaders
│   ├── sprites/            # Game graphics
│   └── sounds/             # Audio files
└── tests/
    └── integration_tests.rs
```

## Configuration

Settings are saved to:
- **Linux**: `~/.local/share/retrovaders/`
- **macOS**: `~/Library/Application Support/retrovaders/`
- **Windows**: `%APPDATA%\retrovaders\`

Files:
- `settings.json` - Game configuration
- `high_scores.json` - High score table

## Dependencies

- [Macroquad](https://macroquad.rs/) - Game engine
- [Serde](https://serde.rs/) - Serialization
- [Rand](https://crates.io/crates/rand) - Random numbers
- [Dirs](https://crates.io/crates/dirs) - Cross-platform directories

## License

MIT License - See [LICENSE](LICENSE) for details.

## Acknowledgments

- Original Space Invaders © 1978 Taito
- Inspired by the classic arcade experience
