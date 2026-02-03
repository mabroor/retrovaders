// CRT shader effects - scanlines, bloom, curvature, color zones

use macroquad::prelude::*;
use crate::game::config::*;

/// CRT effect settings
#[derive(Debug, Clone)]
pub struct CrtSettings {
    pub enabled: bool,
    pub scanline_weight: f32,
    pub bloom_strength: f32,
    pub curvature_amount: f32,
}

impl Default for CrtSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            scanline_weight: 0.15,
            bloom_strength: 0.3,
            curvature_amount: 0.02,
        }
    }
}

impl CrtSettings {
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}

/// Color zone definition
#[derive(Debug, Clone, Copy)]
pub struct ColorZone {
    pub y_start: f32,
    pub y_end: f32,
    pub color: Color,
}

/// Get the color zones matching original arcade gel overlays
pub fn get_color_zones() -> [ColorZone; 4] {
    [
        // UFO zone - red
        ColorZone {
            y_start: ZONE_UFO_Y_START,
            y_end: ZONE_UFO_Y_END,
            color: Color::new(1.0, 0.0, 0.0, 0.3),
        },
        // Play area - white with blue phosphor tint
        ColorZone {
            y_start: ZONE_PLAY_Y_START,
            y_end: ZONE_PLAY_Y_END,
            color: Color::new(0.78, 0.78, 1.0, 0.1),
        },
        // Shield zone - green
        ColorZone {
            y_start: ZONE_SHIELD_Y_START,
            y_end: ZONE_SHIELD_Y_END,
            color: Color::new(0.0, 1.0, 0.0, 0.2),
        },
        // HUD zone - white
        ColorZone {
            y_start: ZONE_HUD_Y_START,
            y_end: ZONE_HUD_Y_END,
            color: Color::new(1.0, 1.0, 1.0, 0.1),
        },
    ]
}

/// Apply color zone tinting (simple version without shader)
pub fn apply_color_zones() {
    let scale = SCALE as f32;
    let zones = get_color_zones();

    for zone in &zones {
        draw_rectangle(
            0.0,
            zone.y_start * scale,
            WINDOW_WIDTH as f32,
            (zone.y_end - zone.y_start) * scale,
            zone.color,
        );
    }
}

/// Draw simple scanline effect (without shader)
pub fn draw_scanlines(settings: &CrtSettings) {
    if !settings.enabled {
        return;
    }

    let scale = SCALE as f32;
    let scanline_color = Color::new(0.0, 0.0, 0.0, settings.scanline_weight);

    // Draw horizontal lines every 2 scaled pixels
    let step = 2.0 * scale;
    let mut y = 0.0;

    while y < WINDOW_HEIGHT as f32 {
        draw_line(0.0, y, WINDOW_WIDTH as f32, y, 1.0, scanline_color);
        y += step;
    }
}

/// Draw vignette effect (darken corners)
pub fn draw_vignette() {
    let w = WINDOW_WIDTH as f32;
    let h = WINDOW_HEIGHT as f32;

    // Draw gradient rectangles at corners
    let corner_size = 100.0;
    let alpha = 0.3;

    // Top-left
    for i in 0..10 {
        let size = corner_size * (10 - i) as f32 / 10.0;
        let a = alpha * i as f32 / 10.0;
        draw_rectangle(0.0, 0.0, size, size, Color::new(0.0, 0.0, 0.0, a));
    }

    // Top-right
    for i in 0..10 {
        let size = corner_size * (10 - i) as f32 / 10.0;
        let a = alpha * i as f32 / 10.0;
        draw_rectangle(w - size, 0.0, size, size, Color::new(0.0, 0.0, 0.0, a));
    }

    // Bottom-left
    for i in 0..10 {
        let size = corner_size * (10 - i) as f32 / 10.0;
        let a = alpha * i as f32 / 10.0;
        draw_rectangle(0.0, h - size, size, size, Color::new(0.0, 0.0, 0.0, a));
    }

    // Bottom-right
    for i in 0..10 {
        let size = corner_size * (10 - i) as f32 / 10.0;
        let a = alpha * i as f32 / 10.0;
        draw_rectangle(w - size, h - size, size, size, Color::new(0.0, 0.0, 0.0, a));
    }
}

/// CRT post-processing manager (for future shader-based implementation)
pub struct CrtPostProcessor {
    pub settings: CrtSettings,
    render_target: Option<RenderTarget>,
    material: Option<Material>,
}

impl Default for CrtPostProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl CrtPostProcessor {
    pub fn new() -> Self {
        Self {
            settings: CrtSettings::default(),
            render_target: None,
            material: None,
        }
    }

    /// Initialize render target and shader
    pub fn init(&mut self) {
        // Create render target at game resolution
        self.render_target = Some(render_target(GAME_WIDTH, GAME_HEIGHT));

        // Try to load CRT shader material
        // Note: In Macroquad, custom shaders require more setup
        // For now, we use the simple software-based effects
        self.material = None;
    }

    /// Begin rendering to the off-screen target
    pub fn begin_render(&self) {
        if let Some(rt) = &self.render_target {
            set_camera(&Camera2D {
                zoom: vec2(2.0 / GAME_WIDTH as f32, 2.0 / GAME_HEIGHT as f32),
                target: vec2(GAME_WIDTH as f32 / 2.0, GAME_HEIGHT as f32 / 2.0),
                render_target: Some(rt.clone()),
                ..Default::default()
            });
        }
    }

    /// End rendering and draw to screen with CRT effects
    pub fn end_render(&self) {
        set_default_camera();

        if let Some(rt) = &self.render_target {
            // Draw the render target scaled up
            let params = DrawTextureParams {
                dest_size: Some(vec2(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)),
                flip_y: true,
                ..Default::default()
            };

            draw_texture_ex(&rt.texture, 0.0, 0.0, WHITE, params);

            // Apply software-based CRT effects
            if self.settings.enabled {
                apply_color_zones();
                draw_scanlines(&self.settings);
                draw_vignette();
            }
        }
    }

    /// Toggle CRT effects
    pub fn toggle(&mut self) {
        self.settings.toggle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = CrtSettings::default();
        assert!(settings.enabled);
        assert!(settings.scanline_weight > 0.0);
        assert!(settings.bloom_strength > 0.0);
    }

    #[test]
    fn test_toggle() {
        let mut settings = CrtSettings::default();
        assert!(settings.enabled);

        settings.toggle();
        assert!(!settings.enabled);

        settings.toggle();
        assert!(settings.enabled);
    }

    #[test]
    fn test_color_zones() {
        let zones = get_color_zones();
        assert_eq!(zones.len(), 4);

        // Verify zones cover full screen height
        assert_eq!(zones[0].y_start, 0.0);
        assert_eq!(zones[3].y_end, GAME_HEIGHT as f32);

        // Verify zones are contiguous
        assert_eq!(zones[0].y_end, zones[1].y_start);
        assert_eq!(zones[1].y_end, zones[2].y_start);
        assert_eq!(zones[2].y_end, zones[3].y_start);
    }

    #[test]
    fn test_post_processor_new() {
        let pp = CrtPostProcessor::new();
        assert!(pp.settings.enabled);
        assert!(pp.render_target.is_none());
    }
}
