// Animation system for sprite frame cycling

/// Simple animation state
#[derive(Debug, Clone)]
pub struct Animation {
    pub frames: usize,
    pub current_frame: usize,
    pub frame_time: f32,
    pub timer: f32,
    pub looping: bool,
    pub finished: bool,
}

impl Animation {
    pub fn new(frames: usize, frame_time: f32, looping: bool) -> Self {
        Self {
            frames,
            current_frame: 0,
            frame_time,
            timer: 0.0,
            looping,
            finished: false,
        }
    }

    /// Two-frame animation that swaps on external trigger (like invader movement)
    pub fn two_frame() -> Self {
        Self::new(2, 0.0, true)
    }

    /// Update animation, returns true if frame changed
    pub fn update(&mut self, delta: f32) -> bool {
        if self.finished {
            return false;
        }

        self.timer += delta;

        if self.timer >= self.frame_time && self.frame_time > 0.0 {
            self.timer = 0.0;
            self.current_frame += 1;

            if self.current_frame >= self.frames {
                if self.looping {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.frames - 1;
                    self.finished = true;
                }
            }

            return true;
        }

        false
    }

    /// Manually advance to next frame
    pub fn next_frame(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.frames;
    }

    /// Reset animation to start
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.timer = 0.0;
        self.finished = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_new() {
        let anim = Animation::new(4, 0.1, true);
        assert_eq!(anim.frames, 4);
        assert_eq!(anim.current_frame, 0);
        assert!(!anim.finished);
    }

    #[test]
    fn test_animation_update() {
        let mut anim = Animation::new(3, 0.1, true);

        // Not enough time passed
        assert!(!anim.update(0.05));
        assert_eq!(anim.current_frame, 0);

        // Enough time passed
        assert!(anim.update(0.06));
        assert_eq!(anim.current_frame, 1);
    }

    #[test]
    fn test_animation_loop() {
        let mut anim = Animation::new(2, 0.1, true);

        anim.update(0.1);  // Frame 1
        anim.update(0.1);  // Frame 0 (loop)

        assert_eq!(anim.current_frame, 0);
        assert!(!anim.finished);
    }

    #[test]
    fn test_animation_no_loop() {
        let mut anim = Animation::new(2, 0.1, false);

        anim.update(0.1);  // Frame 1
        anim.update(0.1);  // Stay at frame 1, finished

        assert_eq!(anim.current_frame, 1);
        assert!(anim.finished);
    }

    #[test]
    fn test_next_frame() {
        let mut anim = Animation::two_frame();
        assert_eq!(anim.current_frame, 0);

        anim.next_frame();
        assert_eq!(anim.current_frame, 1);

        anim.next_frame();
        assert_eq!(anim.current_frame, 0);
    }

    #[test]
    fn test_reset() {
        let mut anim = Animation::new(3, 0.1, false);
        anim.current_frame = 2;
        anim.finished = true;

        anim.reset();

        assert_eq!(anim.current_frame, 0);
        assert!(!anim.finished);
    }
}
