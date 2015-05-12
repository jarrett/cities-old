#version 330

uniform mat4 camera;

in vec3 position;
in vec3 color;

out vec3 vColor;

void main() {
  gl_Position = camera * vec4(position, 1.0);
  
  vColor = color;
}
