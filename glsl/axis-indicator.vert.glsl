#version 330

uniform mat4 model;
uniform mat4 projection;
uniform float scale;

in vec3 position;
in vec3 color;

out vec3 vColor;

void main() {
  gl_Position = projection * model * vec4(position * scale, 1.0);
  
  vColor = color;
}
