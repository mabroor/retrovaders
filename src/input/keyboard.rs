// Keyboard input handling

use macroquad::prelude::*;

/// Current input state for this frame
#[derive(Debug, Clone, Default)]
pub struct InputState {
    pub move_left: bool,
    pub move_right: bool,
    pub fire: bool,
    pub pause: bool,
    pub start: bool,
    pub quit: bool,
}

/// Configurable key bindings
#[derive(Debug, Clone)]
pub struct KeyBindings {
    pub move_left: Vec<KeyCode>,
    pub move_right: Vec<KeyCode>,
    pub fire: Vec<KeyCode>,
    pub pause: Vec<KeyCode>,
    pub start: Vec<KeyCode>,
    pub quit: Vec<KeyCode>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_left: vec![KeyCode::Left, KeyCode::A],
            move_right: vec![KeyCode::Right, KeyCode::D],
            fire: vec![KeyCode::Space, KeyCode::W, KeyCode::Up],
            pause: vec![KeyCode::Escape, KeyCode::P],
            start: vec![KeyCode::Enter, KeyCode::Space],
            quit: vec![KeyCode::Q],
        }
    }
}

/// Input handler with configurable bindings
pub struct InputHandler {
    pub bindings: KeyBindings,
    prev_fire: bool,
    prev_pause: bool,
    prev_start: bool,
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            bindings: KeyBindings::default(),
            prev_fire: false,
            prev_pause: false,
            prev_start: false,
        }
    }

    pub fn with_bindings(bindings: KeyBindings) -> Self {
        Self {
            bindings,
            prev_fire: false,
            prev_pause: false,
            prev_start: false,
        }
    }

    /// Poll current input state
    pub fn poll(&mut self) -> InputState {
        let fire_pressed = self.is_any_key_down(&self.bindings.fire);
        let pause_pressed = self.is_any_key_down(&self.bindings.pause);
        let start_pressed = self.is_any_key_down(&self.bindings.start);

        let state = InputState {
            move_left: self.is_any_key_down(&self.bindings.move_left),
            move_right: self.is_any_key_down(&self.bindings.move_right),
            // Fire only triggers on press, not hold
            fire: fire_pressed && !self.prev_fire,
            // Pause only triggers on press
            pause: pause_pressed && !self.prev_pause,
            // Start only triggers on press
            start: start_pressed && !self.prev_start,
            quit: self.is_any_key_down(&self.bindings.quit),
        };

        self.prev_fire = fire_pressed;
        self.prev_pause = pause_pressed;
        self.prev_start = start_pressed;

        state
    }

    /// Check if any of the given keys are down
    fn is_any_key_down(&self, keys: &[KeyCode]) -> bool {
        keys.iter().any(|&key| is_key_down(key))
    }

    /// Check if any key is pressed this frame
    pub fn any_key_pressed(&self) -> bool {
        get_last_key_pressed().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_bindings() {
        let bindings = KeyBindings::default();

        assert!(bindings.move_left.contains(&KeyCode::Left));
        assert!(bindings.move_left.contains(&KeyCode::A));
        assert!(bindings.move_right.contains(&KeyCode::Right));
        assert!(bindings.move_right.contains(&KeyCode::D));
        assert!(bindings.fire.contains(&KeyCode::Space));
        assert!(bindings.pause.contains(&KeyCode::Escape));
        assert!(bindings.pause.contains(&KeyCode::P));
    }

    #[test]
    fn test_input_handler_new() {
        let handler = InputHandler::new();
        assert!(!handler.prev_fire);
        assert!(!handler.prev_pause);
    }

    #[test]
    fn test_input_state_default() {
        let state = InputState::default();
        assert!(!state.move_left);
        assert!(!state.move_right);
        assert!(!state.fire);
        assert!(!state.pause);
        assert!(!state.start);
        assert!(!state.quit);
    }
}
