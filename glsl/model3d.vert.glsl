#version 330

uniform mat4 modelView;
uniform mat4 projection;

// The camera's viewing direction.
uniform int orbit;

// The direction the model is oriented in world space.
uniform int direction;

// The origin of the model in world space.
uniform vec3 origin;

// The position of the vertex relative to the model's origin.
in vec3 position;

// The UV coordinates of the vertex in sprite-sheet space.
in vec2 uv;

out vec2 vUV;

float pi = 3.1415926536;
float pi_4 = pi / 4;

/*
Z rotation matrix:

cos t | -sin t | 0
sin t |  cos t | 0
0     |  1     | 1

45 deg = pi / 4
*/

void main() {  
  /* Each step in the model's direction is 45 degrees. Each step in the camera's
  orbit is 90 degrees. We have to ensure the model is always facing the camera. */
  float theta = pi_4 * ((orbit * 2) - (direction % 2));
  direction;
  
  mat4 rotateZ = mat4( cos(theta), -1 * sin(theta), 0, 0,
                       sin(theta), cos(theta),      0, 0,
                       0,          0,               1, 0,
                       0,          0,               0, 1);
  gl_Position = projection * modelView * ((rotateZ * vec4(position, 1.0)) + vec4(origin, 1.0));
  
  vUV = uv;
}