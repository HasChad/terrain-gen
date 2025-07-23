#version 100
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 normal;

varying vec2 uv;
varying vec4 norm;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
    norm = normal;
}