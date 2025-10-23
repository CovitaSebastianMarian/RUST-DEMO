#version 430 core
in vec2 TexCoords;

uniform sampler2D iChannel0;    // textura de fundal
uniform sampler2D iChannel1;    // textura de fundal
uniform vec2 iResolution;             // dimensiunea viewport-ului
uniform float iTime;            // timp pentru animatie

// Define constants.
const float distortionStrength = 1.0;
const float maxLensingAngle = 28.3;
const float blackRadius = 0.15;
const float edgeBrightnessFactor = 2.04;

// Define colors.
const vec3 brightColor1 = vec3(0.95, 0.87, 0.75);
const vec3 brightColor2 = vec3(0.966, 0.91, 0.84);
const vec3 accretionDiskFadeColor = vec3(1.0, 0.67, 0.2);

// Define dynamic globals.
vec2 sourcePosition;
vec2 aspectRatioCorrectionFactor;

// Utility from HLSL. Clamps an input to a 0-1 range.
float saturate(float x)
{
    return clamp(x, 0.0, 1.0);
}

// Inverse of the lerp/mix function. Useful for getting back the 0-1 interpolant from a given input range.
// The resulting interpolant cannot extend beyond the 0-1 range, even if the inputs are outside of their expected provided bounds.
float inverseLerp(float from, float to, float x)
{
    return saturate((x - from) / (to - from));
}

vec2 vectorNoise(vec2 coords)
{
    // Sample from the noise texture and interpret the results as an angle.
    float angle = texture(iChannel0, coords).x * 16.03;
    
    // Convert the aforementioned angle into a 0-1 range vector.
    return vec2(cos(angle), sin(angle)) * 0.5 + 0.5;
}

// Rotates a vector by a given angle.
// This is based on the Z rotation matrix, but simplified.
vec2 rotatedBy(vec2 v, float theta)
{
    float s = sin(theta);
    float c = cos(theta);
    return vec2(v.x * c - v.y * s, v.x * s + v.y * c);
}

float calculateGravitationalLensingAngle(vec2 uv)
{
    // Calculate how far the given pixel is from the source of the distortion. This autocorrects for the aspect ratio resulting in
    // non-square calculations.
    float distanceToSource = max(distance((uv - 0.5) * aspectRatioCorrectionFactor + 0.5, sourcePosition), 0.0);
    
    // Calculate the lensing angle based on the aforementioned distance. This uses distance-based exponential decay to ensure that the effect
    // does not extend far past the source itself.
    return distortionStrength * maxLensingAngle * exp(-distanceToSource / blackRadius * 2.0);
}

vec4 applyColorEffects(vec4 color, float gravitationalLensingAngle, vec2 uv, vec2 distortedUV)
{
    // Calculate offset values based on noise.
    vec2 uvOffset1 = vectorNoise(distortedUV + vec2(0, iTime * 0.8));
    vec2 uvOffset2 = vectorNoise(distortedUV * 0.4 + vec2(0, iTime * 0.7));
    
    // Calculate color interpolants. These are used below.
    // The black hole uses a little bit of the UV offset noise for calculating the edge boundaries. This helps make the effect feel a bit less
    // mathematically perfect and more aesthetically interesting.
    float offsetDistanceToSource = max(distance((uv - 0.5) * aspectRatioCorrectionFactor + 0.5, sourcePosition + uvOffset1 * 0.004), 0.0);
    float blackInterpolant = inverseLerp(blackRadius, blackRadius * 0.85, offsetDistanceToSource);
    float brightInterpolant = pow(inverseLerp(blackRadius * (1.01 + uvOffset2.x * 0.1), blackRadius * 0.97, offsetDistanceToSource), 1.6) * 0.6 + gravitationalLensingAngle * 7.777 / maxLensingAngle;
    float accretionDiskInterpolant = inverseLerp(blackRadius * 1.93, blackRadius * 1.3, offsetDistanceToSource) * (1.0 - brightInterpolant);
    
    // Calculate the inner bright color. This is the color used right at the edge of the black hole itself, where everything is burning due to extraordinary amounts of particle friction.
    vec4 brightColor = vec4(mix(brightColor1, brightColor2, uvOffset1.y), 1) * edgeBrightnessFactor;
    
    // Interpolate towards the bright color first.
    color = mix(color, brightColor, saturate(brightInterpolant) * distortionStrength);
    
    // Interpolate towards the accretion disk's color next. This is what is drawn a bit beyond the burning bright edge. It is still heated, but not as much, and as such is closer to an orange
    // glow than a blazing yellowish white.
    color = mix(color, vec4(accretionDiskFadeColor, 1), accretionDiskInterpolant * distortionStrength);
    
    // Lastly, place the black hole in the center above everything.
    color = mix(color, vec4(0, 0, 0, 1), blackInterpolant * distortionStrength);
    
    return color;
}
out vec4 fragColor;
void main()
{
    vec2 fragCoord = TexCoords * iResolution.xy;
    // Normalized pixel coordinates (from 0 to 1).
    vec2 uv = fragCoord/iResolution.xy;
    
    // Calculate dynamic global values.
    aspectRatioCorrectionFactor = vec2(iResolution.x / iResolution.y, 1.0);
    sourcePosition = vec2(0.5, cos(iTime * 4.3) * 0.2 + 0.5);
    sourcePosition = vec2(0.5, 0.5);
    // Calculate the gravitational lensing angle and the coordinates that result from following its rotation.
    // This roughly follows the mathematics of relativistic gravitational lensing in the real world, albeit with a substitution for the impact parameter:
    // https://en.wikipedia.org/wiki/Gravitational_lensing_formalism
    float gravitationalLensingAngle = calculateGravitationalLensingAngle(uv);
    vec2 distortedUV = rotatedBy(uv - 0.5, gravitationalLensingAngle) + 0.5;
    
    // Calculate the colors based on the above information, and supply them to the output color.
    fragColor = applyColorEffects(texture(iChannel1, distortedUV), gravitationalLensingAngle, uv, distortedUV);
}
