#version 330 core

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;
in vec4 FragPosLightSpace;

out vec4 FragColor;

uniform vec3 lightPos;
uniform vec3 viewPos;
uniform vec3 baseColor;
uniform float metallic;
uniform float roughness;
uniform sampler2DShadow shadowMap;
uniform sampler2D baseColorTexture;
uniform int useBaseColorTexture;

float ShadowCalculation(vec4 fragPosLightSpace, vec3 normal, vec3 lightDir)
{
    vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;
    projCoords = projCoords * 0.5 + 0.5;

    if (projCoords.z > 1.0)
        return 0.0;

    //float bias = max(0.01 * (1.0 - dot(normal, lightDir)), 0.001);
    float shadow = 0.0;

    vec2 texelSize = 1.0 / textureSize(shadowMap, 0);

    float cnt = 0.0;
    for (int x = -1; x <= 1; ++x)
    {
        for (int y = -1; y <= 1; ++y)
        {
            cnt = cnt + 1.0;
            vec2 offset = vec2(x, y) * texelSize;
            shadow += texture(shadowMap, vec3(projCoords.xy + offset, projCoords.z));
        }
    }

    shadow /= cnt;
    return mix(0.3, 1.0, shadow); // soft shadow
}
void main()
{
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    vec3 viewDir  = normalize(viewPos - FragPos);
    vec3 halfwayDir = normalize(lightDir + viewDir);
    float diff = max(dot(norm, lightDir), 0.0);
    float spec = pow(max(dot(norm, halfwayDir), 0.0), 32.0 * (1.0 - roughness));

    vec3 baseColorFinal = baseColor;
    if (useBaseColorTexture == 1) {
        vec3 texColor = texture(baseColorTexture, TexCoords).rgb;
        baseColorFinal = baseColor * texColor;
    }

    vec3 diffuse = diff * baseColorFinal;
    vec3 specular = spec * mix(vec3(0.04), baseColorFinal, metallic);
    vec3 ambient = 0.3 * baseColorFinal;

    float shadow = ShadowCalculation(FragPosLightSpace, norm, lightDir);
    vec3 lighting = ambient + (diffuse + specular) * shadow;
    FragColor = vec4(lighting, 1.0);
    //FragColor = vec4(vec3(shadow), 1.0);
}