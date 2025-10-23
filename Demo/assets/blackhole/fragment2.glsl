#version 430 core
out vec4 FragColor;
in vec2 TexCoords;

uniform sampler2D iChannel0;    // textura de fundal
uniform sampler2D iChannel1;    // textura de fundal
uniform vec2 iResolution;       // dimensiunea viewport-ului
uniform float iTime;            // timp pentru animatie

#define PI 3.1415926538

// wormhole settings
float a = .001;           // wormhole throat length
float M = .1;           // wormhole smoothness
float dt = 1.1;          // integration step
int maxSteps = 100;    // maximum steps

// camera settings
float camL = 5.;        // camera distance
float zoom = 1.5;       // camera zoom
float rotationSpeed = 1.5;

// wormhole function r(l)
float LtoR(float l){
    float x = max(0.,2.*(abs(l)-a)/PI/M);
    return 1.+M*(x*atan(x)-.5*log(1.+x*x));
}

// wormhole derivative
float LtoDR(float l){
    float x = max(0.,2.*(abs(l)-a)/(PI*M));
    return 2.*atan(x) * sign(l)/PI;
}

// generate image
void main(){
    vec2 fragCoord = TexCoords * iResolution.xy;
    
    // calculam unghiul de rotatie bazat pe timp
    float angle = iTime * rotationSpeed;
    
    // ray projection cu rotatie
    vec2 uv = (2.*fragCoord-iResolution.xy)/iResolution.x;
    
    // aplicam rotatie la coordonatele UV
    float cosAngle = cos(angle);
    float sinAngle = sin(angle);
    vec2 rotatedUV = vec2(
        uv.x * cosAngle - uv.y * sinAngle,
        uv.x * sinAngle + uv.y * cosAngle
    );
    
    vec3 vel = normalize(vec3(-zoom, rotatedUV));
    vec2 beta = normalize(vel.yz);
    
    // ray tracing
    float l = camL;
    float r = LtoR(camL);
    float dl = vel.x;
    float H = r*length(vel.yz);
    float phi = 0.;
    float dr;
    
    int steps = 0;
    while(abs(l) < max(abs(camL)*2.,a+2.) && steps<maxSteps){
        dr = LtoDR(l);
        r = LtoR(l);
        l += dl*dt;
        phi += H/r/r*dt;
        dl += H*H*dr/r/r/r*dt;
        steps++;
    }
    
    // aplicam rotatia DOAR la directia finala (nu la spatiul de fundal)
    phi -= angle;
    
    // sky direction (cu rotirea aplicata doar la geometria gaurii de vierme)
    float dx = dl*dr*cos(phi)-H/r*sin(phi);
    float dy = dl*dr*sin(phi)+H/r*cos(phi);
    vec3 vec = normalize(vec3(dx,dy*beta));
    vec3 cubeVec = vec3(-vec.x,vec.z,-vec.y);
    
    // set pixel color
    vec2 center = iResolution.xy*0.5;
    if(distance(fragCoord, center) >= iResolution.x*0.5) discard;

    if(l > 0.){
        FragColor = texture(iChannel0, cubeVec.xy);
    } else{
        FragColor = texture(iChannel1, cubeVec.xy);
    }
}