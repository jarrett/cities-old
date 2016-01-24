// The position of the vertex in pixels.
in vec2 position;

// The UV coordinates of the vertex in pixels.
in vec2 uv;

out vec2 vUV;

void main() {
  gl_Position = vec3(position, -1.0);
  vUV = uv;
}