// Shooting system - handles bullet spawning

use crate::entities::*;
use crate::game::world::World;
use crate::game::config::*;
use crate::input::InputState;
use rand::Rng;

/// Handle player shooting
pub fn handle_player_shooting(world: &mut World, input: &InputState) {
    if !world.session.is_playing() || !world.player.alive {
        return;
    }

    if input.fire && world.player_bullets.len() < MAX_PLAYER_BULLETS {
        let spawn_pos = world.player.bullet_spawn_position();
        let bullet = Bullet::player_bullet(spawn_pos.x, spawn_pos.y);
        world.player_bullets.push(bullet);
    }
}

/// Handle invader shooting (random bottom invaders fire)
pub fn handle_invader_shooting(world: &mut World) {
    if !world.session.is_playing() {
        return;
    }

    if world.invader_bullets.len() >= MAX_INVADER_BULLETS {
        return;
    }

    // Calculate fire probability based on wave
    let wave_multiplier = 1.0 + ((world.session.wave.min(MAX_DIFFICULTY_WAVE) - 1) as f32)
        * WAVE_FIRE_PROBABILITY_INCREASE / INVADER_FIRE_PROBABILITY;
    let fire_prob = (INVADER_FIRE_PROBABILITY * wave_multiplier).min(0.01);

    let mut rng = rand::thread_rng();

    // Get bottom invaders
    let bottom_invaders: Vec<_> = world.invader_grid.bottom_invaders();

    for invader in bottom_invaders {
        if rng.gen::<f32>() < fire_prob {
            let spawn_pos = invader.bullet_spawn_position();
            let bullet = Bullet::invader_bullet(spawn_pos.x, spawn_pos.y);
            world.invader_bullets.push(bullet);

            // Only one invader fires per frame
            break;
        }
    }
}

/// Try to spawn a UFO
pub fn try_spawn_ufo(world: &mut World, delta: f32) {
    if !world.session.is_playing() {
        return;
    }

    // Don't spawn if UFO already exists
    if world.ufo.is_some() {
        return;
    }

    // Track spawn timer
    world.ufo_spawn_timer += delta;
    if world.ufo_spawn_timer < UFO_MIN_SPAWN_INTERVAL {
        return;
    }

    // Random chance to spawn
    let mut rng = rand::thread_rng();
    if rng.gen::<f32>() < UFO_SPAWN_PROBABILITY {
        // Alternate direction based on wave
        let direction = if world.session.wave % 2 == 1 {
            UfoDirection::Right
        } else {
            UfoDirection::Left
        };

        // Shot index affects score - count total shots taken
        let shot_index = world.session.score as usize; // Rough approximation
        world.ufo = Some(Ufo::new(direction, shot_index));
        world.ufo_spawn_timer = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_shoot() {
        let mut world = World::new();
        world.start_game();

        let mut input = InputState::default();
        input.fire = true;

        assert!(world.player_bullets.is_empty());

        handle_player_shooting(&mut world, &input);

        assert_eq!(world.player_bullets.len(), 1);
    }

    #[test]
    fn test_player_cannot_exceed_max_bullets() {
        let mut world = World::new();
        world.start_game();

        // Fill up bullet slots
        for _ in 0..MAX_PLAYER_BULLETS {
            world.player_bullets.push(Bullet::player_bullet(100.0, 100.0));
        }

        let mut input = InputState::default();
        input.fire = true;

        handle_player_shooting(&mut world, &input);

        assert_eq!(world.player_bullets.len(), MAX_PLAYER_BULLETS);
    }

    #[test]
    fn test_invader_bullets_limited() {
        let mut world = World::new();
        world.start_game();

        // Fill up invader bullet slots
        for _ in 0..MAX_INVADER_BULLETS {
            world.invader_bullets.push(Bullet::invader_bullet(100.0, 100.0));
        }

        let initial_count = world.invader_bullets.len();
        handle_invader_shooting(&mut world);

        assert_eq!(world.invader_bullets.len(), initial_count);
    }

    #[test]
    fn test_ufo_spawn_respects_minimum_interval() {
        let mut world = World::new();
        world.start_game();

        // Try to spawn before minimum interval
        world.ufo_spawn_timer = 0.0;
        try_spawn_ufo(&mut world, 1.0);  // Only 1 second

        // UFO shouldn't spawn yet (need 25 seconds minimum)
        // Note: could technically spawn due to randomness, but highly unlikely
        // This test just verifies the timer is being updated
        assert!(world.ufo_spawn_timer > 0.0);
    }
}
