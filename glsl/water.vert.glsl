#version 330

uniform mat4 model;
uniform mat4 projection;

in vec3 position;
in float depth;

out vec3 vPosition;
out float vDepth;

void main() {
  gl_Position = projection * model * vec4(position, 1.0);
  
  vPosition = position;
  vDepth = depth;
}