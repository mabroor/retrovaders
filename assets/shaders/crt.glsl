// CRT shader for RetroVaders
// Implements scanlines, bloom, barrel distortion, and color zones

#ifdef VERTEX

attribute vec3 position;
attribute vec2 texcoord;

varying lowp vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}

#endif

#ifdef FRAGMENT

precision mediump float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 resolution;
uniform float scanline_weight;
uniform float bloom_strength;
uniform float curvature_amount;

// Barrel distortion for CRT curvature
vec2 barrel_distort(vec2 coord) {
    vec2 cc = coord - vec2(0.5);
    float dist = dot(cc, cc) * curvature_amount;
    return coord + cc * dist;
}

// Apply color zone tint based on Y position
vec3 apply_color_zone(vec3 color, float y) {
    // UFO zone (top) - red tint
    if (y < 0.125) {
        return color * vec3(1.2, 0.8, 0.8);
    }
    // Play area - slight blue phosphor tint
    else if (y < 0.72) {
        return color * vec3(0.95, 0.95, 1.05);
    }
    // Shield zone - green tint
    else if (y < 0.94) {
        return color * vec3(0.8, 1.2, 0.8);
    }
    // HUD zone - white (no tint)
    return color;
}

void main() {
    // Apply barrel distortion
    vec2 distorted_uv = barrel_distort(uv);

    // Check bounds after distortion
    if (distorted_uv.x < 0.0 || distorted_uv.x > 1.0 ||
        distorted_uv.y < 0.0 || distorted_uv.y > 1.0) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }

    // Sample texture
    vec4 color = texture2D(Texture, distorted_uv);

    // Apply scanlines
    float scanline = sin(distorted_uv.y * resolution.y * 3.14159) * 0.5 + 0.5;
    color.rgb *= mix(1.0, scanline, scanline_weight);

    // Simple bloom (brighten based on luminance)
    float luma = dot(color.rgb, vec3(0.299, 0.587, 0.114));
    if (luma > 0.4) {
        color.rgb += color.rgb * bloom_strength * (luma - 0.4);
    }

    // Apply color zones
    color.rgb = apply_color_zone(color.rgb, distorted_uv.y);

    // Vignette effect (darken corners)
    vec2 vignette_uv = distorted_uv * (1.0 - distorted_uv);
    float vignette = vignette_uv.x * vignette_uv.y * 15.0;
    vignette = pow(vignette, 0.25);
    color.rgb *= vignette;

    gl_FragColor = color;
}

#endif
