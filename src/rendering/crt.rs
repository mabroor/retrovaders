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

/// CRT shader source (for future use with render targets)
pub const CRT_SHADER_VERTEX: &str = r#"
#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying lowp vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
"#;

pub const CRT_SHADER_FRAGMENT: &str = r#"
#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform float scanline_weight;
uniform float bloom_strength;
uniform vec2 resolution;

void main() {
    vec4 color = texture2D(Texture, uv);

    // Scanlines (every other line darker)
    float scanline = sin(uv.y * resolution.y * 3.14159) * 0.5 + 0.5;
    color.rgb *= mix(1.0, scanline, scanline_weight);

    // Simple bloom (brighten based on luminance)
    float luma = dot(color.rgb, vec3(0.299, 0.587, 0.114));
    if (luma > 0.4) {
        color.rgb += color.rgb * bloom_strength * (luma - 0.4);
    }

    gl_FragColor = color;
}
"#;

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
}
