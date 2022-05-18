precision mediump float;

in vec2 vert;
in vec2 coord;

in vec4 color;

out vec4 result;

uniform sampler2D ourTexture;

void main() {
    result = texture(ourTexture,coord);
}