#version 330

uniform mat4 model;
uniform mat4 projection;

in vec3 position;
in vec3 normal;

out vec3 vNormal;
out vec3 vPosition;

void main() {
  gl_Position = projection * model * vec4(position, 1.0);
  
  vPosition = position;
  vNormal = normal;
}