# RetroVaders

**An authentic recreation of the 1978 Space Invaders arcade game, built in Rust.**

```
    ▀▄▀▄▀▄ RETROVADERS ▄▀▄▀▄▀

      /▔▔\    /▔▔\    /▔▔\
     |◉  ◉|  |◉  ◉|  |◉  ◉|    ← Squids (30 pts)
      \▂▂/    \▂▂/    \▂▂/

      ╔══╗    ╔══╗    ╔══╗
      ║◉◉║    ║◉◉║    ║◉◉║     ← Crabs (20 pts)
      ╚══╝    ╚══╝    ╚══╝

      {@@}    {@@}    {@@}
      /  \    /  \    /  \     ← Octopi (10 pts)

     ████  ████  ████  ████    ← Shields

           ▄▄███▄▄               ← You
```

## Features

- **Authentic Gameplay** - 55 invaders in classic 5×11 formation with the iconic march pattern
- **Original Scoring** - Squid (30), Crab (20), Octopus (10), Mystery UFO (50-300)
- **Speed Acceleration** - Invaders speed up as they're eliminated (original arcade quirk!)
- **CRT Effects** - Scanlines, color gel zones, and vignette (toggle with F1)
- **Destructible Shields** - 4 bunkers with pixel-perfect erosion
- **High Score Persistence** - Your scores are saved locally
- **Cross-Platform** - Windows, macOS, and Linux

## Quick Start

```bash
# Install Rust if needed: https://rustup.rs

# Clone and run
git clone https://github.com/mabroor/retrovaders.git
cd retrovaders
cargo run --release
```

## Controls

| Action | Keys |
|--------|------|
| Move | **←** **→** or **A** **D** |
| Fire | **Space** or **W** |
| Pause | **Esc** or **P** |
| CRT Toggle | **F1** |
| Quit | **Q** |

## Screenshots

The game features authentic CRT simulation with:
- **Red zone** (top) - UFO territory
- **White zone** (middle) - Main play area
- **Green zone** (bottom) - Shields and player
- **Scanlines** - Classic CRT effect

## Building

### Prerequisites
- [Rust](https://rustup.rs/) 1.70+

### Commands
```bash
cargo build --release    # Optimized build
cargo test               # Run 110 tests
cargo run                # Debug build & run
```

## Technical Details

| Spec | Value |
|------|-------|
| Resolution | 224×256 (3× scaled to 672×768) |
| Framerate | 60 FPS |
| Engine | [Macroquad](https://macroquad.rs/) |
| Tests | 110 (93 unit + 17 integration) |

### Project Structure
```
src/
├── main.rs          # Game loop
├── game/            # State, config, world
├── entities/        # Player, invaders, bullets, shields, UFO
├── systems/         # Movement, collision, shooting, scoring
├── rendering/       # Sprites, CRT effects
├── audio/           # Sound management
└── input/           # Keyboard handling
```

## Original Arcade Facts

The original Space Invaders (1978) by Taito:
- Used an Intel 8080 CPU at 2MHz
- The speed-up when aliens die was a **happy accident** - fewer sprites = faster rendering!
- Colored gel strips over the monochrome CRT created the distinctive zones
- Earned over $2 billion in quarters by 1982

## Configuration

Settings and high scores are saved to:
- **Linux**: `~/.local/share/retrovaders/`
- **macOS**: `~/Library/Application Support/retrovaders/`
- **Windows**: `%APPDATA%\retrovaders\`

## License

MIT License - See [LICENSE](LICENSE) for details.

---

*Built with Rust and nostalgia*
