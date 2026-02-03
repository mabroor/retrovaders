// Collision detection system

use crate::entities::*;
use crate::game::world::World;
use crate::game::state::GameState;

/// Result of collision processing
#[derive(Debug, Default)]
pub struct CollisionResult {
    pub invaders_killed: Vec<(usize, usize, u32)>,  // (row, col, points)
    pub player_hit: bool,
    pub ufo_hit: Option<u32>,  // Points if UFO was hit
}

/// Process all collisions in the world
pub fn process_collisions(world: &mut World) -> CollisionResult {
    let mut result = CollisionResult::default();

    if !world.session.is_playing() {
        return result;
    }

    // Player bullets vs invaders
    process_player_bullet_vs_invaders(world, &mut result);

    // Player bullets vs UFO
    process_player_bullet_vs_ufo(world, &mut result);

    // Player bullets vs shields
    process_player_bullets_vs_shields(world);

    // Invader bullets vs player
    process_invader_bullets_vs_player(world, &mut result);

    // Invader bullets vs shields
    process_invader_bullets_vs_shields(world);

    // Invaders vs shields (collision from marching)
    process_invaders_vs_shields(world);

    result
}

fn process_player_bullet_vs_invaders(world: &mut World, result: &mut CollisionResult) {
    for bullet in &mut world.player_bullets {
        if !bullet.alive {
            continue;
        }

        let bullet_rect = bullet.hitbox();
        if let Some((row, col)) = world.invader_grid.find_invader_at(&bullet_rect) {
            if let Some(points) = world.invader_grid.kill_at(row, col) {
                bullet.kill();
                result.invaders_killed.push((row, col, points));
            }
        }
    }
}

fn process_player_bullet_vs_ufo(world: &mut World, result: &mut CollisionResult) {
    if world.ufo.is_none() {
        return;
    }

    for bullet in &mut world.player_bullets {
        if !bullet.alive {
            continue;
        }

        if let Some(ref ufo) = world.ufo {
            if check_collision(&bullet.hitbox(), &ufo.hitbox()) {
                let points = ufo.points();
                bullet.kill();
                result.ufo_hit = Some(points);
            }
        }
    }

    // Kill UFO if hit
    if result.ufo_hit.is_some() {
        if let Some(ref mut ufo) = world.ufo {
            ufo.kill();
        }
    }
}

fn process_player_bullets_vs_shields(world: &mut World) {
    for bullet in &mut world.player_bullets {
        if !bullet.alive {
            continue;
        }

        let bullet_rect = bullet.hitbox();
        for shield in &mut world.shields {
            if let Some((hit_x, hit_y)) = shield.check_hit(&bullet_rect) {
                shield.damage_at(hit_x, hit_y, true);  // from below
                bullet.kill();
                break;
            }
        }
    }
}

fn process_invader_bullets_vs_player(world: &mut World, result: &mut CollisionResult) {
    if !world.player.can_be_hit() {
        return;
    }

    let player_rect = world.player.hitbox();

    for bullet in &mut world.invader_bullets {
        if !bullet.alive {
            continue;
        }

        if check_collision(&bullet.hitbox(), &player_rect) {
            bullet.kill();
            result.player_hit = true;
            break;
        }
    }
}

fn process_invader_bullets_vs_shields(world: &mut World) {
    for bullet in &mut world.invader_bullets {
        if !bullet.alive {
            continue;
        }

        let bullet_rect = bullet.hitbox();
        for shield in &mut world.shields {
            if let Some((hit_x, hit_y)) = shield.check_hit(&bullet_rect) {
                shield.damage_at(hit_x, hit_y, false);  // from above
                bullet.kill();
                break;
            }
        }
    }
}

fn process_invaders_vs_shields(world: &mut World) {
    for row in &world.invader_grid.invaders {
        for invader in row {
            if !invader.alive {
                continue;
            }

            let inv_rect = invader.hitbox();
            for shield in &mut world.shields {
                // If invader overlaps shield, erode the shield
                if check_collision(&inv_rect, &shield.hitbox()) {
                    // Erode from top of shield
                    for x_offset in 0..=(inv_rect.w as i32) {
                        shield.damage_at(inv_rect.x + x_offset as f32, inv_rect.y + inv_rect.h, false);
                    }
                }
            }
        }
    }
}

/// Check if game over condition is met (invaders reached bottom)
pub fn check_invasion_game_over(world: &mut World) {
    if world.invader_grid.has_reached_bottom() {
        world.session.state = GameState::GameOver;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::config::*;

    #[test]
    fn test_player_bullet_kills_invader() {
        let mut world = World::new();
        world.start_game();

        // Place bullet at first invader position
        let inv = &world.invader_grid.invaders[0][0];
        let bullet = Bullet::player_bullet(inv.x, inv.y);
        world.player_bullets.push(bullet);

        let initial_count = world.invader_grid.alive_count();
        let result = process_collisions(&mut world);

        assert_eq!(result.invaders_killed.len(), 1);
        assert_eq!(result.invaders_killed[0].2, SCORE_SQUID);  // Top row = squid
        assert_eq!(world.invader_grid.alive_count(), initial_count - 1);
    }

    #[test]
    fn test_invader_bullet_hits_player() {
        let mut world = World::new();
        world.start_game();

        // Place bullet at player position
        let bullet = Bullet::invader_bullet(world.player.x + 1.0, world.player.y);
        world.invader_bullets.push(bullet);

        let result = process_collisions(&mut world);

        assert!(result.player_hit);
    }

    #[test]
    fn test_invincible_player_not_hit() {
        let mut world = World::new();
        world.start_game();
        world.player.invincible = true;

        let bullet = Bullet::invader_bullet(world.player.x + 1.0, world.player.y);
        world.invader_bullets.push(bullet);

        let result = process_collisions(&mut world);

        assert!(!result.player_hit);
    }

    #[test]
    fn test_bullet_hits_shield() {
        let mut world = World::new();
        world.start_game();

        let shield = &world.shields[0];
        let initial_count = shield.solid_count();

        // Place bullet at shield center
        let bullet = Bullet::player_bullet(
            shield.x + SHIELD_WIDTH / 2.0,
            shield.y + SHIELD_HEIGHT / 2.0,
        );
        world.player_bullets.push(bullet);

        process_collisions(&mut world);

        assert!(world.shields[0].solid_count() < initial_count);
        assert!(world.player_bullets.is_empty() || !world.player_bullets[0].alive);
    }
}
