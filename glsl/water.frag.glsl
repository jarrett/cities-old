#version 150

const float texScale      = 0.04;                   // Smaller numbers make the texture appear larger.
const vec3  baseColor     = vec3(0.05, 0.10, 0.12); // Base color of water.
const float maxOpacity    = 0.8;                    // Max opacity of water.
const float opacitySlope  = 0.7;                    // How quickly water turns opaque as depth increases.
const float minFoam       = 0.2;                    // Minimum amount of foam in the mix.
const float maxFoam       = 0.5;                    // Maximum amount of foam in the mix.
const float foamThreshold = 2.5;                    // Depth at which foam hits minimum value.

in vec3 vPosition;
in float vDepth;

uniform sampler2D foam;

out vec4 outColor;

vec3 foamColor() {
  return vec3(texture(foam, vec2(vPosition) * texScale));
}

vec3 waterColor() {
  return mix(    
    baseColor,
    foamColor(),
    minFoam + (
      (maxFoam - minFoam) *
      smoothstep(0, foamThreshold, foamThreshold - vDepth)
    )
  );
}

void main() {
  float opacity = clamp(vDepth * opacitySlope, 0, maxOpacity);
  outColor = vec4(waterColor(), opacity);
}