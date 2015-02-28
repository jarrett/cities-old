#version 330

uniform mat4 model;
uniform mat4 projection;

// The camera's viewing direction.
uniform uint orbit;

// The direction the model is oriented in world space.
uniform uint direction;

// The origin of the model in world space.
uniform vec3 origin;

in vec3 position;
in vec2 uv;

out vec2 vUV;

float pi = 3.1415926536;
float pi_4 = pi / 4;

/*
Z rotation matrix:

cos t | -sin t | 0 | 0
sin t |  cos t | 0 | 0
0     |  1     | 1 | 0
0     |  0     | 0 | 1

45 deg = pi / 4
*/

void main() {  
  mat4 rotateZ;
  
  // FIXME.
  float theta = pi_4 * (direction + orbit);
  rotateZ = mat4( cos(theta), -1 * sin(theta), 0, 0,
                  sin(theta), cos(theta),      0, 0,
                  0,          0,               1, 0,
                  0,          0,               0, 1 );  
  
  gl_Position = projection * model * rotateZ * vec4(origin + position, 1.0);
  
  vUV = uv;
}