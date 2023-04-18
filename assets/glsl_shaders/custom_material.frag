#version 450
layout(location = 0) in vec2 v_Uv;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec3 v_Position;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
    mat4 View;
    mat4 InverseView;
    mat4 Projection;
    vec3 WorldPosition;
    float width;
    float height;
};

layout(set = 2, binding = 0) uniform Mesh {
    mat4 Model;
    mat4 InverseTransposeModel;
    uint flags;
};

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform CustomMaterial {
    vec4 Color;
    
};

layout(set = 1, binding = 1) uniform vec3 lightPositon;

void main() {
    float threshold = 3.0;
    float intensity = 1.0;

    vec3 worldPosition = (Model * vec4(v_Position, 1.0)).xyz;
    vec3 worldNormal = normalize(vec3(Model * vec4(v_Normal, 0.0)));
    vec3 lightVector= normalize(lightPositon - WorldPosition);

    float brightness = dot(worldNormal, lightVector);
    


    float step_val = 10.0 / intensity;
    float normalized_br = ((brightness + 1.0) / 2.0) * intensity;

    float final_br = ceil(normalized_br + threshold) / intensity;


    o_Target = Color;
}
