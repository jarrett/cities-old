#version 330

uniform mat4 model;
uniform mat4 projection;
uniform float xSize;
uniform float ySize;
uniform float zSize;
uniform uint direction;

in vec3 position;
in vec3 uv;

out vec3 vUV;

void main() {
  vec3 scaledPosition = vec3(0, 0, zSize * position.z);
  if (direction == 0u || direction == 1u || direction == 4u || direction == 5u) {
    scaledPosition.x = xSize * position.x / 2;
    scaledPosition.y = ySize * position.y / 2;
  } else if (direction == 2u || direction == 3u || direction == 6u || direction == 7u) {
    scaledPosition.x = ySize * position.x / 2;
    scaledPosition.y = xSize * position.y / 2;
  }
  
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
  
  gl_Position = projection * model * rotateZ * vec4(scaledPosition, 1.0);
  
  vUV = uv;
}