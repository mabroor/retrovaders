# Ralph Loop Prompt: RetroVaders - Rust Desktop Space Invaders

## Project Overview

Build a production-quality **Retro Space Invaders** desktop game in Rust using the Macroquad game engine. The game must authentically recreate the 1978 arcade experience with proper CRT effects, scanlines, color gel overlays, phosphor bloom, and the iconic four-note soundtrack that speeds up as aliens are eliminated.

This is a desktop application targeting Windows, macOS, and Linux with authentic retro aesthetics including portrait-mode display (rotated CRT simulation), period-accurate color zones (green shields, white text, red UFO), and the emergent gameplay mechanic where fewer aliens = faster movement.

## Subagent Architecture

This project uses a **subagent pattern** where Claude Haiku is invoked for discovery and research tasks to ensure each iteration works efficiently and uses existing codebase features where possible.

### Subagent Invocation Template

Before implementing ANY new feature, invoke a Haiku subagent for discovery:

```xml
<subagent_task model="claude-haiku-latest">
  <purpose>Discovery and research before implementation</purpose>
  <context>
    - Current codebase structure
    - Files relevant to the feature
    - Questions to answer
  </context>
  <questions>
    1. Does similar functionality already exist in the codebase?
    2. What patterns are used elsewhere that should be followed?
    3. What dependencies or modules need updating?
    4. Are there any edge cases to consider?
  </questions>
  <output_format>
    - Findings summary
    - Recommended approach
    - Files to modify/create
    - Potential blockers
  </output_format>
</subagent_task>
```

### Parallel Task Orchestration with TaskList

Use Claude Code's Task system for parallel agent execution on independent work items:

```javascript
// === ORCHESTRATION PATTERN ===

// 1. Create tasks for independent features
TaskCreate({
  subject: "Implement invader sprite rendering",
  description: "Create InvaderSprite component with animation frames for crab, octopus, squid types",
  activeForm: "Rendering invaders...",
  metadata: { "feature": "rendering", "phase": "core" }
})

TaskCreate({
  subject: "Implement player cannon controls",
  description: "Handle left/right movement and fire input with configurable keybindings",
  activeForm: "Building controls...",
  metadata: { "feature": "input", "phase": "core" }
})

// 2. Set up dependencies where needed
TaskUpdate({ taskId: "3", addBlockedBy: ["1", "2"] })

// 3. Monitor with TaskList() and coordinate completion
```

### Opus Worker Agent Spawning

For complex parallel work, spawn Opus worker agents:

```javascript
// Spawn parallel workers for independent modules
Task({
  name: "renderer-worker",
  subagent_type: "general-purpose",
  prompt: `
    You are building the CRT shader effects module.
    1. Read PRD.json section: visual_effects.crt_simulation
    2. Implement scanlines, bloom, color zones
    3. Test with cargo test --lib crt_effects
    4. Mark TaskUpdate when complete
  `,
  run_in_background: true
})

Task({
  name: "audio-worker", 
  subagent_type: "general-purpose",
  prompt: `
    You are building the audio engine.
    1. Read PRD.json section: audio
    2. Implement four-note march, explosion sounds
    3. Speed scaling based on remaining invaders
    4. Mark TaskUpdate when complete
  `,
  run_in_background: true
})
```

---

## Session Protocol

### Session Start Ritual

Every session MUST begin with these steps:

1. **Orient**: Run `pwd && ls -la` to confirm location
2. **Read Progress**: `cat claude-progress.txt | tail -100`
3. **Check Features**: `cat features.json | jq '.features[] | select(.status == "in_progress" or .status == "pending") | {id, name, status}'`
4. **Run Tests**: `cargo test --lib 2>&1 | tail -50`
5. **Git Status**: `git status && git log --oneline -5`

### Session End Ritual

Every session MUST end with:

1. **Commit Clean**: `git add -A && git commit -m "feat: [description]"`
2. **Update Progress**: Append to `claude-progress.txt`
3. **Update Features**: Mark completed features in `features.json`
4. **Run Tests**: Ensure all tests pass
5. **Document Blockers**: Note any blockers for next session

---

## Implementation Order

### Phase 1: Foundation (Features 1-6)
- Rust project setup with Macroquad
- Window configuration (portrait mode 224x256 scaled)
- Basic game loop structure
- Asset loading system
- Input handling framework
- Entity component system (simple, no ECS crate needed)

### Phase 2: Core Gameplay (Features 7-16)
- Player cannon sprite and movement
- Invader grid (5 rows × 11 columns = 55 aliens)
- Invader movement pattern (left→drop→right→drop→repeat)
- Speed acceleration when aliens eliminated
- Player shooting mechanics
- Invader shooting mechanics
- Collision detection (pixel-perfect)
- Shield/bunker system with erosion
- Lives system (3 lives default)
- Score tracking

### Phase 3: Polish (Features 17-24)
- UFO/mystery ship (random spawn, 50-300 points)
- CRT shader effects (scanlines, bloom, curvature)
- Color gel overlay zones (green/red/white)
- Four-note march audio (speeds up with fewer invaders)
- Sound effects (shoot, explosion, UFO)
- High score persistence
- Game over/restart flow
- Attract mode/demo

### Phase 4: Production (Features 25-32)
- Cross-platform builds (Windows/macOS/Linux)
- Configurable settings (controls, difficulty)
- Frame rate limiting (60 FPS locked)
- Performance optimization
- Unit test coverage >80%
- Integration tests
- README documentation
- Release packaging

---

## Technical Specifications

### Engine Choice: Macroquad

Macroquad is selected for:
- Minimal dependencies (fast compile times)
- Cross-platform with single codebase
- Immediate-mode rendering suitable for retro games
- Built-in audio support
- Easy shader integration for CRT effects

### Project Structure

```
space-invaders/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, game loop
│   ├── lib.rs               # Library exports for testing
│   ├── game/
│   │   ├── mod.rs
│   │   ├── state.rs         # GameState enum, transitions
│   │   ├── world.rs         # World containing all entities
│   │   └── config.rs        # Game configuration constants
│   ├── entities/
│   │   ├── mod.rs
│   │   ├── player.rs        # Player cannon
│   │   ├── invader.rs       # Invader types and grid
│   │   ├── bullet.rs        # Player and invader bullets
│   │   ├── shield.rs        # Destructible bunkers
│   │   └── ufo.rs           # Mystery ship
│   ├── systems/
│   │   ├── mod.rs
│   │   ├── movement.rs      # Movement logic
│   │   ├── collision.rs     # Collision detection
│   │   ├── shooting.rs      # Fire logic
│   │   └── scoring.rs       # Score management
│   ├── rendering/
│   │   ├── mod.rs
│   │   ├── sprites.rs       # Sprite definitions
│   │   ├── animation.rs     # Frame animation
│   │   └── crt.rs           # CRT shader effects
│   ├── audio/
│   │   ├── mod.rs
│   │   └── sounds.rs        # Audio playback
│   └── input/
│       ├── mod.rs
│       └── keyboard.rs      # Input handling
├── assets/
│   ├── sprites/
│   │   ├── invader_crab.png
│   │   ├── invader_octopus.png
│   │   ├── invader_squid.png
│   │   ├── player.png
│   │   ├── bullet.png
│   │   ├── shield.png
│   │   └── ufo.png
│   ├── sounds/
│   │   ├── shoot.wav
│   │   ├── explosion.wav
│   │   ├── invader_killed.wav
│   │   ├── ufo.wav
│   │   └── march_[1-4].wav
│   └── shaders/
│       └── crt.glsl
├── tests/
│   ├── collision_tests.rs
│   ├── movement_tests.rs
│   └── scoring_tests.rs
├── features.json
├── claude-progress.txt
├── PRD.json
└── README.md
```

### Core Constants

```rust
// Display (original arcade was 224x256, portrait mode)
pub const GAME_WIDTH: u32 = 224;
pub const GAME_HEIGHT: u32 = 256;
pub const SCALE: u32 = 3;  // 3x upscale = 672x768 window

// Timing
pub const TARGET_FPS: u32 = 60;
pub const BASE_INVADER_MOVE_DELAY: f32 = 0.8;  // seconds at full grid
pub const MIN_INVADER_MOVE_DELAY: f32 = 0.05;  // seconds with 1 invader

// Invader Grid
pub const INVADER_ROWS: usize = 5;
pub const INVADER_COLS: usize = 11;
pub const INVADER_TOTAL: usize = 55;

// Scoring (original arcade values)
pub const SCORE_SQUID: u32 = 30;      // Top row
pub const SCORE_CRAB: u32 = 20;       // Middle rows
pub const SCORE_OCTOPUS: u32 = 10;    // Bottom rows
pub const SCORE_UFO_MIN: u32 = 50;
pub const SCORE_UFO_MAX: u32 = 300;

// Player
pub const PLAYER_SPEED: f32 = 120.0;  // pixels per second
pub const PLAYER_LIVES: u32 = 3;
pub const PLAYER_BULLET_SPEED: f32 = 400.0;

// Shields
pub const NUM_SHIELDS: usize = 4;
```

---

## CRT Effect Specifications

### Color Zones (Original Gel Overlay)

```rust
pub struct ColorZone {
    y_start: u32,
    y_end: u32,
    color: Color,
}

pub const COLOR_ZONES: [ColorZone; 4] = [
    // UFO zone - red
    ColorZone { y_start: 0, y_end: 32, color: Color::from_rgba(255, 0, 0, 255) },
    // Play area - white (with blue phosphor tint)
    ColorZone { y_start: 32, y_end: 184, color: Color::from_rgba(200, 200, 255, 255) },
    // Shield zone - green
    ColorZone { y_start: 184, y_end: 240, color: Color::from_rgba(0, 255, 0, 255) },
    // HUD zone - white
    ColorZone { y_start: 240, y_end: 256, color: Color::from_rgba(255, 255, 255, 255) },
];
```

### Scanline Shader

```glsl
// CRT scanline effect (simplified)
uniform float scanline_weight;  // 0.1 = subtle, 0.3 = pronounced
uniform float bloom_strength;   // 0.5 = subtle glow

void main() {
    vec4 color = texture(tex, uv);
    
    // Scanlines (every other line darker)
    float scanline = sin(uv.y * resolution.y * 3.14159) * 0.5 + 0.5;
    color.rgb *= mix(1.0, scanline, scanline_weight);
    
    // Bloom (brighten based on luminance)
    float luma = dot(color.rgb, vec3(0.299, 0.587, 0.114));
    if (luma > 0.4) {
        color.rgb += color.rgb * bloom_strength * (luma - 0.4);
    }
    
    // Slight barrel distortion for CRT curvature
    vec2 curved_uv = uv * 2.0 - 1.0;
    float dist = length(curved_uv);
    curved_uv *= 1.0 + dist * dist * 0.02;
    
    gl_FragColor = color;
}
```

---

## Audio System Specifications

### Four-Note March

The iconic Space Invaders march consists of four notes that play sequentially as aliens move. The tempo increases as fewer aliens remain:

```rust
pub struct MarchAudio {
    notes: [Sound; 4],
    current_note: usize,
    base_interval: f32,
    
    fn calculate_interval(&self, remaining_invaders: usize) -> f32 {
        // Original arcade behavior: faster as aliens eliminated
        let ratio = remaining_invaders as f32 / INVADER_TOTAL as f32;
        self.base_interval * ratio.max(0.1)  // Floor at 10% speed
    }
    
    fn play_next(&mut self) {
        play_sound_once(&self.notes[self.current_note]);
        self.current_note = (self.current_note + 1) % 4;
    }
}
```

---

## Critical Rules

### DO
- ✅ Run discovery subagent BEFORE implementing any feature
- ✅ Use TaskList for parallel independent work
- ✅ Keep each session focused on 1-3 features maximum
- ✅ Write tests BEFORE or WITH implementation
- ✅ Commit after every completed feature
- ✅ Update claude-progress.txt at session end
- ✅ Follow Rust idioms (ownership, Result types, clippy clean)

### DON'T
- ❌ Skip the session start ritual
- ❌ Implement without checking existing patterns
- ❌ Leave code in broken state at session end
- ❌ Make large changes without intermediate commits
- ❌ Ignore test failures
- ❌ Add dependencies without justification

---

## Anti-Patterns to Avoid

| Anti-Pattern | Why It's Bad | Do This Instead |
|--------------|--------------|-----------------|
| One-shotting the game | Runs out of context mid-way | Incremental feature-by-feature |
| Skipping subagent research | Duplicates existing code | Always discover first |
| Global state everywhere | Hard to test, race conditions | Pass state explicitly |
| Raw pixel manipulation | Slow, error-prone | Use Macroquad primitives |
| Blocking audio | Game freezes | Async audio playback |
| Fixed timestep issues | Different speeds on hardware | Delta-time based updates |

---

## Blocker Handling

When blocked, document in this format:

```markdown
### BLOCKER: [Title]
- **Severity**: Critical/High/Medium/Low
- **Description**: What's blocking progress
- **Attempted Solutions**: What was tried
- **Hypothesis**: Likely root cause
- **Next Steps**: Suggested approach
- **Files Affected**: List of files
```

---

## Testing Strategy

### Unit Tests (80% coverage goal)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_invader_movement_direction_change() {
        let mut grid = InvaderGrid::new();
        grid.move_to_edge();
        assert!(grid.should_drop());
    }
    
    #[test]
    fn test_collision_player_invader_bullet() {
        let player = Player::at(100.0, 220.0);
        let bullet = Bullet::at(102.0, 218.0);
        assert!(player.collides_with(&bullet));
    }
    
    #[test]
    fn test_score_calculation() {
        assert_eq!(score_for_invader(InvaderType::Squid), 30);
        assert_eq!(score_for_invader(InvaderType::Crab), 20);
        assert_eq!(score_for_invader(InvaderType::Octopus), 10);
    }
}
```

### Integration Tests

```rust
// tests/game_flow_tests.rs
#[test]
fn test_full_wave_clear() {
    let mut game = Game::new();
    game.eliminate_all_invaders();
    assert_eq!(game.state(), GameState::WaveComplete);
}

#[test]
fn test_game_over_on_invasion() {
    let mut game = Game::new();
    game.move_invaders_to_bottom();
    assert_eq!(game.state(), GameState::GameOver);
}
```

---

## Completion Promise

The loop continues until **BUILD_COMPLETE** signal is emitted, which requires:

1. All 32 features in `features.json` marked `"status": "completed"`
2. `cargo test` passes with 0 failures
3. `cargo clippy` has 0 warnings
4. Game runs on all three platforms (verified via cargo build --release)
5. README.md is complete with build instructions
6. High score persistence works across sessions

---

## Reference: Original Space Invaders Facts

| Aspect | Original 1978 Arcade |
|--------|---------------------|
| Resolution | 224 × 256 pixels (portrait) |
| Colors | Monochrome with colored gel overlays |
| Aliens | 55 (5 rows × 11 columns) |
| Alien Types | Squid (30pts), Crab (20pts), Octopus (10pts) |
| UFO | Random spawn, 50-300 points |
| Shields | 4 destructible bunkers |
| Lives | 3 (configurable via DIP switch) |
| CPU | Intel 8080 @ 2MHz |
| Sound | Texas Instruments SN76477 |
| Speed Quirk | Faster movement with fewer aliens (processor had less work) |

---

## Session Log Format

```markdown
## Session [N] - [Date]

### Goals
- [ ] Feature X
- [ ] Feature Y

### Completed
- [x] Feature X
  - Files: src/entities/player.rs
  - Tests: tests/player_tests.rs
  - Notes: Used existing movement pattern from config

### Blockers
- None | [Blocker description]

### Next Session
- Feature Y
- Feature Z

### Metrics
- Tests: 45 passing, 0 failing
- Coverage: 72%
- Build: ✅ Clean
```
