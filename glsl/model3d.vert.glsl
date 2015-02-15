#version 330

uniform mat4 model;
uniform mat4 projection;
uniform uint direction;
uniform vec3 origin;

in vec3 position;
in vec3 uv;

out vec3 vUV;

void main() {  
  mat4 rotateZ;
  
  if (mod(direction, 2u) == 1u) {    
    /* 45 degrees. */
    rotateZ = mat4( 0.7071067812, -0.7071067812, 0, 0,
                    0.7071067812,  0.7071067812, 0, 0,
                    0,             0,            1, 0,
                    0,             0,            0, 1 );
  } else {
    rotateZ = mat4( 1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    0, 0, 0, 1 );
  }
  
  //gl_Position = projection * model * rotateZ * vec4(origin + position, 1.0);
  gl_Position = projection * model * vec4(origin + position, 1.0);
  
  vUV = uv;
}