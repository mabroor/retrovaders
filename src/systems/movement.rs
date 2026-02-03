// Movement system - handles entity movement

use crate::game::world::World;
use crate::input::InputState;

/// Update player movement based on input
pub fn update_player_movement(world: &mut World, input: &InputState, delta: f32) {
    if !world.session.is_playing() || !world.player.alive {
        return;
    }

    if input.move_left {
        world.player.move_left(delta);
    }
    if input.move_right {
        world.player.move_right(delta);
    }

    world.player.update(delta);
}

/// Update invader grid movement
pub fn update_invader_movement(world: &mut World, delta: f32) -> bool {
    if !world.session.is_playing() {
        return false;
    }

    let should_move = world.invader_grid.update(delta);
    if should_move {
        world.invader_grid.move_step();
    }

    should_move
}

/// Update all bullets
pub fn update_bullets(world: &mut World, delta: f32) {
    // Update player bullets
    for bullet in &mut world.player_bullets {
        bullet.update(delta);
    }

    // Update invader bullets
    for bullet in &mut world.invader_bullets {
        bullet.update(delta);
    }

    // Remove dead bullets
    world.player_bullets.retain(|b| b.alive);
    world.invader_bullets.retain(|b| b.alive);
}

/// Update UFO movement
pub fn update_ufo(world: &mut World, delta: f32) {
    if let Some(ref mut ufo) = world.ufo {
        ufo.update(delta);
        if !ufo.alive {
            world.ufo = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::Bullet;

    #[test]
    fn test_player_movement() {
        let mut world = World::new();
        world.start_game();
        let start_x = world.player.x;

        let mut input = InputState::default();
        input.move_right = true;

        update_player_movement(&mut world, &input, 0.1);

        assert!(world.player.x > start_x);
    }

    #[test]
    fn test_invader_movement() {
        let mut world = World::new();
        world.start_game();

        // Force immediate move
        world.invader_grid.move_timer = world.invader_grid.move_delay;

        let initial_x = world.invader_grid.invaders[0][0].x;
        let moved = update_invader_movement(&mut world, 0.0);

        assert!(moved);
        assert!(world.invader_grid.invaders[0][0].x != initial_x);
    }

    #[test]
    fn test_bullet_update() {
        let mut world = World::new();
        world.player_bullets.push(Bullet::player_bullet(100.0, 200.0));

        let start_y = world.player_bullets[0].y;
        update_bullets(&mut world, 0.1);

        assert!(world.player_bullets[0].y < start_y);
    }

    #[test]
    fn test_dead_bullets_removed() {
        let mut world = World::new();
        let mut bullet = Bullet::player_bullet(100.0, 200.0);
        bullet.alive = false;
        world.player_bullets.push(bullet);

        update_bullets(&mut world, 0.0);

        assert!(world.player_bullets.is_empty());
    }
}
