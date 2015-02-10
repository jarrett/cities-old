#version 330

// Configure various constants.
const float texScale          = 0.05;   // Smaller numbers make the texture appear larger.
const float waterHeight       = 5;      // Below this Z coord, we use the underwater texture.
const float waterTransition   = 2.5;    // Size of transition zone from underwater to land texture.
const float plainMinLevelness = 0.98;   // Threshold dividing plain from slope.
const float plainTransition   = 0.25;   // Size of transition zone from plain to slope.
const float slopeMinLevelness = 0.70;   // Threshold dividing slope from cliff.
const float slopeTransition   = 0.2;    // Size of transition zone from slope to cliff.
const vec3  sunDir            = normalize(vec3(-0.3, -0.3, 1.0));
const vec3  sunColor          = vec3(1.0, 0.96, 0.87);
const float sunBrightness     = 0.8;
const vec3  ambientColor      = vec3(0.8, 0.8, 0.85);
const float ambientBrightness = 0.1;

uniform sampler2D underwater; // Texture for the seafloor and shores.
uniform sampler2D plain;      // Texture for plain land.
uniform sampler2D slope;      // Texture for sloping land. (But not as sloped as cliffs.)
uniform sampler2D cliff;      // Texture for the most extreme slopes.

in vec3 vPosition;
in vec3 vNormal;

out vec4 outColor;

vec3 planarMap(sampler2D tex, float scale, vec3 position, vec3 normal) {
  /* First, check whether we're closely aligned with any of the three axes. If we are,
  then we don't want to waste GPU cycles sampling from all three maps and mixing them. */
  if (normal.z > 0.85) {
    //return vec3(0,0,1); // Uncomment to debug with blue z.
    return vec3(texture(tex, vec2(vPosition.x * scale, vPosition.y * scale)));
  } else if (normal.y > 0.75) {
    //return vec3(0,1,0); // Uncomment to debug with green y.
    return vec3(texture(tex, vec2(vPosition.x * scale, vPosition.z * scale)));
  } else if (normal.x > 0.75) {
    //return vec3(1,0,0); // Uncomment to debug with red x.
    return vec3(texture(tex, vec2(vPosition.y * scale, vPosition.z * scale)));
  } else {
    /* We're not closely aligned with any of the three axes. So we need to blend. First,
    we obtain the color for each of the three axis-aligned planar maps. */
    vec3 xCol = vec3(texture(tex, vec2(vPosition.y * scale, vPosition.z * scale)));
    //xCol      = vec3(1,0,0); // Uncomment to debug with red x.
    vec3 yCol = vec3(texture(tex, vec2(vPosition.x * scale, vPosition.z * scale)));
    //yCol      = vec3(0,1,0); // Uncomment to debug with green y.
    vec3 zCol = vec3(texture(tex, vec2(vPosition.x * scale, vPosition.y * scale)));
    //zCol      = vec3(0,0,1); // Uncomment to debug with blue z.
    
    /* Now we blend between the three. The order of operations is mix(mix(x, y), z), which
    means the x and y samples are weighted the same as each other, but not the same as
    the z sample. That's fine, because z is the "special" axis. */
    return mix(
      mix(
        xCol, yCol,
        smoothstep(-0.05, 0.05, abs(normal.y) - abs(normal.x))
      ),
      zCol,
      smoothstep(0.8, 0.85, normal.z)
    );
  }
}

vec3 underwaterColor() {
  return planarMap(underwater, texScale, vPosition, vNormal);
}

vec3 plainColor() {
  return planarMap(plain, texScale, vPosition, vNormal);
}

vec3 slopeColor() {
  return planarMap(slope, texScale, vPosition, vNormal);
}

vec3 cliffColor() {
  return planarMap(cliff, texScale, vPosition, vNormal);
}

vec3 landColor() {
  float levelness = vNormal.z; // Just an alias to make later code more readable.
  
  if (levelness >= plainMinLevelness) {
    return plainColor();
  } else if (levelness >= plainMinLevelness - plainTransition) {
    return mix(
      plainColor(),
      slopeColor(),
      smoothstep(plainMinLevelness, plainMinLevelness - plainTransition, levelness)
    );
  } else if (levelness >= slopeMinLevelness) {
    return slopeColor();
  } else if (levelness >= slopeMinLevelness - slopeTransition) {
    return mix(
      slopeColor(),
      cliffColor(),
      smoothstep(slopeMinLevelness, slopeMinLevelness - slopeTransition, levelness)
    );
  } else {
    return cliffColor();
  }
}

// Uses Lambert reflectance for the diffuse component. Clamps all three channels to [1,1].
vec3 ambientDiffuse() {
  vec3 lightColor = (sunColor * sunBrightness * dot(vNormal, sunDir)) + 
                    (ambientColor * ambientBrightness);
  return clamp(lightColor, vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0));
}

void main() {
  vec3 texColor = vec3(0, 0, 0); // We'll write to this variable shortly.
  
  if (vPosition.z <= waterHeight) {
    // Use nothing but the underwater texture.
    texColor = underwaterColor();
  } else if (vPosition.z <= waterHeight + waterTransition) {
    // Transition zone. Blend underwater and land texture.
    texColor = mix(
      underwaterColor(),
      landColor(),
      smoothstep(waterHeight, waterHeight + waterTransition, vPosition.z)
    );
  } else {
    // Use nothing but the land texture.
    texColor = landColor();
  }
  
  outColor = vec4(ambientDiffuse() * texColor, 1.0);
}