#version 330

uniform uvec2 viewportSize;

// The position of the vertex in pixels.
in vec2 position;

// The UV coordinates of the vertex in pixels.
in vec2 uv;

out vec2 vUV;

void main() {
  // viewportSize is width and height of screen in pixels. Position is in pixels, measured
  // from top-left of screen. 0, 0 in screen coordinates is the center of the screen.
  viewportSize.x;
  gl_Position = vec4(
     2 * position.x / viewportSize.x - 1,
    -2 * position.y / viewportSize.y + 1,
    -1.0, 1.0
  );
  //gl_Position = vec4(position.x - 0.5, position.y - 0.5, -1.0, 1.0);
  vUV = uv;
}