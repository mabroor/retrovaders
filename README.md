# RetroVaders 🕹️

An authentic recreation of the 1978 Space Invaders arcade game, built in Rust with Macroquad.

![RetroVaders Screenshot](docs/screenshot.png)

## Features

- **Authentic Retro Experience**: CRT scanlines, phosphor bloom, color gel zones
- **Accurate Gameplay**: 55 invaders, 4 shields, UFO mystery ship
- **Original Audio**: Four-note march that speeds up as aliens are eliminated
- **Cross-Platform**: Windows, macOS, and Linux support
- **High Score Persistence**: Your achievements saved locally

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (1.70 or newer)

### Build and Run

```bash
# Clone the repository
git clone https://github.com/yourusername/retrovaders.git
cd retrovaders

# Run in debug mode
cargo run

# Build release version
cargo build --release
```

### Controls

| Key | Action |
|-----|--------|
| ← / A | Move Left |
| → / D | Move Right |
| Space / W | Fire |
| Escape / P | Pause |
| Enter | Start Game |
| R | Toggle CRT Effects |
| Q | Quit |

## Scoring

| Alien Type | Points |
|------------|--------|
| Squid (top row) | 30 |
| Crab (middle rows) | 20 |
| Octopus (bottom rows) | 10 |
| UFO (mystery ship) | 50-300 |

Extra life awarded at 1,500 points!

## Technical Details

- **Engine**: Macroquad 0.4
- **Resolution**: 224×256 (scaled 3×)
- **Frame Rate**: 60 FPS locked
- **Display**: Portrait mode (authentic to original)

### CRT Effects

Toggle with `R` key:
- Scanlines
- Phosphor bloom
- Barrel distortion
- Color gel zones (green shields, red UFO zone)

## Development

This project uses the Ralph Loop development methodology with Claude Code.

### Project Structure

```
retrovaders/
├── src/
│   ├── main.rs          # Entry point
│   ├── game/            # Game state and logic
│   ├── entities/        # Game objects
│   ├── systems/         # Game systems
│   ├── rendering/       # Graphics and shaders
│   └── audio/           # Sound system
├── assets/
│   ├── sprites/         # Pixel art
│   ├── sounds/          # Audio files
│   └── shaders/         # CRT effects
└── tests/               # Unit and integration tests
```

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
cargo clippy
cargo fmt
```

## License

MIT License - See [LICENSE](LICENSE) for details.

## Acknowledgments

- Original Space Invaders by Tomohiro Nishikado (Taito, 1978)
- Macroquad by Fedor Logachev
- Retro gaming community for visual reference

---

*"The aliens are coming. Defend Earth!"*
