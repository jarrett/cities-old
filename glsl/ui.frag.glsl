#version 330

uniform sampler2D sprite;

in vec2 vUV;

out vec4 outColor;

void main() {
  outColor = texture(sprite, vUV);
  //outColor = vec4(1.0, 0.0, 0.0, 1.0);
}