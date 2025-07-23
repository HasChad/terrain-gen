#version 100
precision lowp float;

varying vec2 uv;
varying vec4 norm;

uniform sampler2D Texture;

// Light direction in world/view space
const vec3 lightDir = normalize(vec3(1.0, 1.0, 1.0)); // Light coming from front

void main() {
    vec3 normal = normalize(norm.xyz);

    // Simple diffuse lighting
    float lightIntensity = max(dot(normal, lightDir), 0.0);

    // Sample the base texture
    vec4 texColor = texture2D(Texture, uv);

    // Modulate brightness with light intensity
    gl_FragColor = texColor * lightIntensity;
}