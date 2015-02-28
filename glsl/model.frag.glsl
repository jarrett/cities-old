#version 330

uniform sampler2D sprite;

in vec2 vUV;

out vec4 outColor;

void main() {
  vec4 texColor = texture(sprite, vUV);
  outColor = vec4(texColor.x, texColor.y, texColor.z, 1.0);
  //outColor = vec4(1.0, 1.0, 1.0, 1.0);
}