#version 330

uniform mat4 modelView;
uniform mat4 projection;

in vec3 position;
in vec3 color;

out vec3 vColor;

void main() {
  gl_Position = projection * modelView * vec4(position, 1.0);
  
  vColor = color;
}
