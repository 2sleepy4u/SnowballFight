#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

struct CustomMaterial {
    color: vec4<f32>,
    light: vec4<f32>,
}

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;


@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var light_position = material.light;
    var light_dir = normalize(light_position.xyz - in.world_position.xyz);
    var brightness = max(dot(in.world_normal, light_dir), 0.1);

    var intensity = 0.1;
    var threshold = 3.0;

    var normalized_br = ((brightness + 1.0) / 2.0) * intensity;
    var final_br = (ceil(normalized_br + threshold) / intensity);
    
    if(brightness < 0.25) {
	brightness = 0.25;
    }
    else if(brightness < 0.5){
        brightness = 0.5;
    }
    else {
	brightness = 1.0;
    }

    return vec4(material.color.xyz * brightness, 1.0);
}
